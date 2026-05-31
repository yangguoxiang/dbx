use gpui::*;

use crate::state::UiState;
use crate::theme::ColorTokens;

pub fn render_overlay(
    tokens: ColorTokens,
    state: &Entity<UiState>,
    cx: &mut App,
) -> impl IntoElement {
    let ui = state.read(cx);
    let show_ssh = ui.show_ssh_section;
    let show_ssl = ui.show_ssl_section;
    let _ = ui;

    div()
        .absolute().top_0().left_0()
        .size_full()
        .bg(hsla(0.0, 0.0, 0.0, 0.45))
        .flex().items_center().justify_center()
        // Close on backdrop click
        .on_mouse_down(MouseButton::Left, {
            let state = state.clone();
            let id = state.entity_id();
            move |_e, _w, cx| {
                state.update(cx, |ui, _cx| { ui.show_connection_dialog = false; });
                cx.notify(id);
            }
        })
        .child(
            // Dialog card — eat clicks to prevent backdrop close
            div()
                .flex().flex_col()
                .w(px(520.0))
                .bg(tokens.surface)
                .rounded(tokens.radius_lg)
                .border_1().border_color(tokens.border)
                .overflow_hidden()
                .on_mouse_down(MouseButton::Left, |_e, _w, _cx| {})
                .child(header(&tokens, state))
                .child(body(&tokens, show_ssh, show_ssl, state))
                .child(footer(&tokens, state))
        )
}

fn header(t: &ColorTokens, state: &Entity<UiState>) -> impl IntoElement {
    let t = t.clone();
    div()
        .flex().flex_row().items_center().justify_between()
        .px(px(24.0)).py(px(14.0))
        .bg(t.bg_secondary).border_b_1().border_color(t.border)
        .child(div().font_weight(FontWeight::SEMIBOLD).text_size(t.text_size_lg).child("New Connection"))
        .child(
            div().size(px(24.0)).flex().items_center().justify_center()
                .rounded(px(12.0)).text_size(px(14.0)).text_color(t.text_muted)
                .cursor_pointer().child("\u{2715}")
                .hover(|s| s.bg(t.bg_hover).text_color(t.text))
                .on_mouse_down(MouseButton::Left, {
                    let state = state.clone();
                    let id = state.entity_id();
                    move |_e, _w, cx| {
                        state.update(cx, |ui, _cx| { ui.show_connection_dialog = false; });
                        cx.notify(id);
                    }
                })
        )
}

fn body(t: &ColorTokens, show_ssh: bool, show_ssl: bool, state: &Entity<UiState>) -> impl IntoElement {
    let t = t.clone();
    let mut body = div().flex().flex_col().flex_1().overflow_hidden().px(px(24.0)).py(px(16.0)).gap(px(14.0))
        .child(sec(&t, "Basic"))
        .child(fld(&t, "Name", "My Connection"))
        .child(fld(&t, "Type", "MySQL"))
        .child(div().flex().flex_row().gap(px(12.0))
            .child(fld_w(&t, "Host", "localhost", 1.0))
            .child(fld_w(&t, "Port", "3306", 0.3)))
        .child(fld(&t, "Username", "root"))
        .child(fld(&t, "Password", "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}"))
        .child(fld(&t, "Database", ""))
        .child(toggle(&t, "SSH Tunnel", show_ssh, state, true))
        .child(fld(&t, "SSH Host", ""));

    if show_ssh {
        body = body.child(div().flex().flex_row().gap(px(12.0))
            .child(fld_w(&t, "SSH Port", "22", 0.3))
            .child(fld_w(&t, "SSH User", "", 0.7)));
    }

    body = body.child(toggle(&t, "SSL", show_ssl, state, false));
    body
}

fn footer(t: &ColorTokens, state: &Entity<UiState>) -> impl IntoElement {
    let t = t.clone();
    div().flex().flex_row().justify_end().gap(px(8.0))
        .px(px(24.0)).py(px(14.0))
        .bg(t.bg_secondary).border_t_1().border_color(t.border)
        .child(btn(&t, "Test", false, {
            let state = state.clone();
            let id = state.entity_id();
            move |_w, cx| {
                state.update(cx, |ui, _cx| { ui.status_message = "connection.testing".into(); });
                cx.notify(id);
            }
        }))
        .child(btn(&t, "Connect", true, {
            let state = state.clone();
            let id = state.entity_id();
            move |_w, cx| {
                state.update(cx, |ui, _cx| {
                    ui.connections.push("New Connection".into());
                    ui.connected_ids.push("New Connection".into());
                    ui.show_connection_dialog = false;
                    ui.status_message = "connection.connected".into();
                });
                cx.notify(id);
            }
        }))
}

fn btn(
    t: &ColorTokens,
    label: &str,
    primary: bool,
    handler: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let t = t.clone();
    let label = label.to_string();
    div()
        .px(px(16.0)).py(px(8.0)).rounded(t.radius_md)
        .cursor_pointer()
        .bg(if primary { t.accent } else { t.bg_tertiary })
        .text_color(if primary { hsla(1.0, 0.0, 1.0, 1.0) } else { t.text })
        .text_size(t.text_size_sm).font_weight(FontWeight::MEDIUM)
        .hover(|s| s.bg(if primary { t.accent_hover } else { t.bg_hover }))
        .on_mouse_down(MouseButton::Left, move |_e, window, cx| {
            handler(window, cx);
        })
        .child(label)
}

// --- Form helpers ---

fn sec(t: &ColorTokens, label: &str) -> impl IntoElement {
    let label = label.to_string();
    div().text_size(t.text_size_xs).text_color(t.text_muted)
        .font_weight(FontWeight::SEMIBOLD).child(label)
}

fn toggle(t: &ColorTokens, label: &str, active: bool, state: &Entity<UiState>, is_ssh: bool) -> impl IntoElement {
    let label = label.to_string();
    div().flex().flex_row().items_center().gap(px(8.0)).cursor_pointer()
        .child(div().size(px(16.0)).flex().items_center().justify_center()
            .text_size(px(10.0)).text_color(if active { t.accent } else { t.text_muted })
            .child(if active { "\u{2713}" } else { "\u{25CB}" }))
        .child(div().text_size(t.text_size_sm).text_color(t.text).child(label))
        .on_mouse_down(MouseButton::Left, {
            let state = state.clone();
            let id = state.entity_id();
            move |_e, _w, cx| {
                state.update(cx, |ui, _cx| {
                    if is_ssh {
                        ui.show_ssh_section = !ui.show_ssh_section;
                    } else {
                        ui.show_ssl_section = !ui.show_ssl_section;
                    }
                });
                cx.notify(id);
            }
        })
}

fn fld(t: &ColorTokens, label: &str, value: &str) -> impl IntoElement {
    let t = t.clone();
    let label = label.to_string();
    let value = value.to_string();
    let has = !value.is_empty();
    div().flex().flex_col().gap(px(4.0))
        .child(div().text_size(t.text_size_xs).text_color(t.text_muted).child(label))
        .child(div().px(px(10.0)).py(px(8.0))            .bg(t.bg_tertiary).rounded(t.radius_md).border_1().border_color(t.border)
            .text_size(t.text_size_sm)
            .text_color(if has { t.text } else { t.text_muted })
            .child(if has { value } else { String::new() }))
}

fn fld_w(t: &ColorTokens, label: &str, value: &str, _flex: f32) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    let has = !value.is_empty();
    div().flex().flex_col().gap(px(4.0))
        .child(div().text_size(t.text_size_xs).text_color(t.text_muted).child(label))
        .child(div().px(px(10.0)).py(px(8.0))            .bg(t.bg_tertiary).rounded(t.radius_md).border_1().border_color(t.border)
            .text_size(t.text_size_sm)
            .text_color(if has { t.text } else { t.text_muted })
            .child(if has { value } else { String::new() }))
}
