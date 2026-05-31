use gpui::*;

use crate::actions::*;
use crate::theme::ColorTokens;

pub fn render_sidebar(
    tokens: &ColorTokens,
    connections: &[String],
    connected_ids: &[String],
    empty_text: &str,
    header_text: &str,
) -> impl IntoElement {
    let t = tokens.clone();
    let empty = empty_text.to_string();
    let hdr = header_text.to_string();

    div().flex().flex_col()
        .w(px(260.0))
        .bg(t.bg_secondary).border_r_1().border_color(t.border)
        .child(
            div().flex().flex_row().items_center().justify_between()
                .px(px(12.0)).py(px(8.0))
                .child(div().text_color(t.text_secondary).text_size(t.text_size_xs)
                    .font_family(t.font_ui.clone()).font_weight(FontWeight::MEDIUM).child(hdr))
                .child(
                    div().text_color(t.text_muted).text_size(px(12.0)).cursor_pointer().child("+")
                        .hover(|s| s.text_color(t.text))
                        .on_mouse_down(MouseButton::Left, move |_e, _w, cx| {
                            cx.dispatch_action(&OpenConnectionDialog);
                        })
                )
        )
        .child(
            if connections.is_empty() {
                div().flex_1().overflow_hidden()
                    .flex().items_center().justify_center()
                    .child(div().text_color(t.text_muted).text_size(t.text_size_sm).child(empty))
            } else {
                div().flex_1().overflow_hidden()
                    .flex().flex_col().children(connections.iter().map(|name| {
                        let connected = connected_ids.contains(name);
                        conn_item(&t, name, connected)
                    }))
            }
        )
}

fn conn_item(t: &ColorTokens, name: &str, connected: bool) -> impl IntoElement {
    let t = t.clone();
    let name = name.to_string();
    div().flex().flex_row().items_center().cursor_pointer()
        .px(px(8.0)).py(px(6.0)).gap(px(8.0))
        .rounded(t.radius_sm)
        .hover(|s| s.bg(t.bg_hover))
        .child(div().size(px(8.0)).rounded(px(4.0))
            .bg(if connected { t.success } else { t.text_muted }))
        .child(div().flex_1()
            .text_color(if connected { t.text } else { t.text_secondary })
            .text_size(t.text_size_sm).child(name))
}
