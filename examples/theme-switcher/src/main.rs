use egui_mobius_reactive::*;
use masonry::app::{AppDriver, DriverCtx};
use masonry::core::{Action, WidgetId};
use masonry::dpi::LogicalSize;
use masonry::peniko::Color;
use masonry::widgets::{RootWidget, Button, Label, Flex};
use winit::window::Window;

use std::sync::Arc;

#[derive(Clone)]
struct Theme {
    text_color: Color,
    button_color: Color,
    background_color: Color,
}

impl ReactiveValue for Theme {
    fn subscribe(&self, _subscriber: Box<dyn Fn() + Send + Sync>) {
        // Theme is immutable once created, no need to notify subscribers
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn tokyo_night_storm() -> Theme {
    Theme {
        text_color: Color::from_rgb8(192, 202, 245),
        button_color: Color::from_rgb8(68, 71, 90),
        background_color: Color::from_rgb8(40, 42, 54),
    }
}

fn tokyo_night_light() -> Theme {
    Theme {
        text_color: Color::from_rgb8(40, 42, 54),
        button_color: Color::from_rgb8(189, 147, 249),
        background_color: Color::from_rgb8(248, 248, 242),
    }
}

struct AppState {
    use_light_theme: Dynamic<bool>,
    click_count: Dynamic<u32>,
    click_label: Derived<String>,
    theme: Dynamic<Theme>,
}

impl AppState {
    fn new() -> Arc<Self> {
        let use_light_theme = Dynamic::new(false);
        let click_count = Dynamic::new(0u32);
        let click_count_arc = Arc::new(click_count.clone());
        let click_label = Derived::new(&[click_count_arc.clone()], move || {
            let count = click_count_arc.get();
            format!("Clicked {} times", count)
        });
        let theme = Dynamic::new(tokyo_night_storm());

        Arc::new(Self {
            use_light_theme,
            click_count,
            click_label,
            theme,
        })
    }
}



struct Driver {
    state: Arc<AppState>,
}

impl AppDriver for Driver {
    fn on_action(&mut self, ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(click_button) => {
                let count = self.state.click_count.get() + 1;
                self.state.click_count.set(count);

                //let theme_state = !self.state.use_light_theme.get();
                //self.state.use_light_theme.set(theme_state);
                //self.state.theme.set(if theme_state { tokyo_night_light() } else { tokyo_night_storm() });

                ctx.render_root().edit_root_widget(|mut root| {
                    let mut root = root.downcast::<RootWidget<Flex>>();
                    let mut flex = RootWidget::child_mut(&mut root);
                    // Update the label text
                    let label_text = self.state.click_label.get().to_string();
                    Flex::clear(&mut flex);
                    Flex::add_child(&mut flex, Button::new(if self.state.use_light_theme.get() { "Switch to Dark Theme" } else { "Switch to Light Theme" }));
                    Flex::add_spacer(&mut flex, 10.0);
                    Flex::add_child(&mut flex, Button::new("Click Me!"));
                    Flex::add_spacer(&mut flex, 10.0);
                    Flex::add_child(&mut flex, Label::new(label_text));
                    Flex::add_spacer(&mut flex, 20.0);
                });
            }
            action => {
                eprintln!("Unexpected action {action:?}");
            }
        }
    }
}


fn main() {
    let state = AppState::new();
    let app = masonry::app::EventLoop::with_user_event();


    let theme_toggle = Button::new(if state.use_light_theme.get() { "Switch to Dark Theme" } else { "Switch to Light Theme" });

    let click_button = Button::new("Click Me!");

    let label = Label::new(state.click_label.get().to_string());

    let main_widget = Flex::column()
        .with_child(theme_toggle)
        .with_spacer(10.0)
        .with_child(click_button)
        .with_spacer(10.0)
        .with_child(label)
        .with_spacer(20.0);


    let window_size = LogicalSize::new(400.0, 400.0);
    let window_attributes = Window::default_attributes()
        .with_title("Theme Switcher")
        .with_resizable(true)
        .with_min_inner_size(window_size);

    masonry::app::run(
        app,
        window_attributes,
        RootWidget::new(main_widget),
        Driver { state },
    ).unwrap();
}
