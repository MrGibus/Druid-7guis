mod counter;
mod temperature;
mod flights;
mod timer;
mod crud;

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Size, widget::{Button, Label, Flex, Align}, WidgetExt};

const WINDOW_TITLE: &str = "Menu";
const WINDOW_SIZE: Size = Size::new(250., 300.);

// TEMP uncomment which one to run
pub fn main() {
    // counter::main().expect("launch Failed");
    // temperature::main().expect("Launch Failed");
    // flights::main().expect("Launch Failed");
    // timer::main().expect("Launch Failed");
    crud::main().expect("Launch Failed");
}

// Todo: rename to main once multi-windows are implemented
fn actual_main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .resizable(false)
        .title(WINDOW_TITLE);
    AppLauncher::with_window(window)
        .launch(())?;
    Ok(())
}

// TODO: spawn relevant example from press of button -> Refer multiwin example, but don't modify mods
fn build_ui() -> impl Widget<()> {
    let layout = Flex::column()
        .with_child(Label::new("7Guis in Druid"))
        .with_flex_spacer(1.)
        .with_child(Button::new("Counter")
             .on_click(|_, _, _: &_| {
                 println!("bar");
             })
            .expand_width())
        .with_child(Button::new("Temperature")
             .on_click(|_, _, _: &_| {
                 println!("bar");
             })
            .expand_width())
        .with_child(Button::new("Flight Booker")
             .on_click(|_, _, _: &_| {
                 println!("bar");
             })
            .expand_width())
        .with_child(Button::new("Timer")
             .on_click(|_, _, _: &_| {
                 println!("bar");
             })
            .expand_width())
        .with_child(Button::new("CRUD")
             .on_click(|_, _, _: &_| {
                 println!("WIP");
             })
            .expand_width())
        .with_child(Button::new("Circle Drawer")
             .on_click(|_, _, _: &_| {
                 println!("WIP");
             })
            .expand_width())
        .with_child(Button::new("Cells")
             .on_click(|_, _, _: &_| {
                 println!("WIP");
             })
            .expand_width())
        .padding(20.);

    Align::centered(layout)
}