// debugging is like being the detective for a crime you also committed.

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, widget::{Label, TextBox, Flex, Align, Checkbox, Button, Either}, WidgetExt, Color, PaintCtx};
use druid::widget::SizedBox;

const WINDOW_TITLE: &str = "Flight booker";
const WINDOW_SIZE: Size = Size::new(200., 250.);
const WIDGET_W: f64 = 150.;
const WIDGET_H: f64 = 25.;
const SPACING: f64 = 15.;

//TODO: disabled items, dropdown menu
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


fn submit(data: &mut AppData) {

    let out_flight = Date::from_str(data.out_flight.as_str());


    if data.return_flight {
        let in_flight = Date::from_str(data.in_flight.as_str());
        if in_flight.is_ok() && out_flight.is_ok() {
            if out_flight.unwrap().is_before(&in_flight.unwrap()) {
                println!("Return flight\nleave: {}\nreturn:{}\n", data.out_flight, data.in_flight);
            }
        }
    } else {
        if out_flight.is_ok() {
            println!("One-way flight\nleave: {}\n", data.out_flight);
        }
    }
}


// define UI
fn build_ui() -> impl Widget<AppData> {

    let tbox_return = Either::new(|data, _| data.return_flight,
                                  TextBox::new().fix_size(WIDGET_W, WIDGET_H).lens(AppData::in_flight),{
            Label::new("").fix_size(WIDGET_W, WIDGET_H)

                                  }
    );

    let btn_book = Button::new("Book")
        .fix_size(WIDGET_W, WIDGET_H)
        .on_click(|_, data: &mut AppData, _: &_| submit(data));

    let layout = Flex::column()
        .with_child(Checkbox::new("Return").lens(AppData::return_flight))
        .with_spacer(SPACING)
        .with_child(TextBox::new().fix_size(WIDGET_W, WIDGET_H).lens(AppData::out_flight))
        .with_spacer(SPACING)
        .with_child(tbox_return)
        .with_spacer(SPACING * 2.)
        .with_child(btn_book);

    Align::centered(layout)
}


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