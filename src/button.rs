use iced::{
    Border, Element,
    Length::Fill,
    Radians, Theme,
    advanced::widget::text,
    gradient::Linear,
    widget::{Button, button, container},
};
use iced_font_awesome::fa_icon_solid;

pub enum Icon {
    Lock,
    Sleep,
    Shutdown,
    Reboot,
    Logout,
}

pub struct PowerButton<T> {
    pub icon: Icon,
    pub message: T,
}

impl<T: Clone + 'static> PowerButton<T> {
    pub fn view(&self) -> Element<'static, T> {
        let icon = (match self.icon {
            Icon::Lock => fa_icon_solid("lock"),
            Icon::Sleep => fa_icon_solid("moon"),
            Icon::Reboot => fa_icon_solid("rotate"),
            Icon::Shutdown => fa_icon_solid("power-off"),
            Icon::Logout => fa_icon_solid("arrow-right-to-bracket"),
        })
        .style(|theme: &Theme| {
            let palette = theme.palette();
            text::Style {
                color: Some(palette.background),
            }
        })
        .size(15.0);

        let button = Button::new(container(icon).center(Fill))
            .style(|theme: &Theme, _| {
                let palette = theme.palette();

                button::Style {
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
            })
            .padding(10)
            .width(Fill)
            .height(40)
            .on_press(self.message.clone());

        let button_wrapper = container(button)
            .style(|theme: &Theme| {
                let palette = theme.palette();

                container::Style {
                    border: Border::default()
                        .rounded(1.0)
                        .color(palette.text)
                        .width(3.0),
                    ..Default::default()
                }
            })
            .padding(1);

        container(button_wrapper).center_x(Fill).into()
    }
}
