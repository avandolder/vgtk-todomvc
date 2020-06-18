use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
struct Model {
    tasks: Vec<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tasks: vec![
                "Call Joe".to_string(),
                "Call Mike".to_string(),
                "Call Robert".to_string(),
                "Get Robert to fix the bug".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
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
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("com.example.vgtk-todomvc"), ApplicationFlags::empty())>
                <Window border_width=20 on destroy=|_| Message::Exit>
                    <ListBox>
                        {
                            self.tasks.iter().map(|task| gtk! {
                                <ListBoxRow>
                                    <Label label=task.clone() />
                                </ListBoxRow>
                            })
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
