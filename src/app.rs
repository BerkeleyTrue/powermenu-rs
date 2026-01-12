use iced::{
    Element, Event, Size, Subscription, Task, event, exit,
    keyboard::{self, Key, key::Named},
    widget::{self, column},
};
use tokio::process::Command;
use tracing::{debug, info};

use crate::Cli;

pub const SIZE: Size = Size {
    width: 623.0,
    height: 390.0,
};

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

#[derive(Debug)]
pub enum Message {
    User(String, Option<String>),
    Lock,
    Sleep,
    Shutdown,
    Reboot,
    Logout,

    QuitApp,
}

#[derive(Default)]
pub struct AppModel {
    dryrun: bool,
    user: Option<String>,
    no_focus: bool,
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

impl AppModel {
    pub fn new(init: Init) -> (Self, Task<Message>) {
        // TODO: convert dead internet
        // let dead_internet_video = DeadInternet::builder().launch(()).detach();
        // let dead_internet = dead_internet_video.widget();

        (
            AppModel {
                dryrun: init.dryrun,
                no_focus: init.no_focus,
                user: None,
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

        Subscription::batch(vec![key_events, app_events])
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            widget::text(
                self.user
                    .as_ref()
                    .map(|u| format!("Hello {u}"))
                    .unwrap_or("Hello World".to_string())
            )
            .size(18)
        ]
        .spacing(10)
        .into()
        // gtk::Window {
        //     set_title: Some("DeadInternet"),
        //     add_css_class: "dead-internet",
        //     set_default_width: WIDTH,
        //     set_default_height: HEIGHT,
        //     set_margin_all: 0,
        //
        //     connect_close_request[sender] => move |_| {
        //         info!("close request");
        //         sender.input(AppMessage::QuitApp);
        //         gtk::glib::Propagation::Stop
        //     },
        //
        //     connect_is_active_notify[sender, no_focus = init.no_focus] => move |window| {
        //         if !window.is_active() {
        //             info!("lost focus");
        //             if !no_focus {
        //                 sender.input(AppMessage::QuitApp);
        //             }
        //         }
        //     },
        //
        //     gtk::Overlay {
        //         add_css_class: "overlay",
        //
        //         #[local_ref]
        //         dead_internet -> gtk::Picture {
        //             set_hexpand: true,
        //             set_vexpand: true,
        //             set_content_fit: gtk::ContentFit::Cover,
        //         },
        //
        //         add_overlay = &gtk::Box {
        //             set_orientation: gtk4::Orientation::Vertical,
        //
        //             gtk::FlowBox {
        //                 set_max_children_per_line: 5,
        //                 set_min_children_per_line: 2,
        //                 set_selection_mode: gtk::SelectionMode::Single,
        //                 set_homogeneous: true,
        //                 set_row_spacing: 6,
        //                 set_column_spacing: 6,
        //                 set_margin_all: 12,
        //
        //                 gtk::FlowBoxChild {
        //                     set_focusable: false,
        //
        //                     gtk::Button {
        //                         add_css_class: "btn",
        //                         set_icon_name: icon_names::ROTATION_LOCK,
        //                         connect_clicked => AppMessage::Lock,
        //                     }
        //                 },
        //                 gtk::FlowBoxChild {
        //                     set_focusable: false,
        //
        //                     gtk::Button {
        //                         add_css_class: "btn",
        //                         set_icon_name: icon_names::MOON_OUTLINE,
        //                         connect_clicked => AppMessage::Sleep,
        //                     }
        //                 },
        //                 gtk::FlowBoxChild {
        //                     set_focusable: false,
        //
        //                     gtk::Button {
        //                         add_css_class: "btn",
        //                         set_icon_name: icon_names::ARROW_CIRCULAR_SMALL_BOTTOM_RIGHT,
        //                         connect_clicked => AppMessage::Reboot,
        //                     },
        //                 },
        //                 gtk::FlowBoxChild {
        //                     set_focusable: false,
        //
        //                     gtk::Button {
        //                         add_css_class: "btn",
        //                         set_icon_name: icon_names::TURN_OFF,
        //                         connect_clicked => AppMessage::Shutdown,
        //                     },
        //                 },
        //                 gtk::FlowBoxChild {
        //                     set_focusable: false,
        //
        //                     gtk::Button {
        //                         add_css_class: "btn",
        //                         set_icon_name: icon_names::ARROW_INTO_BOX,
        //                         connect_clicked => AppMessage::Logout,
        //                     }
        //                 }
        //             },
        //
        //             gtk::Box {
        //                 gtk::Label {
        //                     add_css_class: "user-label",
        //                     set_halign: gtk::Align::Center,
        //                     set_valign: gtk::Align::Center,
        //                     set_text: &user,
        //                 },
        //             }
        //         },
        //     }
    }
}
