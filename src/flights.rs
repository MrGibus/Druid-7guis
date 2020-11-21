//! # An application to demonstrate data-validation
//! There are no drop-down widgets implemented at this stage. A checkbox has been substituted.
//! Custom widget functionality is explored elsewhere.

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size,
            Key, Color, WidgetExt, LocalizedString, Env, UpdateCtx, EventCtx, Event};
use druid::widget::{TextBox, Flex, Checkbox, Button, Controller};


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
        .lens(AppData::out_flight)
        .controller(TboxControl)
        .env_scope(|env,data: &AppData| {
            match data.out_state {
                TboxState::Standard => env.set(druid::theme::LABEL_COLOR, env.get(druid::theme::LABEL_COLOR)),
                TboxState::Invalid => env.set(druid::theme::LABEL_COLOR, env.get(TXT_CLR_INVALID)),
                TboxState::Disabled => env.set(druid::theme::LABEL_COLOR, env.get(BTN_CLR_DISABLED)),
            }
        });

    let tbox_return = TextBox::new()
        .expand_width()
        .lens(AppData::in_flight)
        .controller(TboxControl)
        .env_scope(|env,data: &AppData| {
            match data.in_state {
                TboxState::Standard => env.set(druid::theme::LABEL_COLOR, env.get(druid::theme::LABEL_COLOR)),
                TboxState::Invalid => env.set(druid::theme::LABEL_COLOR, env.get(TXT_CLR_INVALID)),
                TboxState::Disabled => env.set(druid::theme::LABEL_COLOR, env.get(BTN_CLR_DISABLED)),
            }
        });

    let btn_book = Button::new("Book")
        .expand_width()
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
        })
        .on_click(|_, data: &mut AppData, _: &_| submit(data))
        .controller(BtnController);

    let chk_box = Checkbox::new("Return")
        .lens(AppData::return_flight)
        .controller(CBoxController);


    Flex::column()
        .with_child(chk_box)
        .with_spacer(SPACING)
        .with_child(tbox_out)
        .with_spacer(SPACING)
        .with_child(tbox_return)
        .with_flex_spacer(1.)
        .with_child(btn_book)
        .padding(SPACING)
}

#[derive(Clone, Data, PartialEq, Debug)]
enum TboxState {
    Standard,
    Invalid,
    Disabled,
}


/// ## App State
#[derive(Clone, Data, Lens, Debug)]
struct AppData {
    return_flight: bool,
    out_flight: String,
    in_flight: String,
    out_state: TboxState,
    in_state: TboxState,
}

impl AppData {
    fn new() -> AppData {

        AppData {
            return_flight: false,
            out_flight: "27.03.2021".into(),
            in_flight: "14.04.2021".into(),
            out_state: TboxState::Standard,
            in_state: TboxState::Disabled,
        }
    }

    fn btn_valid(&self) -> bool {
        if self.return_flight {
            self.in_state == TboxState::Standard && self.out_state == TboxState::Standard
        } else {
            self.out_state == TboxState::Standard
        }
    }

    // To update the text box states based on their inputs
    fn update_states(&mut self) {
        let out_date = Date::from_str(self.out_flight.as_str());
        let in_date = Date::from_str(self.in_flight.as_str());

        // out flight must either be standard or invalid
        match out_date {
            Ok(_) => {
                self.out_state = TboxState::Standard;
            },
            Err(_) => self.out_state = TboxState::Invalid,
        }

        if self.return_flight {
            match in_date {
                Ok(date) => {
                    if out_date.is_ok() && date.is_before(&out_date.unwrap()) {
                            self.in_state = TboxState::Invalid;
                    } else {
                        // we don't want to throw an error here if out flight is invalid
                        self.in_state = TboxState::Standard;
                    }
                },
                Err(_) => self.in_state = TboxState::Invalid,
            }
        }
        // if not a return flight in state must be disabled
        else {
            self.in_state = TboxState::Disabled
        }
    }
}

/// ## Widget extensions

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
        child.update(ctx, old, data, env);
        // repaint widget everytime data changes
        ctx.request_paint()
    }
}

/// ## Checkbox functionality override:
/// because lensed data would not been updated until AFTER the controller
/// we must flip data, request paint and *not* pass the event to the child:
/// It's expected that this will change with env_scope changes in version 0.7
struct CBoxController;

impl <W: Widget<AppData>> Controller<AppData, W> for CBoxController {
    fn event(
        &mut self,
        _child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        _env: &Env,
    ) {
        if let Event::MouseDown(_) = event {
            data.return_flight = !data.return_flight;
            data.update_states();
        }
        ctx.request_paint();
    }
}

/// ## Textbox(s) functionality override:
/// This is boilerplate to update the state on a keypress

struct TboxControl;

impl <W: Widget<AppData>> Controller<AppData, W> for TboxControl {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        // Pass to child first to save an update block
        child.event(ctx, event, data, env);

        if let Event::KeyDown(_) = event {
            data.update_states()
        }
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

/// could also implement a datetime library, but given our needs we will just make our own
/// with the minimal functionality needed to demonstrate UI requirements
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

        // Use .get to verify position exists
        let v_day = vect.get(0);
        let v_month = vect.get(1);
        let v_year = vect.get(2);

        // If all of the items exist, then parse and return the result
        if v_year.is_some() && v_month.is_some() && v_day.is_some() {
            let day: u16 = v_day.unwrap().parse()?;
            let month: u16 = v_month.unwrap().parse()?;
            let year: u16 = v_year.unwrap().parse()?;

            Ok(Date {
                day,
                month,
                year,
            })

        } // Otherwise return a read error
        else {
            Err("read error: Enter full date".into())
        }
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