use iced::{
    Element,
    widget::{button, text},
};

pub struct PowerButton<T> {
    pub icon: String,
    pub message: T,
}

impl<T: Clone + 'static> PowerButton<T> {
    pub fn view(&self) -> Element<'static, T> {
        button(text(self.icon.clone()))
            .on_press(self.message.clone())
            .into()
    }
}
