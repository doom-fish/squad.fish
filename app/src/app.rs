use gpui::*;
use theme::active_theme::ActiveThemeTrait;
pub struct SquadFish {}

impl Render for SquadFish {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(cx.theme().colors.background_color)
            .size_full()
            .justify_center()
            .items_center()
            .text_size(rems(5.))
            .text_color(cx.theme().colors.text_color)
            .child("SQUAD FISH")
    }
}
