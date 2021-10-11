use chrono::prelude::*;
use info_keep_lib::{InfoKeep,Tag};

use iced::{
    button, pick_list, scrollable, text_input, Align, Button, Container, Element, Length, PickList,
    Sandbox, Scrollable, Settings, Space, Text, TextInput,
};
use std::ops::Deref;

pub fn main() {
    InfoKeepIced::run(Settings::default()).expect("Could not open GUI");
}

#[derive(Default)]
struct InfoKeepIced {
    scroll: scrollable::State,
    text_box: text_input::State,
    mode_section: pick_list::State<Options>,
    input_data: Option<String>,
    selected_mode: Option<Options>,
    confirm_button: button::State,
    output: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    InputData(String),
    ModeSection(Options),
    ConfirmButton,
    // Output
}

impl Sandbox for InfoKeepIced {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Info Keep")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ModeSection(option) => self.selected_mode = Some(option),
            Message::InputData(text) => self.input_data = Some(text),
            Message::ConfirmButton => {
                run_command(
                    self.selected_mode.expect("No mode selected"),
                    self.input_data.clone(),
                );
                self.input_data = Some(String::new());
            } // Message::Output => self.output = Some(output_db())
        }
    }

    fn view(&mut self) -> Element<Message> {
        let pick_list = PickList::new(
            &mut self.mode_section,
            &Options::ALL[..],
            self.selected_mode,
            Message::ModeSection,
        );

        let place_holder = if self.selected_mode == Some(Options::DeleteEntry) {
            "Entry key for deletion"
        } else {
            "Input data to keep"
        };

        let value = match self.input_data.as_ref() {
            None => String::new(),
            Some(text) => text.to_string(),
        };

        let output = match self.output.as_ref() {
            None => output_db(),
            Some(text) => {
                if self.selected_mode != Some(Options::SearchDB) {
                    text.to_string()
                } else {
                    run_command(Options::SearchDB, self.input_data.clone()) +"\n"+ &output_db()
                }
            }
        };

        println!("{}", &output);

        let input_box = TextInput::new(
            &mut self.text_box,
            place_holder,
            &*value,
            Message::InputData,
        )
        .padding(10)
        .width(Length::FillPortion(10));

        let button_confirm = Button::new(&mut self.confirm_button, Text::new("Confirm"))
            .on_press(Message::ConfirmButton);

        let mut content = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(Space::with_height(Length::Units(5)))
            .push(Text::new("Select Mode"))
            .push(pick_list)
            .push(input_box)
            .push(button_confirm)
            .push(Text::new(output));

        content = content.push(Space::with_height(Length::Units(600)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Options {
    NewEntry,
    ExportDB,
    ImportDB,
    DeleteEntry,
    SearchDB,
}

impl Options {
    const ALL: [Options; 5] = [
        Options::NewEntry,
        Options::ExportDB,
        Options::ImportDB,
        Options::DeleteEntry,
        Options::SearchDB,
    ];
}

impl Default for Options {
    fn default() -> Options {
        Options::NewEntry
    }
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Options::NewEntry => "New Entry",
                Options::ExportDB => "Export Database",
                Options::ImportDB => "Import Database",
                Options::DeleteEntry => "Delete Entry",
                Options::SearchDB => "Search DB",
            }
        )
    }
}

fn output_db() -> String {
    let mut db = InfoKeep::init("Dates");
    let keys = db.sort_db(false);
    let mut outputs: Vec<String> = Vec::new();
    for (k, v) in keys {
        outputs.push(format!(
            "{} :: {}\n",
            String::from_utf8_lossy(&*k),
            String::from_utf8_lossy(&*v)
        ))
    }
    let mut mid = String::new();
    for output in outputs {
        mid += output.as_str()
    }
    mid
}

fn run_command(mode: Options, data: Option<String>) -> String {
    let mut db: InfoKeep = InfoKeep::init("Dates");
    db.sort_db(false);
    let key = Utc::now().format("%Y-%m-%d+%H:%M:%S").to_string();
    let mut output: String = "".to_string();
    match mode {
        Options::NewEntry => {
           output = db.new_entry( &key, &data.unwrap());
        }
        Options::ExportDB => {
            db.export_db();
        }
        Options::ImportDB => {
            db.import_db();
        }
        Options::DeleteEntry => {
            db.remove_info(&data.unwrap());
        }
        Options::SearchDB => {
            let mid: Vec<String> = {
                let tmp = data.unwrap();
                tmp.split('+').map(|s| s.to_string()).collect()
            };

            let dates: Vec<String> = {
                let tmp: Vec<&str> = mid.get(0).unwrap().split('-').collect();
                tmp.iter().map(|s| s.deref().deref().to_string()).collect()
            };

            let tag = Tag::new(
                dates.get(0).map(|s| s.as_str()),
                dates.get(1).map(|s| s.as_str()),
                dates.get(2).map(|s| s.as_str()),
            );
            output = db.search_tag( tag);
        }
    }
    output
}
