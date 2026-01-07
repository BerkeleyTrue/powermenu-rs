use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{
    gtk::{self, gdk::Key, gio::prelude::ApplicationExt, glib, prelude::*},
    prelude::*,
};
use tracing::info;

use crate::{dead_internet::DeadInternet, icon_names};

const WIDTH: i32 = 623;
const HEIGHT: i32 = 390;

pub struct InitApp {
    pub no_focus: bool,
}

pub struct AppModel {
    dead_internet: Controller<DeadInternet>,
}

#[derive(Debug)]
pub enum AppMessage {
    Lock,
    Sleep,
    Shutdown,
    Reboot,
    Logout,

    QuitApp,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = InitApp;
    type Input = AppMessage;
    type Output = ();
    view! {
        gtk::Window {
            set_title: Some("DeadInternet"),
            add_css_class: "dead-internet",
            set_default_width: WIDTH,
            set_default_height: HEIGHT,
            set_margin_all: 0,

            connect_close_request[sender] => move |_| {
                info!("close request");
                sender.input(AppMessage::QuitApp);
                gtk::glib::Propagation::Stop
            },

            connect_is_active_notify[sender, no_focus = init.no_focus] => move |window| {
                if !window.is_active() {
                    info!("lost focus");
                    if !no_focus {
                        sender.input(AppMessage::QuitApp);
                    }
                }
            },

            gtk::Overlay {
                add_css_class: "overlay",
                #[local_ref]
                dead_internet -> gtk::Picture {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_content_fit: gtk::ContentFit::Cover,
                },

                add_overlay = &gtk::FlowBox {
                    set_valign: gtk::Align::Start,
                    set_max_children_per_line: 5,
                    set_min_children_per_line: 2,
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_homogeneous: true,
                    set_row_spacing: 6,
                    set_column_spacing: 6,
                    set_margin_all: 12,

                    gtk::FlowBoxChild {
                        set_focusable: false,

                        gtk::Button {
                            add_css_class: "btn",
                            set_icon_name: icon_names::ROTATION_LOCK,
                            connect_clicked => AppMessage::Lock,
                        }
                    },
                    gtk::FlowBoxChild {
                        set_focusable: false,

                        gtk::Button {
                            add_css_class: "btn",
                            set_icon_name: icon_names::MOON_OUTLINE,
                            connect_clicked => AppMessage::Sleep,
                        }
                    },
                    gtk::FlowBoxChild {
                        set_focusable: false,

                        gtk::Button {
                            add_css_class: "btn",
                            set_icon_name: icon_names::ARROW_CIRCULAR_SMALL_BOTTOM_RIGHT,
                            connect_clicked => AppMessage::Reboot,
                        },
                    },
                    gtk::FlowBoxChild {
                        set_focusable: false,

                        gtk::Button {
                            add_css_class: "btn",
                            set_icon_name: icon_names::TURN_OFF,
                            connect_clicked => AppMessage::Shutdown,
                        },
                    },
                    gtk::FlowBoxChild {
                        set_focusable: false,

                        gtk::Button {
                            add_css_class: "btn",
                            set_icon_name: icon_names::ARROW_INTO_BOX,
                            connect_clicked => AppMessage::Logout,
                        }
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dead_internat_video = DeadInternet::builder().launch(()).detach();
        let model = AppModel {
            dead_internet: dead_internat_video,
        };

        let dead_internet = model.dead_internet.widget();
        let widgets = view_output!();

        root.init_layer_shell();
        root.set_layer(Layer::Overlay);
        root.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            info!("key pressed {:?}", key);
            if key == Key::q || key == Key::Escape {
                sender.input(AppMessage::QuitApp);
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        root.add_controller(key_controller);
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMessage::QuitApp => {
                relm4::main_application().quit();
            }
            AppMessage::Lock => {
                info!("Lock Request");
            }
            AppMessage::Sleep => {
                info!("Sleep Request");
            }
            AppMessage::Reboot => {
                info!("Reboot Request");
            }
            AppMessage::Shutdown => {
                info!("Shutdown Request");
            }
            AppMessage::Logout => {
                info!("Logout request");
            }
        }
    }
}
