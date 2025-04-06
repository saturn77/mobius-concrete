use egui_mobius_reactive::*;
use masonry::app::{AppDriver, DriverCtx};
use masonry::core::*;
use masonry::kurbo::*;
use masonry::palette;
use masonry::peniko::Color;
use masonry::widgets::RootWidget;
use parley::layout::{Alignment, AlignmentOptions};
use parley::style::{FontFamily, FontStack, StyleProperty};
use smallvec::SmallVec;
use tracing::{Span, trace_span};
use vello::Scene;
use vello::peniko::{Fill, Image, ImageFormat};
use winit::window::Window;
use std::sync::Arc;

#[derive(Clone)]
struct Theme {
    text_color: Color,
    button_color: Color,
    background_color: Color,
}

fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::from(([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]))   
}

fn tokyo_night_storm() -> Theme {
    Theme {
        text_color: rgb8(192, 202, 245),
        button_color: rgb8(68, 71, 90),
        background_color: rgb8(40, 42, 54),
    }
}

fn tokyo_night_light() -> Theme {
    Theme {
        text_color: rgb8(40, 42, 54),
        button_color: rgb8(189, 147, 249),
        background_color: rgb8(248, 248, 242),
    }
}

struct AppState {
    use_light_theme: Dynamic<bool>,
    click_count: Dynamic<u32>,
    click_label: Derived<String>,
    theme: Dynamic<Theme>,
    signals: SignalRegistry,
}

fn build_state() -> Arc<AppState> {
    let mut signals = SignalRegistry::default();

    let use_light_theme = signals.register_named_signal("use_light_theme", Dynamic::new(false));
    let click_count = signals.register_named_signal("click_count", Dynamic::new(0u32));
    let click_label = signals.register_named_signal(
        "click_label",
        Derived::new_with([click_count.signal()], move |signals| {
            let count = signals[0].as_u32().unwrap();
            format!("Clicked {} times", count)
        }),
    );
    let theme = signals.register_named_signal("theme", Dynamic::new(tokyo_night_storm()));

    Arc::new(AppState {
        use_light_theme,
        click_count,
        click_label,
        theme,
        signals,
    })
}

struct Driver {
    state: Arc<AppState>,
}

impl AppDriver for Driver {
    type AppData = ();

    fn root_widget(&self) -> Box<dyn Widget<Self::AppData>> {
        Box::new(ui(&self.state))
    }
}

fn ui(state: &Arc<AppState>) -> impl Widget<()> {
    let theme_toggle = Switch::new()
        .label("Use Tokyo Night Light Theme")
        .on_toggle({
            let use_light_theme = state.use_light_theme.clone();
            let theme = state.theme.clone();
            move |_ctx, is_light| {
                use_light_theme.set(is_light);
                theme.set(if is_light { tokyo_night_light() } else { tokyo_night_storm() });
            }
        });

    let button = Button::new("Click Me!")
        .on_click({
            let click_count = state.click_count.clone();
            move |_ctx, _| {
                click_count.modify(|v| *v += 1);
            }
        });

    let label = Label::dynamic({
        let click_label = state.click_label.signal();
        move |_data: &(), _env| click_label.get()
    });

    Flex::column()
        .with_child(theme_toggle)
        .with_spacer(10.0)
        .with_child(button)
        .with_spacer(10.0)
        .with_child(label)
        .padding(20.0)
}

fn main() {
    let state = build_state();
    let mut app = App::new();
    app.set_driver(Driver { state });
    app.run();
}
