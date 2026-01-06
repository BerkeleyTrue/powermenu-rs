use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
    gtk::{self, gdk::Key, gio::prelude::ApplicationExt, glib, prelude::*},
};
use tracing::info;

pub struct AppModel {}

#[derive(Debug)]
pub enum AppMessage {
    QuitApp,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMessage;
    type Output = ();
    view! {
        gtk::Window {
            set_title: Some("PowerMenu"),
            set_default_width: 600,
            set_default_height: 300,

            connect_close_request[sender] => move |_| {
                info!("close request");
                sender.input(AppMessage::QuitApp);
                gtk::glib::Propagation::Stop
            },

            connect_is_active_notify[sender] => move |window| {
                if !window.is_active() {
                    info!("lost focus");
                    sender.input(AppMessage::QuitApp);
                }
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 12,

                gtk::FlowBox {
                    set_valign: gtk::Align::Start,
                    set_max_children_per_line: 5,
                    set_min_children_per_line: 2,
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_homogeneous: true,
                    set_row_spacing: 6,
                    set_column_spacing: 6,

                    gtk::Box {
                        gtk::Label {
                            set_label: "Lock",
                        }
                    },
                    gtk::Box {
                        gtk::Label {
                            set_label: "sleep",
                        }
                    },
                    gtk::Box {
                        gtk::Label {
                            set_label: "logout",
                        }
                    },
                    gtk::Box {
                        gtk::Label {
                            set_label: "shutdown",
                        }
                    },
                    gtk::Box {
                        gtk::Label {
                            set_label: "restart",
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};
        let widgets = view_output!();
        root.init_layer_shell();
        root.set_layer(Layer::Overlay);
        root.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            info!("key pressed {:?}", key);
            if key == Key::Q || key == Key::Escape {
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
        }
    }
}
