//! # WIP: An application to demonstrate data-validation

//TODO:
// [] disabled items (https://github.com/linebender/druid/issues/746),
// [] Enum for checking the state of textbox widgets: std, disabled, invalid
// [] dropdown menu

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, RenderContext, Data, Lens, Size, Key, Color, WidgetExt, LocalizedString, Env, UpdateCtx};
use druid::widget::{TextBox, Flex, Checkbox, Button, Painter, Controller};


/// ## ENV Keys
/// https://linebender.org/druid/env.html

const TXT_CLR_INVALID: Key<Color> = Key::new("app.txt.clr.invalid");
const BTN_CLR_DISABLED: Key<Color> = Key::new("app.btn.clr.disabled");

/// ## Constants
const WINDOW_SIZE: Size = Size::new(250., 225.);
const SPACING: f64 = 15.;

/// ## Entry Point
pub fn main() -> Result<(), PlatformError> {
    // model data
    let data = AppData::new();

    // create the window and ui
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .title(LocalizedString::new("multiwin-demo-window-title")
            .with_placeholder("Flight booker"))
        .resizable(false);

    // link ui and data starts loop
    AppLauncher::with_window(window)
        // Set environment keys
        .configure_env(|env, _state| {
            env.set(TXT_CLR_INVALID, Color::rgb(0.85, 0.05, 0.1));
            env.set(BTN_CLR_DISABLED, Color::grey(0.5));
        })
        .launch(data)?;
    Ok(())
}

/// ## Builder
fn build_ui() -> impl Widget<AppData> {

    let tbox_out = TextBox::new()
        .expand_width()
        .lens(AppData::out_flight);
        // .env_scope(|env,data: &AppData| {
        //     if !data.out_valid {
        //         env.set(druid::theme::LABEL_COLOR, env.get(TXT_CLR_INVALID));
        //     }
        // });

    let tbox_return = TextBox::new()
        .expand_width()
        .lens(AppData::in_flight);

    let btn_book = Button::new("Book")
        .expand_width()
        .on_click(|_, data: &mut AppData, _: &_| submit(data))
        .controller(BtnController)
        .env_scope(|env,data: &AppData| {
            if data.btn_valid() {
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
        });

    Flex::column()
        .with_child(Checkbox::new("Return").lens(AppData::return_flight))
        .with_spacer(SPACING)
        .with_child(tbox_out)
        .with_spacer(SPACING)
        .with_child(tbox_return)
        .with_flex_spacer(1.)
        .with_child(btn_book)
        .padding(SPACING)
}


/// ## App State
#[derive(Clone, Data, Lens)]
struct AppData {
    return_flight: bool,
    out_flight: String,
    in_flight: String,
    out_valid: bool,
    in_valid: bool,
}

impl AppData {
    fn new() -> AppData {
        let default_str = "27.03.2014";

        AppData {
            return_flight: false,
            out_flight: default_str.into(),
            in_flight: default_str.into(),
            in_valid: true,
            out_valid: false
        }
    }

    fn btn_valid(&self) -> bool {
        if self.return_flight {
            self.in_valid && self.out_valid
        } else {
            self.in_valid
        }
    }
}

/// ## Widget extensions
// Revise the update mehthod of the widget to tell it to redraw once the validity has changed
struct BtnController;

impl <W: Widget<AppData>> Controller<AppData, W> for BtnController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old: &AppData,
        data: &AppData,
        env: &Env
    ) {
        if old.btn_valid() != data.btn_valid() {
            ctx.request_paint()
        };
        child.update(ctx, old, data, env);
    }
}

/// ## Application Logic

fn submit(data: &mut AppData) {

    let out_flight = Date::from_str(data.out_flight.as_str());

    if data.return_flight {
        let in_flight = Date::from_str(data.in_flight.as_str());
        if in_flight.is_ok() && out_flight.is_ok() && out_flight.unwrap().is_before(&in_flight.unwrap()) {
                println!("Return flight\nleave: {}\nreturn:{}\n", data.out_flight, data.in_flight);
        }
    } else if out_flight.is_ok() {
            println!("One-way flight\nleave: {}\n", data.out_flight);
        }
}

struct Date {
    day: u16,
    month: u16,
    year: u16,
}

impl Date {
    fn from_str(input: &str) -> Result<Date, Box<dyn std::error::Error>> {
        // remove whitespace
        let input: String = input.split_whitespace().collect();
        // split via '.' character and map to a vector
        let vect: Vec<String> = input.split('.').map(|s| s.to_string()).collect();

        let day: u16 = vect[0].parse()?;
        let month: u16 = vect[1].parse()?;
        let year: u16 = vect[2].parse()?;

        Ok(Date {
            day,
            month,
            year,
        })
    }

    fn is_after(&self, other: &Date) -> bool {
        if self.year > other.year {
            true
        } else if self.year < other.year {
            false
        } else if self.month > other.month {
            true
        } else if self.month < other.month {
            false
        } else { self.day > other.day }
    }

    fn is_before(&self, other: &Date) -> bool {
        if self.is_equal(&other) {
            false
        } else {
            !self.is_after(&other)
        }
    }

    fn is_equal(&self, other: &Date) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.day, self.month, self.year)
    }
}