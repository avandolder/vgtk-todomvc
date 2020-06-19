#![recursion_limit = "1024"]

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Callback, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
struct Task {
    text: String,
    done: bool,
}

impl Task {
    fn new(text: impl ToString, done: bool) -> Self {
        Self {
            text: text.to_string(),
            done,
        }
    }

    fn label(&self) -> String {
        if self.done {
            format!(
                "<span strikethrough=\"true\" alpha=\"50%\">{}</span>",
                self.text
            )
        } else {
            self.text.clone()
        }
    }

    fn render(&self, index: usize) -> VNode<Model> {
        gtk! {
            <ListBoxRow>
                <Box>
                    <CheckButton active=self.done on toggled=|_| Message::Toggle { index } />
                    <Label label=self.label() use_markup=true />
                    <Button Box::pack_type=PackType::End
                            relief=ReliefStyle::None image="edit-delete"
                            on clicked=|_| Message::Delete { index } />
                </Box>
            </ListBoxRow>
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Radio {
    labels: &'static [&'static str],
    active: usize,
    on_changed: Callback<usize>,
}

#[derive(Clone, Debug)]
enum RadioMessage {
    Changed(usize),
}

impl Component for Radio {
    type Message = RadioMessage;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        if *self == props {
            UpdateAction::None
        } else {
            *self = props;
            UpdateAction::Render
        }
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            RadioMessage::Changed(idx) => {
                self.on_changed.send(idx);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box spacing=10>
                {
                    self.labels.iter().enumerate().map(|(idx, lbl)| gtk! {
                        <ToggleButton label={ *lbl }
                                active={ idx == self.active }
                                on toggled=|_| RadioMessage::Changed(idx) />
                    })
                }
            </Box>
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    tasks: Vec<Task>,
    filter: usize,
}

impl Model {
    fn filter_task(&self, task: &Task) -> bool {
        match self.filter {
            // "All"
            0 => true,
            // "Active"
            1 => !task.done,
            // "Completed"
            2 => task.done,
            // Index out of bounds
            _ => unreachable!(),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task::new("Call Joe", true),
                Task::new("Call Mike", true),
                Task::new("Call Robert", false),
                Task::new("Get Robert to fix the bug", false),
            ],
            filter: 0,
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    Toggle { index: usize },
    Add { task: String },
    Delete { index: usize },
    Filter { filter: usize },
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::Toggle { index } => {
                self.tasks[index].done = !self.tasks[index].done;
                UpdateAction::Render
            }
            Message::Add { task } => {
                self.tasks.push(Task::new(task, false));
                UpdateAction::Render
            }
            Message::Delete { index } => {
                self.tasks.remove(index);
                UpdateAction::Render
            }
            Message::Filter { filter } => {
                self.filter = filter;
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("com.example.vgtk-todomvc"), ApplicationFlags::empty())>
                <Window default_width=800 default_height=600
                        border_width=20 on destroy=|_| Message::Exit>
                    <Box orientation=Orientation::Vertical spacing=10>
                        <Entry placeholder_text="What needs to be done?"
                                on activate=|entry| {
                                    entry.select_region(0, -1);
                                    Message::Add {
                                        task: entry.get_text().unwrap().to_string()
                                    }
                                } />
                        <ScrolledWindow Box::fill=true Box::expand=true>
                            <ListBox selection_mode=SelectionMode::None>
                                {
                                    self.tasks.iter().filter(|task| self.filter_task(task))
                                        .enumerate().map(|(idx, task)| task.render(idx))
                                }
                            </ListBox>
                        </ScrolledWindow>
                        <Box>
                            <@Radio Box::center_widget=true active=self.filter
                                    labels=["All", "Active", "Completed"].as_ref()
                                    on changed=|filter| Message::Filter { filter } />
                        </Box>
                    </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
