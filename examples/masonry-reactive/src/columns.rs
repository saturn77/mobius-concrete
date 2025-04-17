// TwoColumnLayout.rs implemented as a full Masonry widget with accessibility + reactive label

use masonry::core::WidgetPod;
use masonry::core::{
    AccessCtx, AccessEvent, BoxConstraints, ComposeCtx, EventCtx, LayoutCtx, PaintCtx,
    PointerEvent, PropertiesMut, PropertiesRef, RegisterCtx, TextEvent, Update, UpdateCtx, Widget,
    WidgetId,
};
use masonry::kurbo::{Point, Size};
use masonry::vello::Scene;
use masonry::widgets::{Flex, Label};
use smallvec::{SmallVec, smallvec};
use egui_mobius_reactive::Derived;
use masonry::parley::Style;  

pub struct TwoColumnLayout {
    left_column: WidgetPod<Flex>,
    right_column: WidgetPod<Flex>,
    spacing: f64,
}

impl TwoColumnLayout {
    pub fn new(left: Flex, right: Flex, spacing: f64) -> Self {
        Self {
            left_column: WidgetPod::new(left),
            right_column: WidgetPod::new(right),
            spacing,
        }
    }
}

impl Widget for TwoColumnLayout {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _props: &mut PropertiesMut<'_>, _event: &PointerEvent) {}
    fn on_text_event(&mut self, _ctx: &mut EventCtx, _props: &mut PropertiesMut<'_>, _event: &TextEvent) {}
    fn on_access_event(&mut self, _ctx: &mut EventCtx, _props: &mut PropertiesMut<'_>, _event: &AccessEvent) {}
    fn on_anim_frame(&mut self, _ctx: &mut UpdateCtx, _props: &mut PropertiesMut<'_>, _interval: u64) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _props: &mut PropertiesMut<'_>, _event: &Update) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, _props: &mut PropertiesMut<'_>, bc: &BoxConstraints) -> Size {
        let total_width = bc.max().width.max(self.spacing);
        let column_width = ((total_width - self.spacing) / 2.0).max(0.0);

        let left_constraints = BoxConstraints::new(Size::ZERO, Size::new(column_width, bc.max().height));
        let left_size = ctx.run_layout(&mut self.left_column, &left_constraints);
        ctx.place_child(&mut self.left_column, Point::ORIGIN);

        let right_constraints = BoxConstraints::new(Size::ZERO, Size::new(column_width, bc.max().height));
        let right_size = ctx.run_layout(&mut self.right_column, &right_constraints);
        ctx.place_child(&mut self.right_column, Point::new(column_width + self.spacing, 0.0));

        Size::new(total_width, left_size.height.max(right_size.height))
    }

    fn compose(&mut self, _ctx: &mut ComposeCtx) {}

    fn register_children(&mut self, ctx: &mut RegisterCtx) {
        ctx.register_child(&mut self.left_column);
        ctx.register_child(&mut self.right_column);
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.left_column.id(), self.right_column.id()]
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _props: &PropertiesRef<'_>, _scene: &mut Scene) {}

    fn accessibility_role(&self) -> accesskit::Role {
        accesskit::Role::Group
    }

    fn accessibility(&mut self, _ctx: &mut AccessCtx, _props: &PropertiesRef<'_>, node: &mut accesskit::Node) {
        node.set_role(accesskit::Role::Group);
        node.set_class_name("Two Column Layout Container");
    }
}


// --- MARK: TESTS ---
#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use masonry::assert_render_snapshot;
    use masonry::testing::TestHarness;
    use masonry::widgets::{Flex, Label};

    #[test]
    fn test_two_column_layout() {
        let left = Flex::column().with_child(Label::new("Left"));
        let right = Flex::column().with_child(Label::new("Right"));
        let widget = TwoColumnLayout::new(left, right, 10.0);

        let mut harness = TestHarness::create(widget);
        assert_debug_snapshot!(harness.root_widget());
        assert_render_snapshot!(harness, "two_column_layout");
    }
}
