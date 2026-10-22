use appcore::{UserNotification, document};
use viewkit::prelude::*;

struct BorderExample {
    scroll: ScrollState,
    input: TextFieldInteractionState,
    buttons: [ButtonInteractionState; 9],
    reference_input: TextFieldInteractionState,
    toggle: State<bool>,
    slider: State<f32>,
    segment: State<usize>,
    toggle_interaction: SwitchInteractionState,
    slider_interaction: SliderInteractionState,
    segment_interaction: SegmentedControlInteractionState,
}

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
        for (index, title) in ["Status", "Notifications", "Session", "Network"]
            .iter()
            .enumerate()
        {
            if let Err(error) = appcore::ControlCenterItem::register(
                "org.mochios.viewkit-test",
                format!("example-{}", index + 1),
            )
            .card(
                appcore::ControlCenterCard::new(format!("Example {title}"))
                    .row("Provider", "ViewKit Test / AppCore")
                    .row("Registration", "Active")
                    .row("Purpose", "Control Center example"),
            )
            .publish()
            {
                eprintln!("could not publish example control: {error}");
            }
        }
        Self {
            scroll: ScrollState::new(),
            toggle: State::new(false),
            slider: State::new(0.5),
            segment: State::new(0),
            toggle_interaction: SwitchInteractionState::new(),
            slider_interaction: SliderInteractionState::new(),
            segment_interaction: SegmentedControlInteractionState::new(),
            input: TextFieldInteractionState::new(),
            reference_input: TextFieldInteractionState::new(),
            buttons: std::array::from_fn(|_| ButtonInteractionState::new()),
        }
    }

    fn window(&self) -> WindowOptions {
        WindowOptions::new("omochi — ViewKit Test")
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

        let controls = VStack::new()
            .gap(StackGap::Medium)
            .alignment(StackAlignment::Stretch)
            .child(Text::styled("omochi", TextRole::TitleLarge))
            .child(Text::caption(
                "Press, hold, pull, and release to compare omochi surfaces. Use Tab for keyboard input.",
            ))
            .child(
                HStack::new()
                    .gap(StackGap::Small)
                    .child(Button::with_interaction_and_label(
                        self.buttons[0].clone(),
                        "Standard",
                    ))
                    .child(
                        Button::with_interaction_and_label(self.buttons[1].clone(), "Accent")
                            .style(ButtonStyle::Accent),
                    )
                    .child(
                        Button::with_interaction_and_label(self.buttons[2].clone(), "Danger")
                            .style(ButtonStyle::Danger),
                    )
                    .child(
                        Button::with_interaction_and_label(self.buttons[3].clone(), "Disabled")
                            .enabled(false),
                    ),
            )
            .child(Text::caption("Moving surfaces — switch / slider / selection"))
            .child(HStack::new().alignment(StackAlignment::Center).gap(StackGap::Large)
                .child(Switch::with_interaction(self.toggle.binding(), self.toggle_interaction.clone()).label("Switch"))
                .child(Slider::with_interaction(self.slider.binding(), self.slider_interaction.clone()).label("Slider")
                    .width(240.0)))
            .child(SegmentedControl::with_interaction(self.segment.binding(), self.segment_interaction.clone())
                .accessibility_label("omochi selection")
                .item(0, "First").item(1, "Second").disabled_item(2, "Disabled").item(3, "Third"))
            .child(Text::caption("Common sizes — Small / Standard / Large"))
            .child(
                HStack::new()
                    .gap(StackGap::Small)
                    .alignment(StackAlignment::Center)
                    .child(
                        Button::with_interaction_and_label(self.buttons[4].clone(), "Button")
                            .size(ButtonSize::Small),
                    )
                    .child(Button::with_interaction_and_label(
                        self.buttons[5].clone(),
                        "Button",
                    ))
                    .child(
                        Button::with_interaction_and_label(self.buttons[6].clone(), "Button")
                            .size(ButtonSize::Large),
                    ),
            )
            .child(Text::caption(
                "Surface response — omochi / static reference",
            ))
            .child(
                HStack::new()
                    .gap(StackGap::Small)
                    .alignment(StackAlignment::Center)
                    .child(Button::with_interaction_and_label(
                        self.buttons[7].clone(),
                        "Button",
                    ))
                    .child(
                        Button::with_interaction_and_label(self.buttons[8].clone(), "Button")
                            .omochi(false),
                    ),
            )
            .child(
                HStack::new()
                    .gap(StackGap::Small)
                    .child(
                        TextField::with_interaction(self.input.clone())
                            .placeholder("Text input / IME")
                            .layout()
                            .flex_grow(1.0),
                    )
                    .child(
                        TextField::with_interaction(self.reference_input.clone())
                            .placeholder("Reference / IME")
                            .layout()
                            .flex_grow(1.0),
                    ),
            )
            .child(
                TextField::with_interaction(TextFieldInteractionState::new())
                    .placeholder("Invalid input")
                    .invalid(true),
            )
            .child(
                TextField::with_interaction(TextFieldInteractionState::new())
                    .placeholder("Disabled input")
                    .enabled(false),
            )
            .child(
                Menu::new()
                    .item(MenuItem::new("Standard menu item"))
                    .item(MenuItem::new("Disabled menu item").enabled(false))
                    .group()
                    .item(MenuItem::new("Destructive action").danger(true)),
            );
        Box::new(
            Scroll::new(self.scroll.clone()).content(
                Padding::all(24.0).content(
                    VStack::new()
                        .gap(StackGap::Large)
                        .alignment(StackAlignment::Center)
                        .distribution(StackDistribution::Center)
                        .child(controls.layout().width(560.0))
                        .child(Self::card(
                            "Default — Standard 1px",
                            BorderStyle::standard(1.0),
                        ))
                        .child(Self::card("Strong — 1px", BorderStyle::strong(1.0)))
                        .child(Self::card(
                            "Accent — 1px",
                            BorderStyle::custom(Theme::current().colors.accent, 1.0),
                        ))
                        .child(Self::card(
                            "Accent — 2px",
                            BorderStyle::custom(Theme::current().colors.accent, 2.0),
                        ))
                        .child(Self::card("No border", BorderStyle::None))
                        .child(send_notification),
                ),
            ),
        )
    }
}

fn main() -> Result<(), ViewKitError> {
    if let Some(path) = std::env::args().nth(1) {
        match document::open(&path, "image/png", document::AssociationRoles::VIEW) {
            Ok(_) => return Ok(()),
            Err(error) => {
                eprintln!("Test.app: could not open {path}: {error}");
                std::process::exit(1);
            }
        }
    }
    run::<BorderExample>()
}
