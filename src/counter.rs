//! # A simple counting application

use druid::{
    AppLauncher, WindowDesc, Widget, PlatformError,
    widget::{Button, Label, Flex, Align},
    Data, Lens, Size
};

const WINDOW_TITLE: &str = "Counter";
const WINDOW_SIZE: Size = Size::new(200., 75.);

fn build_ui() -> impl Widget<AppData> {
    let layout = Flex::row()
        .with_child(Label::new(|data: &AppData, _: &_| {
            format!("{}", data.count)
        }))
        .with_spacer(25.)
        .with_child(Button::new("Count")
                             .on_click(|_, data: &mut AppData, _: &_| data.count += 1));

    Align::centered(layout)

}

pub fn main() -> Result<(), PlatformError> {
    let data = AppData::new();
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .title(WINDOW_TITLE)
        .resizable(false);
    AppLauncher::with_window(window).launch(data)?;
    Ok(())
}

#[derive(Clone, Data, Lens)]
struct AppData {
    count: u64,
}

impl AppData {
    fn new() -> AppData {
        AppData {
            count: 0
        }
    }
}
