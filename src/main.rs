mod counter;
mod temperature;
mod flights;
mod timer;
mod crud;
mod circles;

use std::io;

// TEMP CLI because I can't get multiwindows working without modifying the example source
pub fn main() {
    println!("Enter example 1 - 7");
    println!("1: Counter");
    println!("2: Temperature Converter");
    println!("3: Flight Booker");
    println!("4: Timer");
    println!("5: CRUD (create, read, update, delete)");
    println!("6: Circle Drawing");
    println!("7: Cells");

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", input);
        }
        Err(error) => println!("Error: {}", error)
    };

    let v = input.trim().parse::<u32>().expect("please enter a number from 1- 7");
    match v {
        1 => {
            println!("Counter Selected");
            counter::main().expect("launch Failed");
        },
        2 => {
            println!("Temperature Converter Selected");
            temperature::main().expect("Launch Failed");
        },
        3 => {
            println!("Flight Booker Selected");
            flights::main().expect("Launch Failed");
        },
        4 => {
            println!("Timer Selected");
            timer::main().expect("Launch Failed");
        },
        5 => {
            println!("CRUD Selected");
            crud::main().expect("Launch Failed");
        },
        6 => {
            println!("Circle drawer Selected");
            circles::main().expect("Launch Failed");
        },
        7 => {
            println!("Cells Selected");
            println!("NOT YET IMPLEMENTED");
        },
        _ => ()
    }
}

// TODO: Druid app launcher
// use druid::{commands as sys_cmds, AppLauncher, WindowDesc, Widget, PlatformError, DelegateCtx, Target, Command, Env, Size, widget::{Button, Label, Flex, Align}, WidgetExt, AppDelegate, Application};

// const WINDOW_TITLE: &str = "Menu";
// const WINDOW_SIZE: Size = Size::new(250., 300.);

// fn main() -> Result<(), PlatformError> {
//     let window = WindowDesc::new(build_ui)
//         .window_size(WINDOW_SIZE)
//         .resizable(false)
//         .title(WINDOW_TITLE);
//     AppLauncher::with_window(window)
//         .launch(())?;
//     Ok(())
// }
//
// fn build_ui() -> impl Widget<()> {
//     let layout = Flex::column()
//         .with_child(Label::new("7Guis in Druid"))
//         .with_flex_spacer(1.)
//         .with_child(Button::new("Counter")
//              .on_click(|ctx, _, _: &_| {
//                  Application::global().quit();
//                 // let data = counter::AppData::new();
//                 // let new_win = WindowDesc::new(counter::build_ui)
//                 //     .title(counter::WINDOW_TITLE)
//                 //     .resizable(false)
//                 //     .window_size(counter::WINDOW_SIZE);
//                 // AppLauncher::with_window(new_win)
//                 //     .launch(data)
//                 //     .expect("Launch Failed");
//              })
//             .expand_width())
//         .with_child(Button::new("Temperature")
//              .on_click(|_, _, _: &_| {
//                  println!("bar");
//              })
//             .expand_width())
//         .with_child(Button::new("Flight Booker")
//              .on_click(|_, _, _: &_| {
//                  println!("bar");
//              })
//             .expand_width())
//         .with_child(Button::new("Timer")
//              .on_click(|_, _, _: &_| {
//                  println!("bar");
//              })
//             .expand_width())
//         .with_child(Button::new("CRUD")
//              .on_click(|_, _, _: &_| {
//                  println!("WIP");
//              })
//             .expand_width())
//         .with_child(Button::new("Circle Drawer")
//              .on_click(|_, _, _: &_| {
//                  println!("WIP");
//              })
//             .expand_width())
//         .with_child(Button::new("Cells")
//              .on_click(|_, _, _: &_| {
//                  println!("WIP");
//              })
//             .expand_width())
//         .padding(20.);
//
//     Align::centered(layout)
// }
