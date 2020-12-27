//! # A circle drawing application
//! This will be revisited at a later date as there's likely a cleaner solution.
//! The slider from the second window and the circle being drawn is not smooth.

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, WidgetExt, Color,
            Selector, MenuDesc, MenuItem, LocalizedString, DelegateCtx, Target, Command, KeyCode};
use druid::widget::prelude::*;
use druid::widget::{Flex, Button, MainAxisAlignment, Slider, Label, Controller};
use druid::{ContextMenu, AppDelegate, WindowId, Key};
use druid::im::Vector;

use crate::circles::custom::{CanvasData};

/*
TODO:
    [x] Layout the widgets including custom canvas widget
    [x] Draw a circle in the canvas on a click
    [x] Make circles selectable
    [X] add a slider to control radius of currently selected
    [X] add a context menu
    [X] add the slider to a context menu
    [ ] change slider to only affect the circle selected at time window is open, disable enable canvas
    [X] check that only one pop-up can occur at one time and close on lost focus
    [ ] add a list of instructions to implement undo and redo functionality
    [ ] temp functionality: redo adds an circle to the middle, undo removes it and prints current status
    [*] Lag issue between multiple windows
    [X] add escape key to set selection to None
    [ ] add scroll functionality
 */

/**
*Lag Issue
Tried to fix Lag issue via commands and target specific window/widget instead of lens
The lag issue could not be solved, merely linking two windows via one lens lags out the
other window (works both ways).
Discussed on Zulip it is not occuring on MacOS, could be a windows issue
 **/

const WINDOW_TITLE: &str = "Circles";
const WINDOW_SIZE: Size = Size::new(500., 500.);
const WINDOW_SIZE_MIN: Size = Size::new(250., 250.);
const PADDING: f64 = 8.;
const POPUP_SIZE: Size = Size::new(250., 100.);

const BTN_CLR_DISABLED: Key<Color> = Key::new("app.btn.clr.disabled");
const BTN_TXT_DISABLED: Key<Color> = Key::new("app.btn.txt.disabled");

const MAX_RADIUS: f64 = 100.;
const MIN_RADIUS: f64 = 5.;

pub fn main()-> Result<(), PlatformError>  {
    let data = AppData::new();
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .with_min_size(WINDOW_SIZE_MIN)
        .title(WINDOW_TITLE);
    AppLauncher::with_window(window)
        .configure_env(|env, _state| {
            env.set(BTN_TXT_DISABLED, Color::grey(0.7));
            env.set(BTN_CLR_DISABLED, Color::grey(0.5));
        })
        .delegate(Delegate)
        .launch(data)?;
    Ok(())
}

#[derive(Clone, Data, Lens)]
struct AppData{
    canvas: custom::CanvasData,
    radius: f64,
    undo_valid: bool,
    redo_valid: bool,
    window_count: u32,
    action_log: ActionLog,
}

impl AppData {
    fn new() -> Self {
        AppData {
            canvas: CanvasData::new(),
            radius: (MAX_RADIUS + MIN_RADIUS) / 2.,
            undo_valid: false,
            redo_valid: false,
            window_count: 0,
            action_log: ActionLog::default(),
        }
    }
}

fn build_ui() -> impl Widget<AppData> {
    let btn_undo = Button::new("Undo")
        .env_scope(|env,data: &AppData| {
            if data.undo_valid {
                env.set(druid::theme::BUTTON_DARK, env.get(druid::theme::BUTTON_DARK));
                env.set(druid::theme::BUTTON_LIGHT, env.get(druid::theme::BUTTON_LIGHT));
                env.set(druid::theme::BORDER_LIGHT, env.get(druid::theme::BORDER_LIGHT));
                env.set(druid::theme::LABEL_COLOR, env.get(druid::theme::LABEL_COLOR));
            } else {
                env.set(druid::theme::BUTTON_DARK, env.get(BTN_CLR_DISABLED));
                env.set(druid::theme::BUTTON_LIGHT, env.get(BTN_CLR_DISABLED));
                env.set(druid::theme::BORDER_LIGHT, env.get(druid::theme::BORDER_DARK));
                env.set(druid::theme::LABEL_COLOR, Color::grey(0.7));
            }
        })
        .on_click(|ctx, _data: &mut AppData, _env| {
            ctx.submit_command(druid::commands::UNDO, Target::Global);
            });

    let btn_redo = Button::new("Redo")
        .env_scope(|env,data: &AppData| {
            if data.redo_valid {
                env.set(druid::theme::BUTTON_DARK, env.get(druid::theme::BUTTON_DARK));
                env.set(druid::theme::BUTTON_LIGHT, env.get(druid::theme::BUTTON_LIGHT));
                env.set(druid::theme::BORDER_LIGHT, env.get(druid::theme::BORDER_LIGHT));
                env.set(druid::theme::LABEL_COLOR, env.get(druid::theme::LABEL_COLOR));
            } else {
                env.set(druid::theme::BUTTON_DARK, env.get(BTN_CLR_DISABLED));
                env.set(druid::theme::BUTTON_LIGHT, env.get(BTN_CLR_DISABLED));
                env.set(druid::theme::BORDER_LIGHT, env.get(druid::theme::BORDER_DARK));
                env.set(druid::theme::LABEL_COLOR, Color::grey(0.7));
            }
        })
        .on_click(|ctx, _data: &mut AppData, _env| {
            ctx.submit_command(druid::commands::REDO, Target::Global);
            });

    let header = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(btn_undo)
        .with_spacer(PADDING * 2.)
        .with_child(btn_redo);

    let canvas = custom::Canvas.lens(AppData::canvas);

    Flex::column()
        .with_child(header)
        .with_spacer(PADDING * 2.)
        .with_flex_child(canvas, 1.)
        .with_spacer(PADDING * 2.)
        .padding(PADDING * 2.)
}

fn build_popup() -> impl Widget<AppData> {

    let lbl = Label::new(|data: &AppData, _: &_| {
        if let Some(i) = data.canvas.selected {
            format!("Radius for Circle {} = {:.1}", i , data.radius)
        } else {
            "Nothing Selected".to_string()
        }
    });

    let slider = Slider::new()
        .with_range(MIN_RADIUS, MAX_RADIUS)
        .expand_width()
        .lens(AppData::radius)
        .controller(RadController);

    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_flex_child(lbl, 1.)
        .with_flex_child(slider, 1.)
        .padding(PADDING * 2.)
}

/// This controller is used to update the radius of the current circle
struct RadController;

impl <W: Widget<AppData>> Controller<AppData, W> for RadController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);

        if let Event::MouseMove(_) = event {
            data.canvas.update_radius(data.radius);
        }
    }
}

/// ## Context Menu

const CVS_CTX_RESIZE: Selector = Selector::new("ctx-menu-resize");
const CVS_CTX_DESELECT: Selector = Selector::new("ctx-menu-deselect");

/// Context Menu items
fn build_context<T:Data>() -> MenuDesc<T> {
    MenuDesc::empty()
        .append(MenuItem::new(
            LocalizedString::new("Deselect"),
            CVS_CTX_DESELECT,
        ))
        .append(MenuItem::new(
            LocalizedString::new("Resize"),
            CVS_CTX_RESIZE,
        ))
}

struct Delegate;

impl AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env
    ) -> bool {
        match cmd {
            _ if cmd.is(CVS_CTX_DESELECT) => {
                data.canvas.selected = None;
                false
            },
            _ if cmd.is(CVS_CTX_RESIZE) && data.canvas.selected.is_some() && data.window_count == 1 => {
                let popup = WindowDesc::new(build_popup)
                    .window_size(POPUP_SIZE)
                    .resizable(false)
                    .title("resize");
                ctx.new_window(popup);
                false
            },
            _ if cmd.is(druid::commands::UNDO) => {
                println!("UNDO CMD");
                false
            },
            _ if cmd.is(druid::commands::REDO) => {
                println!("REDO CMD");
                false
            },
            _ => true
        }
    }

    fn window_added(
        &mut self,
        _id: WindowId,
        data: &mut AppData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        data.window_count += 1;
        if data.window_count > 1 {
            data.canvas.enabled = false;
        }
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        data: &mut AppData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        data.window_count -= 1;
        if data.window_count == 1 {
            data.canvas.enabled = true;
        }
    }
}

/// # Action History
/// This manages the undo and redo functionality in the struct
/// position represents the current location in the list,
/// if position is None there is nothing to redo, undo from end
/// max_actions represents the maximum number of items to store before actions are removed
/// action list is the actual history list

#[derive(Clone, Data)]
pub struct ActionLog {
    position: usize,
    max_actions: usize,
    action_list: Vector<usize> // TODO: Change to Vector<ActionItem> once implemented
}

impl ActionLog {
    /// new creates a new history of specified length
    pub fn new(max: usize) -> Self {
        ActionLog {
            position: 0,
            /// Maxmimum number of actions the log can store
            max_actions: max,
            /// Actual list of actions
            action_list: Vector::new()
        }
    }

    /// default creates a new history with a default max length of 10
    pub fn default() -> Self {
        ActionLog::new(10)
    }
}

/// The action item stores what action occured and an optional value
#[derive(Clone, Data)]
struct ActionItem {
    action_type: ActionType,
    radius: Option<f64>,
    circle_id: usize,
}

/// implements the creation and adjustment methods
impl ActionItem {
    fn creation(circle_id: usize) -> Self {
        ActionItem {
            action_type: ActionType::Creation,
            radius: None,
            circle_id
        }
    }

    fn adjustment(circle_id: usize, radius: f64) -> Self {
        ActionItem {
            action_type: ActionType::Adjustment,
            radius: Some(radius),
            circle_id
        }
    }
}

/// The action type defines what actions can occur
#[derive(Clone, Data, PartialEq)]
enum ActionType {
    Creation,
    Adjustment,
}

/// ## Custom widgets implemented in this app
mod custom {
    use super::*;
    use druid::{Point, MouseButton, Size, kurbo};

    const RADIUS: f64 = 25.;

    /// Holds individual circle data, only implements Widget<CanvasData>
    #[derive(Clone, Data, Lens)]
    pub struct Circle {
        pos: Point,
        index: usize,
        radius: f64,
    }

    impl Circle {
        pub fn new(pos: Point, index: usize) -> Self {
            Circle {
                pos,
                index,
                radius: RADIUS,
            }
        }
    }

    impl Widget<CanvasData> for Circle {
        fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut CanvasData, _env: &Env) {}

        fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &CanvasData, _env: &Env) {}

        fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &CanvasData, _data: &CanvasData, _env: &Env) {
            ctx.request_paint()
        }

        fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &CanvasData, _env: &Env) -> Size {
            let dim = self.radius *2.;
            bc.constrain(Size::new(dim, dim))
        }

        fn paint(&mut self, ctx: &mut PaintCtx, data: &CanvasData, _env: &Env) {
            let shape = kurbo::Circle::new(self.pos, self.radius);
            ctx.stroke(shape, &Color::BLACK, 2.);
            if let Some(i) = data.selected {
                if i == self.index {
                    ctx.fill(shape, &Color::BLACK.with_alpha(0.3));
                }
            }
        }
    }



    /// This holds the data for the canvas.
    /// This is created in AppData. use a lens on the Canvas widget from Appdata
    /// much like lensing a string to a label
    #[derive(Clone, Data, Lens)]
    pub struct CanvasData {
        pub circles: Vector<custom::Circle>,
        pub selected: Option<usize>,
        pub enabled: bool,
    }

    impl CanvasData {
        pub fn new() -> Self {
            CanvasData {
                circles: Vector::new(),
                selected: None,
                enabled: true,
            }
        }

        fn add_circle(&mut self, pos: Point) {
            let v_len = self.circles.len();
            self.circles.push_back(Circle::new(pos, v_len));
        }

        /// This function will update the radius of the currently selected item if it exists.
        /// It works given an f64 input from 0 to 1 and references the max and min radius
        /// Use with a custom controller
        pub fn update_radius(&mut self, radius: f64) {
            if let Some(i) = self.selected {
                self.circles[i].radius = radius;
            }
        }

        pub fn update_specific(&mut self, radius: f64, index: usize) {
            self.circles[index].radius = radius
        }
    }

    /// The canvas widget requires a lens to CanvasData
    /// For example:
    /// ```no_run
    /// struct AppData{
    ///     canvas: custom::CanvasData,
    /// }
    ///
    /// let canvas = custom::Canvas.lens(AppData::canvas);
    /// ```
    pub struct Canvas;

    impl Widget<CanvasData> for Canvas {
        fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut CanvasData, _env: &Env) {

            // Must request focus to use keyboard widgets
            ctx.request_focus();

            // straightforward way to disable direct interaction
            if data.enabled {
                match event {
                    Event::MouseDown(e) => {
                        // println!("{:?} pressed at {}", e.button , e.pos);
                        // println!("Canvas is enabled: {}", data.enabled);

                        match e.button {
                            MouseButton::Left => {
                                let mut nearest: Option<(usize, f64)> = None;
                                for c in &data.circles {
                                    let distance = (c.pos - e.pos).hypot();
                                    if distance < c.radius {
                                        println!("ITEM SELECTED = {}", c.index);
                                        if nearest.is_none() || nearest.unwrap().1 > distance {
                                            nearest = Some((c.index, distance));
                                        }
                                    }
                                }

                                if let Some((index, _)) = nearest {
                                    // Deselect by clicking circle again
                                    if data.selected == Some(index) {
                                        data.selected = None;
                                    } else {
                                        data.selected = Some(index);
                                    }
                                } else {
                                    data.selected = None;
                                    data.add_circle(e.pos);
                                }
                                ctx.request_paint()
                            },

                            MouseButton::Right => {
                                let menu = ContextMenu::new(
                                    build_context::<AppData>()
                                    , e.pos
                                );
                                ctx.show_context_menu(menu);
                            },
                            _ => ()
                        }
                    },
                    // Deselection through escape
                    Event::KeyDown(e) => {
                        if e.key_code == KeyCode::Escape {
                            data.selected = None;
                        }
                    },
                    _ => (),
                }
            }
        }

        // no functionality
        fn lifecycle(
            &mut self,
            _ctx: &mut LifeCycleCtx,
            _ev: &LifeCycle,
            _data: &CanvasData,
            _env: &Env,
        ) {}

        // no functionality
        fn update(
            &mut self,
            ctx: &mut UpdateCtx,
            old: &CanvasData,
            new: &CanvasData,
            env: &Env,
        ) {
            for c in &new.circles {
                //Can this be done without cloning?
                c.clone().update(ctx, old, new, env)
            }
        }

        // sets boundaries
        fn layout(
            &mut self,
            _ctx: &mut LayoutCtx,
            bc: &BoxConstraints,
            _data: &CanvasData,
            _env: &Env,
        ) -> Size {
            bc.max()
        }

        // Paint the widget
        fn paint(&mut self, ctx: &mut PaintCtx, data: &CanvasData, env: &Env) {
            // paint goes here
            let rect = ctx.size().to_rect();
            ctx.clip(rect);
            ctx.fill(rect, &Color::grey(0.4));

            for c in &data.circles {
                //Can this be done without cloning?
                c.clone().paint(ctx, data, env)
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test() {
        main().expect("Launch Error")
    }
}
