use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

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
                </Box>
            </ListBoxRow>
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    tasks: Vec<Task>,
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
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    Toggle { index: usize },
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
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("com.example.vgtk-todomvc"), ApplicationFlags::empty())>
                <Window border_width=20 on destroy=|_| Message::Exit>
                    <ListBox>
                        {
                            self.tasks.iter().enumerate().map(|(idx, task)| task.render(idx))
                        }
                    </ListBox>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
