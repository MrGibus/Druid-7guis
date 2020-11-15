//! # A Timer application to demonstrate concurrent inputs
//! it's important to note that std::time should be used for tracking time like any other app
//! and not the timer provided by druid.

use druid::{
    AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, WidgetExt, TimerToken,
    Event, EventCtx, Env,
};
use druid::widget::{Button, Label, Flex, Align, ProgressBar, Slider, Controller};
use std::time::{Duration, Instant};


const WINDOW_TITLE: &str = "Timer";
const WINDOW_SIZE: Size = Size::new(300., 150.);
const MAX_TIME_RANGE: f64 = 30.;

//16ms is around 60fps
static TIMER_INTERVAL: Duration = Duration::from_millis(16);


pub fn main() -> Result<(), PlatformError> {
    let data = AppData::new();
    let window = WindowDesc::new(build_ui)
        .window_size(WINDOW_SIZE)
        .title(WINDOW_TITLE)
        .resizable(false);
    AppLauncher::with_window(window).launch(data)?;
    Ok(())
}


fn get_time(data: &AppData) -> f64 {
    if data.progress < 1. {
            let now = Instant::now();
    let elapsed = now - data.time;
    elapsed.as_secs_f64()
    } else {
        MAX_TIME_RANGE * data.slider_time
    }
}


fn build_ui() -> impl Widget<AppData> {

    let layout = Flex::column()
        .with_flex_spacer(0.05)
        .with_child(Flex::row()
            .with_child(Label::new("Elapsed Time: "))
            .with_flex_child(ProgressBar::new()
                                 .lens(AppData::progress)
                                 .controller(TimeControl::new())
                             , 1.)
        )
        .with_child(Align::left(Label::new(|data: &AppData, _: &_| {
            format!("{:.1}s", get_time(data))
        })))
        .with_child(Flex::row()
            .with_child(Label::new("Duration: "))
            .with_flex_child(Slider::new().lens(AppData::slider_time), 1.)
        )
        .with_flex_spacer(0.05)
        .with_child(Button::new("Reset").on_click(|_, data: &mut AppData, _: &_| {
            data.progress = 0.;
            data.time = Instant::now();
        }))
        .with_flex_spacer(0.05);

    Align::centered(layout)
}

#[derive(Clone, Data, Lens)]
struct AppData {
    // because data is not available for time
    #[data(same_fn = "PartialEq::eq")]
    time: Instant,
    progress: f64,
    slider_time: f64,
}

impl AppData {
    fn new() -> AppData {
        AppData {
            time: Instant::now(),
            progress: 0.,
            slider_time: 0.5,
        }
    }
}

struct TimeControl {
    timer_id: TimerToken,
}

impl TimeControl {
    fn new() -> Self {
        TimeControl {
            timer_id: TimerToken::INVALID,
        }
    }
}

impl <W: Widget<AppData>> Controller<AppData, W> for TimeControl {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {

        // intercept the event
        match event {
            // on start
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(TIMER_INTERVAL)
            },
            // This is not precise so we must calculate the actual time
            Event::Timer(id) => (
                if *id == self.timer_id {
                        //determine amount of time elapsed
                        let current_time = Instant::now();
                        let elapsed_time = current_time - data.time;
                        // update based on time elapsed since timer was started
                        data.progress = elapsed_time.as_secs_f64() / (data.slider_time * MAX_TIME_RANGE);
                //request an update
                self.timer_id = ctx.request_timer(TIMER_INTERVAL)
                }
            ),
            _ => () // do nothing for other events
        }

        // pass everything else to the child widget
        child.event(ctx, event, data, env);
    }
}