use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, WidgetExt, Color};
use druid::widget::{Flex, Button, MainAxisAlignment, Slider, Label, WidgetId};
use druid::im::Vector;
use crate::circles::custom::CanvasData;

/*
Examples that may be useful:
    anim.rs: Has a circle which reacts to a click
    invalidation.rs: draws circles on a canvas-like widget
    panels.rs: Has a panel full of circles
    scroll.rs: To add additional scroll functionality
*/

/*
TODO:
    [x] Layout the widgets including custom canvas widget
    [x] Draw a circle in the canvas on a click
    [x] Make circles selectable
    [] add a slider to control radius of currently selected -> Lens
    [] add the slider to a context menu
    [] add a list of instructions to implement undo and redo functionality
    [] add escape key to set selection to None
    [] add scroll functionality
 */

const WINDOW_TITLE: &str = "Circles";
const WINDOW_SIZE: Size = Size::new(500., 500.);
const WINDOW_SIZE_MIN: Size = Size::new(250., 250.);
const PADDING: f64 = 8.;

pub fn main()-> Result<(), PlatformError>  {
    let data = AppData::new();
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .with_min_size(WINDOW_SIZE_MIN)
        .title(WINDOW_TITLE);
    AppLauncher::with_window(window)
        .launch(data)?;
    Ok(())
}

#[derive(Clone, Data, Lens)]
struct AppData{
    canvas: custom::CanvasData,
    radius: f64
}

impl AppData {
    fn new() -> Self {
        AppData {
            canvas: CanvasData::new(),
            radius: 0.5
        }
    }
}

fn build_ui() -> impl Widget<AppData> {

    //TEMP: Linking a widget to canvas data
    let index_label = Label::new(|d: &AppData, _: &_| {
        if let Some(i) = d.canvas.selected {
            format!("Current index: {}", i)
        } else {
            format!("Nothing Selected")
        }
    });

    let btn_undo = Button::new("Undo")
        .on_click(|_ctx, _data: &mut AppData, _env| {
            println!("UNDO!")
            });

    let btn_redo = Button::new("Redo")
        .on_click(|_ctx, _data: &mut AppData, _env| {
            println!("REDO!")
            });

    let header = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(index_label)
        .with_spacer(PADDING * 3.)
        .with_child(btn_undo)
        .with_spacer(PADDING * 2.)
        .with_child(btn_redo)
        // TEMP
        .with_child(Slider::new().lens(AppData::radius));

    //TODO: Link to the canvas in APPDATA
    // Will no doubt need to box this as I'm anticipating a recursion
    let canvas = custom::Canvas::new().lens(AppData::canvas);

    Flex::column()
        .with_child(header)
        .with_spacer(PADDING * 2.)
        .with_flex_child(canvas, 1.)
        .padding(PADDING * 2.)
}

mod custom {
    use super::*;
    use druid::{Point, MouseButton, Size, kurbo};
    use druid::widget::prelude::*;
    use druid::im::Vector;

    const RADIUS: f64 = 25.;
    const MAX_RADIUS: f64 = 50.;
    const MIN_RADIUS: f64 = 2.;

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

        fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &CanvasData, _data: &CanvasData, _env: &Env) {}

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

    // This holds the data for the canvas.
    // This is created in AppData. use a lens on the Canvas widget from Appdata
    // much like lensing a string to a label
    // TODO: CurrentItemLens struct and lens trait impl
    #[derive(Clone, Data, Lens)]
    pub struct CanvasData {
        pub circles: Vector<custom::Circle>,
        pub selected: Option<usize>,
    }

    impl CanvasData {
        pub fn new() -> Self {
            CanvasData {
                // the circles field is scoped to the Canvas widget
                circles: Vector::new(),
                selected: None,
            }
        }

        fn add_circle(&mut self, pos: Point) {
            let v_len = self.circles.len();
            self.circles.push_back(Circle::new(pos, v_len));
        }
    }

    // the fields herein should be deprecated
    pub struct Canvas;

    impl Canvas {
        pub fn new() -> Self {Canvas}
    }

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
                            data.selected = Some(index);
                        } else {
                            data.selected = None;
                            data.add_circle(e.pos);
                        }
                        ctx.request_paint()
                    },

                    MouseButton::Right => {
                        //TODO: open a context menu
                        println!("Current selection = {:?}", data.selected)
                    },
                    _ => ()
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
            _ctx: &mut UpdateCtx,
            _old: &CanvasData,
            _new: &CanvasData,
            _env: &Env,
        ) {}

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


//TEMP
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main().expect("Launch Error")
    }
}