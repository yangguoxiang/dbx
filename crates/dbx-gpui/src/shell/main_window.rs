use gpui::*;

use crate::state::UiState;
use crate::theme::ColorTokens;
use super::{toolbar, sidebar, tab_bar, status_bar};

pub fn render_main_window(state: &Entity<UiState>, cx: &mut App) -> impl IntoElement {
    let ui = state.read(cx);
    let tokens = ui.tokens.clone();
    let has_tabs = !ui.tabs.is_empty();
    let sidebar_on = ui.sidebar_visible;
    let empty = ui.t("sidebar.no_connections").to_string();
    let hdr = ui.t("sidebar.connections").to_string();
    let status_msg = ui.t(&ui.status_message).to_string();
    let conn_count = ui.connected_ids.len();
    let tab_data: Vec<(String, String)> = ui.tabs.iter().map(|t| (t.id.clone(), t.title.clone())).collect();
    let active_tab = ui.active_tab_id.clone();
    let tagline = ui.t("app.tagline").to_string();
    let _ = ui;

    let mut el = div().flex().flex_row().size_full()
        .bg(tokens.bg).text_color(tokens.text).font_family(tokens.font_ui.clone());

    if sidebar_on {
        el = el.child(sidebar::render_sidebar(&tokens, &[], &[], &empty, &hdr));
    }

    let mut center = div().flex().flex_col().flex_1().overflow_hidden()
        .child(toolbar::render_toolbar(&tokens));

    if has_tabs {
        center = center.child(tab_bar::render_tab_bar(&tokens, &tab_data, active_tab.as_deref()));
    }

    if !has_tabs {
        center = center.child(
            div().flex_1().flex().items_center().justify_center()
                .child(welcome(&tokens, &tagline))
        );
    } else {
        center = center.child(div().flex_1());
    }

    center = center.child(status_bar::render_status_bar(&tokens, &status_msg, conn_count));
    el = el.child(center);
    el
}

fn welcome(t: &ColorTokens, tagline: &str) -> impl IntoElement {
    let t = t.clone();
    let tagline = tagline.to_string();
    div().flex().flex_col().items_center().gap(px(16.0))
        .child(div().text_size(px(32.0)).font_weight(FontWeight::BOLD).text_color(t.text).child("DBX"))
        .child(div().text_size(t.text_size_lg).text_color(t.text_secondary).child(tagline))
        .child(div().text_size(t.text_size_sm).text_color(t.text_muted).child("Ctrl+N New Query  |  Ctrl+Shift+C New Connection"))
}
