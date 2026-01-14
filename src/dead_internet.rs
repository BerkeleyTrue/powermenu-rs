use iced::{Element, Subscription, Task, advanced::image, time};

use crate::atlas::AtlasFrame;

const DEAD_INTERNET_BYTES: &[u8] = include_bytes!("../resources/redlotoo_dead-internet-atlas.png");
const FRAMES: u32 = 30;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

pub struct DeadInternet {
    handle: image::Handle,
    index: u32,
}

impl DeadInternet {
    pub fn new() -> Self {
        Self {
            handle: image::Handle::from_bytes(DEAD_INTERNET_BYTES),
            index: 0,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.index = (self.index + 1) % FRAMES;
            }
        }
        Task::none()
    }

    pub fn subscriptions(&self) -> Subscription<Message> {
        time::every(time::Duration::from_millis(33)).map(|_| Message::Tick)
    }

    pub fn view(&self) -> Element<'_, Message> {
        AtlasFrame::new(self.handle.clone(), 8, 910.0, 512.0, self.index).into()
    }
}
