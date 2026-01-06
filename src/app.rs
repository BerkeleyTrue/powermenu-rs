use gtk4_layer_shell::{Layer, LayerShell};
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    gtk::{
        self,
        gdk::Key,
        gio::prelude::ApplicationExt,
        glib,
        prelude::{GtkWindowExt, OrientableExt, WidgetExt},
    },
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
            set_default_width: 300,
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
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Label {
                    set_label: "Power",
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
            if key == Key::Q {
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
