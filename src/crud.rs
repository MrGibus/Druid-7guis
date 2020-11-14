use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, WidgetExt, Color, LensExt};
use druid::widget::{Label, Flex, Align, TextBox, Button, Scroll, List, CrossAxisAlignment, MainAxisAlignment, Either};
use druid::lens::{self};
use druid::im::{Vector, vector};

const WINDOW_TITLE: &str = "CRUD";
const WINDOW_SIZE: Size = Size::new(500., 350.);
const WINDOW_SIZE_MIN: Size = Size::new(400., 250.);
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
    prefix: String,
    name: String,
    surname: String,
    list: Vector<ListItem>,
    current: Option<usize>  // selected list item
}

impl AppData {
    fn new() -> Self {
        let list = vector![
        "Emil, Hans".into(),
        "Mustermann, Max".into(),
        "Tisch, Roman".into()]
            .into_iter()
            .enumerate()
            .map(|(i, s)| ListItem::new(i, s))
            .collect();

        AppData {
            prefix: "".into(),
            name: "John".into(),
            surname: "Romba".into(),
            list,
            current: None
        }
    }

    fn filter(&self) -> Vector<ListItem> {
        let f = self.prefix.to_lowercase();
        self.list
            .clone()
            .into_iter()
            .filter(|s| s.item.to_lowercase().contains(f.as_str()))
            .collect()
    }
}

fn reindex(v: &Vector<ListItem>) -> Vector<ListItem>{
    let v2 = v.clone();
    v2
        .into_iter()
        .enumerate()
        .map(|(i, s)| ListItem::new(i, s.item))
        .collect::<Vector<ListItem>>()
}

fn build_ui() -> impl Widget<AppData> {

    // HEADER
    let head = Align::left(Flex::row()
        .with_child(Label::new("Filter Prefix:  "))
        .with_child(TextBox::new()
            .lens(AppData::prefix)
            // .controller(FilterController)
        )).fix_height(30.);

    // BODY
    let right_1 = Flex::row()
        .with_child(Label::new("Name:  "))
        .with_child(TextBox::new().lens(AppData::name));

    let right_2 = Flex::row()
        .with_child(Label::new("Surname:  "))
        .with_child(TextBox::new().lens(AppData::surname));

    let right = Flex::column()
        .with_child(right_1)
        .with_spacer(PADDING)
        .with_child(right_2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .padding(8.0);

    let list = Scroll::new(List::new(|| {
        new_item()
    }))
        .vertical()
        .lens(lens::Id.map(
            // Expose shared data with children data
            // Default: 'data.list.clone()' in place of data.filter()
        |data: &AppData| (data.current, data.filter()),
        |data: &mut AppData, (current, _list)| {
            data.current = current;
        }))
        .expand_width();

    let left = Flex::column()
        .with_flex_child(list, 1.)
        .expand_height()
        .expand_width()
        .padding(8.0)
        .background(Color::grey(0.4))
        .border(Color::grey(0.6), 2.0);

    let main_body = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(left, 1.)
        .with_spacer(PADDING)
        .with_child(right)
        .expand_width()
        .expand_height()
        .padding(PADDING)
        .border(Color::grey(0.2), 2.0)
        .rounded(5.0);

    // FOOTER
    let btn_create = Button::new("Create")
        .on_click(|_, data: &mut AppData, _| {
            // Just slap it onto the end
            let s = format!("{}, {}", data.surname, data.name);
            let new = ListItem::new(data.list.len(), s);
            data.list.append(vector![new])
        });

    let btn_update = Button::new("Update")
        .on_click(|_, data: &mut AppData, _| {
            if let Some(i) = data.current {
                let new = format!("{}, {}", data.surname, data.name);
                data.list[i].item = new
            }
        });

    let btn_delete = Button::new("Delete")
        .on_click(|_, data: &mut AppData, _| {
            if let Some(i) = data.current {
                //remove the item
                data.list.remove(i);
                data.list = reindex(&data.list);
                // set the current selection to nothing (could also be nearest element etc.)
                data.current = None;
            }
        });

    let foot = Align::left(Flex::row()
        .with_child(btn_create)
        .with_spacer(PADDING)
        .with_child(btn_update)
        .with_spacer(PADDING)
        .with_child(btn_delete));

    // ROOT
    Flex::column()
        .with_child(head)
        .with_spacer(PADDING)
        .with_flex_child(main_body, 1.)
        .with_spacer(PADDING)
        .with_child(foot)
        .padding(PADDING * 2.)
}

#[derive(Clone, Data, Lens)]
struct ListItem {
    index: usize,
    item: String,
}

impl ListItem {
    fn new(i: usize, s: String) -> Self {
        ListItem {
            index: i,
            item: s,
        }
    }
}

// create the list item widget
fn new_item() -> impl Widget<(Option<usize>, ListItem)> {
    Either::new(|data: &(Option<usize>, ListItem), _:&_| {
        if data.0.is_some() {
            data.0.unwrap() == data.1.index
        } else {
            false
        }},
    // TODO: Generalise this?
    Label::new(|data: &(Option<usize>, ListItem), _:&_|{
        data.1.item.to_string()
    })
        //format the true branch with some background colour
        .background(Color::rgba(0.2, 0.2, 0.6, 0.5))
        .expand_width(),
        // do not format the false branch
        Label::new(|data: &(Option<usize>, ListItem), _:&_| {
        data.1.item.to_string()
    }))
        .on_click(|_, data, _| {
        data.0 = Some(data.1.index);
    })
}