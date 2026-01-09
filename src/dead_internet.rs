use gtk4::gdk::{Paintable, Texture};
use relm4::{
    gtk::{self, gio, glib, prelude::*},
    prelude::*,
};
use tracing::error;

// embeds resource
pub const GRESOURCE_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource"));

// resource id
pub const RESOURCE_PREFIX: &str = "/com/bt/powermenu-rs";

pub fn init_resources() {
    // wraps the bytes?
    let gbytes = glib::Bytes::from_static(GRESOURCE_BYTES);
    // parses gresource format
    let resource =
        gio::Resource::from_data(&gbytes).expect("Failed to load dead_internet gresouce");
    // register resource globally
    gio::resources_register(&resource);
}

pub struct DeadInternet {
    poster: Texture,
    media: Option<gtk::MediaFile>,
}

impl DeadInternet {
    fn load_media(&mut self) {
        let file = gio::File::for_uri(&format!(
            "resource://{RESOURCE_PREFIX}/redlotoo_dead-internet.mp4"
        ));

        let media = gtk::MediaFile::for_file(&file);

        // Debug: check for errors
        media.connect_error_notify(|media| {
            if let Some(error) = media.error() {
                error!("MediaFile error: {:?}", error);
            }
        });
        media.set_loop(true);
        media.play();
        self.media = Some(media);
    }
}

#[derive(Debug)]
pub enum Messages {
    LoadVideo,
}

#[relm4::component(pub)]
impl SimpleComponent for DeadInternet {
    type Init = ();
    type Input = Messages;
    type Output = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let poster =
            Texture::from_resource(&format!("{RESOURCE_PREFIX}/redlotoo_dead-internet.png"));
        let model = DeadInternet {
            poster,
            media: None,
        };

        let widgets = view_output!();

        root.connect_realize(move |_| {
            sender.input(Messages::LoadVideo);
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Messages::LoadVideo => {
                self.load_media();
            }
        }
    }

    view! {
        gtk::Picture {
            add_css_class: "gif",
            #[watch]
            set_paintable: model.media.as_ref()
                .map(|m| m.upcast_ref::<Paintable>())
                .or(Some(model.poster.upcast_ref::<Paintable>())),
        }
    }
}
