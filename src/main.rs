use appcore::UserNotification;
use viewkit::prelude::*;

struct BorderExample;

impl BorderExample {
    fn card(label: &'static str, border: BorderStyle) -> StackChild {
        Card::new()
            .shadow(ShadowStyle::None)
            .border(border)
            .content(
                Padding::symmetric(20.0, 16.0).content(
                    Text::new(label)
                        .font_size(14.0)
                        .line_height(20.0)
                        .weight(600),
                ),
            )
            .width(360.0)
    }
}

impl App for BorderExample {
    type Body = Box<dyn View + 'static>;

    fn new() -> Self {
        Self
    }

    fn window(&self) -> WindowOptions {
        WindowOptions::new("ViewKit Border Example")
            .size(720.0, 520.0)
            .resizable(true)
    }

    fn body(&self, _context: &ViewContext) -> Box<dyn View + 'static> {
        let send_notification = Button::new("Send Notification")
            .style(ButtonStyle::Standard)
            .on_click(move || {
                if let Err(error) =
                    UserNotification::new("org.mochios.viewkit-test", "Test Notification")
                        .body("This notification was sent from Test.app through AppCore.")
                        .deliver()
                {
                    eprintln!("failed to send test notification: {error}");
                }
            });

        Box::new(
            VStack::new()
                .gap(StackGap::Large)
                .alignment(StackAlignment::Center)
                .distribution(StackDistribution::Center)
                .child(Self::card(
                    "Default — Standard 1px",
                    BorderStyle::standard(1.0),
                ))
                .child(Self::card("Strong — 1px", BorderStyle::strong(1.0)))
                .child(Self::card(
                    "Accent — 1px",
                    BorderStyle::custom(Color::from_rgb_hex(0x5f6fff), 1.0),
                ))
                .child(Self::card(
                    "Accent — 2px",
                    BorderStyle::custom(Color::from_rgb_hex(0x5f6fff), 2.0),
                ))
                .child(Self::card("No border", BorderStyle::None))
                .child(send_notification),
        )
    }
}

fn main() -> Result<(), ViewKitError> {
    run::<BorderExample>()
}
