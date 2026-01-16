use iced::{
    Color, Element, Length::Fill, Subscription, Task, Theme, theme::Style, widget::container,
};
use iced_layershell::{
    reexport::{Anchor, KeyboardInteractivity, Layer},
    settings::LayerShellSettings,
    to_layer_message,
};

use crate::{
    app,
    palette::{PALETTE, REM},
};

pub const SIZE: (u32, u32) = (623, 390);

#[to_layer_message]
#[derive(Debug)]
enum Message {
    App(app::Message),
}

struct LayerApp {
    app: app::App,
}

impl LayerApp {
    fn new(init: app::Init) -> (Self, Task<Message>) {
        let (app, task) = app::App::new(init);
        (Self { app }, task.map(Message::App))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::App(message) => self.app.update(message).map(Message::App),
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(self.app.view().map(Message::App))
            .style(container::transparent)
            .height(Fill)
            .width(Fill)
            .padding(REM as f32)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        self.app.subscription().map(Message::App)
    }
}

pub fn start(init: app::Init) -> iced_layershell::Result {
    iced_layershell::application(
        move || LayerApp::new(init.clone()),
        || "Powermenu".to_string(),
        LayerApp::update,
        LayerApp::view,
    )
    .theme(Theme::custom("dead_internet", PALETTE))
    .subscription(LayerApp::subscription)
    .style(|_layer, theme| Style {
        background_color: Color::TRANSPARENT,
        text_color: theme.palette().text,
    })
    .layer_settings(LayerShellSettings {
        layer: Layer::Overlay,
        size: Some(SIZE).map(|(w, h)| (w + REM as u32, h + REM as u32)),
        anchor: Anchor::empty(),
        keyboard_interactivity: KeyboardInteractivity::OnDemand,
        ..Default::default()
    })
    .run()
}
