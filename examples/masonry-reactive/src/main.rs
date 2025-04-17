//! masonry with egui_mobius integration
//!
//! Demonstrates how to integrate egui_mobius_reactive with another
//! GUI framework other than egui/eframe.
//!
// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

// masonry imports
use masonry::app::{AppDriver, DriverCtx};
use masonry::core::{Action, StyleProperty, WidgetId};
use masonry::dpi::LogicalSize;
use masonry::parley::style::FontWeight;
use masonry::widgets::{Button, Flex, Label, RootWidget};
use winit::window::Window;

// egui_mobius_reactive imports using prelude import
use egui_mobius_reactive::*;
use std::sync::Arc;

// other imports for this crate
mod columns;
    use columns::TwoColumnLayout;

const VERTICAL_WIDGET_SPACING: f64 = 40.0;

pub struct AppState {
    click_count : Dynamic<i32>,
    formatted   : Derived<String>,
    doubled     : Derived<i32>,
    sum         : Derived<i32>,
    message     : Dynamic<String>,
}

impl AppState {
    pub fn new() -> Self {
        let click_count = Dynamic::new(0);
        let click_count_arc = Arc::new(click_count.clone());
        let doubled = click_count.doubled();

        let formatted : Derived<String> = Derived::new(&[click_count_arc.clone()], move || {
            let val = *click_count_arc.lock();
            format!("Count: {}", val)
        });

        let sum = click_count.clone() + doubled.clone();
        let message = Dynamic::new("Click the button!".to_string());

        let registry = SignalRegistry::default();
        registry.register_named_signal("click_count", Arc::new(click_count.clone()));
        registry.register_named_signal("doubled", Arc::new(doubled.clone()));
        registry.register_named_signal("sum", Arc::new(sum.clone()));
        registry.register_named_signal("message", Arc::new(message.clone()));

        Self {
            click_count,
            formatted, 
            doubled,
            sum, 
            message,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

struct Driver {
    state: Arc<AppState>,
}

impl AppDriver for Driver {
    fn on_action(&mut self, ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) => {
                // Update the state 
                let count = self.state.click_count.get() + 1;
                self.state.click_count.set(count);
                
                // Small delay to let the reactive system update
                std::thread::sleep(std::time::Duration::from_micros(200));
                
                // Get updated values - note use of ReactiveMath to get the derived values
                let doubled = self.state.doubled.get();
                let sum = self.state.sum.get();
                
                println!("on action : Count: {}, Doubled: {}, Sum: {}", count, doubled, sum);
                self.state.message.set(format!("Count: {}", count));
                
                // Request a full update - let the rebuilt TwoColumnLayout handle the layout

            }
            action => {
                eprintln!("Unexpected action {action:?}");
            }
        }
    }
}

// In your main.rs, modify the following sections:
// Import Container widget if needed
// Removed unused import as `Container` does not exist in `masonry::widgets`
fn main() {
    let state = Arc::new(AppState::new());

    let state_count_clone = state.click_count.clone();
    let state_count_arc : Arc<Dynamic<i32>> = Arc::new(state_count_clone.clone());


    
    // Create left column
    let left_column = Flex::column()
        .with_child(Label::new(state.message.get())
            .with_style(StyleProperty::FontSize(24.0))
            .with_style(StyleProperty::FontWeight(FontWeight::BOLD)))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Label::new("Left column info")
            .with_style(StyleProperty::FontSize(16.0)))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Button::new("Click me!"));
    
    // Create right column
    let right_column = Flex::column()
        .with_child(Label::new("Right column stats")
            .with_style(StyleProperty::FontSize(16.0)))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Label::new(state.formatted.clone().get())
            .with_style(StyleProperty::FontSize(20.0)))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Label::new(format!("Sum: {}", state.sum.get()))
            .with_style(StyleProperty::FontSize(20.0)));
    
    // Create two-column layout using our custom container
    let main_widget = TwoColumnLayout::new(left_column, right_column, 20.0);
    
    // Window setup
    let window_size = LogicalSize::new(600.0, 400.0);
    let window_attributes = Window::default_attributes()
        .with_title("Reactive Two-Column Layout")
        .with_resizable(true)
        .with_min_inner_size(window_size);
    
    // Run application
    masonry::app::run(
        masonry::app::EventLoop::with_user_event(),
        window_attributes,
        RootWidget::new(main_widget),
        Driver {
            state: state.clone(),
        },
    )
    .unwrap();
}