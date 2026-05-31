use gpui::*;

use crate::actions::*;
use crate::theme::ColorTokens;

pub fn render_toolbar(tokens: &ColorTokens) -> impl IntoElement {
    let t = tokens.clone();
    div()
        .flex().flex_row().items_center()
        .h(px(44.0))
        .bg(t.bg_secondary).border_b_1().border_color(t.border)
        .px(px(8.0)).gap(px(4.0))
        .child(
            div().flex().flex_row().gap(px(2.0))
                .child(tb_btn(&t, "+ Query", |_w, cx| { cx.dispatch_action(&NewQueryTab); }))
                .child(tb_btn(&t, "+ Conn", |_w, cx| { cx.dispatch_action(&OpenConnectionDialog); }))
        )
        .child(div().flex_1())
        .child(
            div().flex().flex_row().gap(px(2.0))
                .child(tb_btn(&t, "AI", |_w, cx| { cx.dispatch_action(&ToggleAiPanel); }))
                .child(tb_btn(&t, "Hist", |_w, cx| { cx.dispatch_action(&ToggleHistoryPanel); }))
                .child(tb_btn(&t, "Theme", |_w, cx| { cx.dispatch_action(&ToggleTheme); }))
        )
}

fn tb_btn(
    t: &ColorTokens,
    label: &str,
    handler: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let t = t.clone();
    let label = label.to_string();
    div()
        .px(px(8.0)).py(px(4.0)).rounded(t.radius_md)
        .text_color(t.text_secondary).text_size(t.text_size_sm)
        .font_family(t.font_ui.clone())
        .cursor_pointer()
        .hover(|style| style.bg(t.bg_hover).text_color(t.text))
        .on_mouse_down(MouseButton::Left, move |_e, window, cx| {
            handler(window, cx);
        })
        .child(label)
}
