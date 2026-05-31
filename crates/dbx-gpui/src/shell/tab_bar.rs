use gpui::*;

use crate::actions::CloseTab;
use crate::theme::ColorTokens;

pub fn render_tab_bar(
    tokens: &ColorTokens,
    tabs: &[(String, String)],
    active_id: Option<&str>,
) -> impl IntoElement {
    if tabs.is_empty() {
        return div().into_any_element();
    }
    let t = tokens.clone();
    let tabs_v = tabs.to_vec();
    let active = active_id.map(|s| s.to_string());

    div().flex().flex_row().items_end()
        .h(px(36.0))
        .bg(t.bg).border_b_1().border_color(t.border).overflow_hidden()
        .child(
            div().flex().flex_row()
                .children(tabs_v.iter().map(|(tid, title)| {
                    let is_active = Some(tid) == active.as_ref();
                    tab_item(&t, title, is_active)
                }))
        )
        .into_any()
}

fn tab_item(tokens: &ColorTokens, title: &str, is_active: bool) -> impl IntoElement {
    let t = tokens.clone();
    let title = title.to_string();
    div().flex().flex_row().items_center().cursor_pointer()
        .px(px(12.0)).pr(px(4.0)).gap(px(4.0))
        .bg(if is_active { t.bg } else { t.bg_secondary })
        .border_r_1().border_color(t.border)
        .text_color(if is_active { t.text } else { t.text_muted })
        .text_size(t.text_size_sm).font_family(t.font_ui.clone())
        .child(title)
        .child(
            div().size(px(16.0)).flex().items_center().justify_center()
                .rounded(px(2.0)).text_size(px(10.0)).text_color(t.text_muted)
                .cursor_pointer().child("\u{2715}")
                .hover(|s| s.bg(t.bg_hover).text_color(t.error))
                .on_mouse_down(MouseButton::Left, move |_e, _w, cx| {
                    cx.dispatch_action(&CloseTab);
                })
        )
}
