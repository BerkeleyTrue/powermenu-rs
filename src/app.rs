use iced::{
    Border, Color, Element, Event,
    Length::{self, Fill},
    Padding, Radians, Shadow, Subscription, Task, Theme, Vector, event, exit,
    gradient::Linear,
    keyboard::{self, Key, key::Named},
    padding,
    widget::{column, container, row, space, stack, text},
};
use tokio::process::Command;
use tracing::{debug, info};

use crate::{
    Cli,
    button::{Icon, PowerButton},
    dead_internet,
};

pub const SIZE: (u32, u32) = (623, 390);
pub const REM: f32 = 14.0;

#[derive(Clone)]
pub struct Init {
    pub no_focus: bool,
    pub dryrun: bool,
}

impl From<Cli> for Init {
    fn from(value: Cli) -> Self {
        Self {
            dryrun: value.dryrun,
            no_focus: value.no_focus,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    User(String, Option<String>),
    DeadInternet(dead_internet::Message),
    Lock,
    Sleep,
    Shutdown,
    Reboot,
    Logout,

    QuitApp,
}

pub struct App {
    dead_internet: dead_internet::DeadInternet,
    dryrun: bool,
    user: Option<String>,
    no_focus: bool,
    buttons: Vec<PowerButton<Message>>,
}

async fn get_user() -> Message {
    let user = Command::new("whoami")
        .output()
        .await
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or("Anon".to_string());

    let host = Command::new("uname")
        .arg("-n")
        .output()
        .await
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .ok();

    debug!("user: {}, host: {:?}", user, host);

    Message::User(user, host)
}

impl App {
    pub fn new(init: Init) -> (Self, Task<Message>) {
        (
            App {
                dead_internet: dead_internet::DeadInternet::new(),
                dryrun: init.dryrun,
                no_focus: init.no_focus,
                user: None,
                buttons: vec![
                    PowerButton {
                        icon: Icon::Lock,
                        message: Message::Lock,
                    },
                    PowerButton {
                        icon: Icon::Sleep,
                        message: Message::Sleep,
                    },
                    PowerButton {
                        icon: Icon::Reboot,
                        message: Message::Reboot,
                    },
                    PowerButton {
                        icon: Icon::Shutdown,
                        message: Message::Shutdown,
                    },
                    PowerButton {
                        icon: Icon::Logout,
                        message: Message::Logout,
                    },
                ],
            },
            Task::future(get_user()),
        )
    }

    fn command(&self, program: &str, args: Vec<&str>) -> Task<Message> {
        let mut cmd = std::process::Command::new(&program);
        cmd.args(&args);

        if self.dryrun {
            debug!("{cmd:#?}");
            let args_print = args.join(" ");
            println!("dryrun: {program} {args_print}");
        } else {
            cmd.output()
                .map_err(|err| format!("Error running command: {err:?}"))
                .unwrap();
        }
        exit()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QuitApp => exit(),
            Message::Lock => {
                info!("Lock Request");
                self.command("loginctl", vec!["lock-session"])
            }
            Message::Sleep => {
                info!("Sleep Request");
                self.command("systemctl", vec!["suspend"])
            }
            Message::Reboot => {
                info!("Reboot Request");
                self.command("systemctl", vec!["reboot"])
            }
            Message::Shutdown => {
                info!("Shutdown Request");
                self.command("systemctl", vec!["poweroff"])
            }
            Message::Logout => {
                info!("Logout request");
                self.command(
                    "systemctl",
                    vec!["--user", "start", "shutdown-graphical.target"],
                )
            }
            Message::User(user, host) => {
                self.user = host.map(|host| format!("{user}@{host}")).or(Some(user));
                Task::none()
            }
            Message::DeadInternet(message) => self
                .dead_internet
                .update(message)
                .map(Message::DeadInternet),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let no_focus = self.no_focus.clone();
        let key_events = keyboard::listen()
            .filter_map(|event| match event {
                keyboard::Event::KeyPressed { key, .. } => Some(key),
                _ => None,
            })
            .filter_map(|key| match key.as_ref() {
                Key::Named(Named::Escape) | Key::Character("q") => Some(Message::QuitApp),
                _ => None,
            });

        let app_events = event::listen_with(move |event, _, _| match event {
            Event::Window(iced::window::Event::Unfocused) => Some(Message::QuitApp),
            _ => None,
        })
        .with(no_focus)
        .filter_map(move |(no_focus, e)| if no_focus { None } else { Some(e) });

        let dead_internet_subs = self
            .dead_internet
            .subscriptions()
            .map(Message::DeadInternet);

        Subscription::batch(vec![key_events, app_events, dead_internet_subs])
    }

    pub fn view(&self) -> Element<'_, Message> {
        let dead_internet = self.dead_internet.view().map(Message::DeadInternet);
        let buttons = row(self.buttons.iter().map(|b| b.view()).collect::<Vec<_>>())
            .padding(padding::top(10))
            .width(Fill);

        let user_container = self
            .user
            .as_ref()
            .map(|user| {
                let inner_box = container(text(user.clone()))
                    .padding(10)
                    .style(|theme: &Theme| {
                        let palette = theme.palette();

                        container::Style {
                            background: Some(
                                Linear::new(Radians::PI)
                                    .add_stop(0.0, palette.text)
                                    .add_stop(0.80, palette.text)
                                    .add_stop(0.81, palette.primary)
                                    .add_stop(0.87, palette.primary)
                                    .add_stop(0.88, palette.success)
                                    .into(),
                            ),
                            border: Border::default()
                                .rounded(1.0)
                                .color(palette.background)
                                .width(2.0),
                            ..Default::default()
                        }
                    });
                let outer_box = container(inner_box)
                    .style(|theme: &Theme| {
                        let palette = theme.palette();

                        container::Style {
                            border: Border::default()
                                .rounded(1.0)
                                .color(palette.text)
                                .width(2.0),
                            ..Default::default()
                        }
                    })
                    .padding(1);
                container(outer_box).width(Fill)
            })
            .unwrap_or_else(|| container(space()).width(Fill));

        let content = column![
            buttons,
            row![user_container].padding(Padding::from([15, 55]))
        ];

        // main layout
        let main_layout = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.palette();

                container::Style {
                    background: None,
                    border: Border::default()
                        .color(palette.text)
                        .rounded(2.0)
                        .width(2.0),
                    shadow: Shadow {
                        blur_radius: 8.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.75),
                        offset: Vector::new(3.0, 3.0),
                    },
                    ..Default::default()
                }
            })
            .padding(8);

        stack![dead_internet, main_layout].into()
    }
}
