//! # An app for converting Celsius to Farenheight and back, to show basic Lenses and controllers

use druid::{AppLauncher, WindowDesc, Widget, PlatformError,
            widget::{Label, TextBox, Flex, Align, Controller},
            Data, Lens, Size, WidgetExt, Event, EventCtx, Env
};


const WINDOW_TITLE: &str = "Temperature Converter";
const WINDOW_SIZE: Size = Size::new(350., 100.);

pub fn main() -> Result<(), PlatformError> {
    // model data
    let data = AppData::new();

    // create the window and ui
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .title(WINDOW_TITLE)
        .resizable(false);

    // link ui and data starts loop
    AppLauncher::with_window(window).launch(data)?;
    Ok(())
}

// define UI
fn build_ui() -> impl Widget<AppData> {

    // the only way to update the text is to update the model
    let tbox_f = TextBox::new()
        .lens(AppData::tbox_f)
        .controller(FController);

    let tbox_c = TextBox::new()
        .lens(AppData::tbox_c)
        .controller(CController);

    let layout = Flex::row()
        .with_child(tbox_c)
        .with_child(Label::new( "℃  =  "))
        .with_child(tbox_f)
        .with_child(Label::new( "℉ "));

    Align::centered(layout)
}


#[derive(Clone, Data, Lens)]
struct AppData {
    tbox_c: String,
    tbox_f: String
}

impl AppData {
    fn new() -> AppData {
        AppData {
            tbox_c: "0".into(),
            tbox_f: "32".into()
        }
    }
}

struct FController;

impl <W: Widget<AppData>> Controller<AppData, W> for FController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        // pass everything to the child widget
        // needs to occur before other items are updated beware this method in threaded apps
        child.event(ctx, event, data, env);

        // intercept the event
        if let Event::KeyDown(_) = event {
            f_to_c(data);
        }

        // Note: For multiple events use:

        // match event {
        //     Event::KeyDown(_) => {
        //         f_to_c(data);
        //     },
        //     _ => () // do nothing for other events
        // }
    }
}

struct CController;

impl <W: Widget<AppData>> Controller<AppData, W> for CController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        child.event(ctx, event, data, env);

        if let Event::KeyDown(_) = event {
            c_to_f(data);
        }
    }
}


//LOGIC
fn f_to_c(data: &mut AppData) {
    if let Ok(v) = data.tbox_f.parse::<f64>() {
        let c: f64 = (v - 32.) * (5. / 9.);
        data.tbox_c = format!("{:.1}", c)
    }
}

fn c_to_f(data: &mut AppData) {
    if let Ok(v) = data.tbox_c.parse::<f64>() {
        let f: f64 = v * ( 9. /  5.) + 32.;
        data.tbox_f = format!("{:.1}", f)
    }
}