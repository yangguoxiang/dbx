use gpui::*;

use crate::theme::ColorTokens;

pub fn render_status_bar(tokens: &ColorTokens, message: &str, count: usize) -> impl IntoElement {
    let t = tokens.clone();
    let msg = message.to_string();

    div()
        .flex().flex_row().items_center().justify_between()
        .h(px(26.0))
        .bg(t.bg_secondary).border_t_1().border_color(t.border)
        .px(px(12.0))
        .text_color(t.text_muted).text_size(px(11.0)).font_family(t.font_ui.clone())
        .child(msg)
        .child(format!("{} connected", count))
}
