//! # A circle drawing application

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, WidgetExt, Color,
            Selector, MenuDesc, MenuItem, LocalizedString, DelegateCtx, Target, Command, KeyCode};
use druid::widget::prelude::*;
use druid::widget::{Flex, Button, MainAxisAlignment, Slider, Label, Controller};
use druid::{ContextMenu, AppDelegate};

use crate::circles::custom::{CanvasData};

/*
TODO:
    [x] Layout the widgets including custom canvas widget
    [x] Draw a circle in the canvas on a click
    [x] Make circles selectable
    [X] add a slider to control radius of currently selected
    [X] add a context menu
    [X] add the slider to a context menu
    [ ] check that only one pop-up can occur at one time
    [ ] add a list of instructions to implement undo and redo functionality
    [?] add escape key to set selection to None -> not working
    [ ] add scroll functionality
 */

const WINDOW_TITLE: &str = "Circles";
const WINDOW_SIZE: Size = Size::new(500., 500.);
const WINDOW_SIZE_MIN: Size = Size::new(250., 250.);
const PADDING: f64 = 8.;
const POPUP_SIZE: Size = Size::new(250., 100.);

const MAX_RADIUS: f64 = 100.;
const MIN_RADIUS: f64 = 5.;

pub fn main()-> Result<(), PlatformError>  {
    let data = AppData::new();
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .with_min_size(WINDOW_SIZE_MIN)
        .title(WINDOW_TITLE);
    AppLauncher::with_window(window)
        .delegate(Delegate{})
        .launch(data)?;
    Ok(())
}

#[derive(Clone, Data, Lens)]
struct AppData{
    canvas: custom::CanvasData,
    radius: f64,
}

impl AppData {
    fn new() -> Self {
        AppData {
            canvas: CanvasData::new(),
            radius: (MAX_RADIUS + MIN_RADIUS) / 2.,
        }
    }
}

fn build_ui() -> impl Widget<AppData> {

    let btn_undo = Button::new("Undo")
        .on_click(|_ctx, _data: &mut AppData, _env| {
            println!("UNDO!");
            });

    let btn_redo = Button::new("Redo")
        .on_click(|_ctx, _data: &mut AppData, _env| {
            println!("REDO!");
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
        .padding(PADDING * 2.)
}

fn build_popup() -> impl Widget<AppData> {
    let lbl = Label::new(|data: &AppData, _: &_| {
        if let Some(i) = data.canvas.selected {
            format!("Radius for Circle {} = {:.1}", i, data.radius)
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


        // REVIEW: Is there a better event or method.
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
            _ if cmd.is(CVS_CTX_RESIZE) => {
                let popup = WindowDesc::new(build_popup)
                    .window_size(POPUP_SIZE)
                    .title("resize");
                ctx.new_window(popup);
                false
            },
            _ => true
        }
    }
}

/// ## Custom widgets implemented in this app
mod custom {
    use super::*;
    use druid::{Point, MouseButton, Size, kurbo};
    use druid::im::Vector;

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
    }

    impl CanvasData {
        pub fn new() -> Self {
            CanvasData {
                circles: Vector::new(),
                selected: None,
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
            if let Event::MouseDown(e) = event {
                // TEMP
                println!("{:?} pressed at {}", e.button , e.pos);

                match e.button {
                    MouseButton::Left => {
                        let mut nearest: Option<(usize, f64)> = None;
                        for c in & data.circles {
                            let distance = (c.pos - e.pos).hypot();
                            if distance < c.radius {
                                println!( "ITEM SELECTED = {}", c.index );
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
                            ,e.pos
                        );
                        ctx.show_context_menu(menu);
                    },
                    _ => ()
                }
            }
            // FIXME
            if let Event::KeyDown(e) = event {
                println!("KEY DOWN! ");
                if let KeyCode::Escape = e.key_code { data.selected = None }

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

    // TODO
    // enum ActionHistory {
    //     Creation,
    //     Adjustment,
    // }
}


//TEMP
#[cfg(test)]
mod tests {
    use super::*;
    // #[ignore]
    #[test]
    fn test() {
        main().expect("Launch Error")
    }
}
