use gpui::*;

use crate::actions::*;
use crate::connection::connection_dialog;
use crate::shell::main_window::render_main_window;
use crate::state::{QueryTab, UiState};

pub struct AppView {
    state: Entity<UiState>,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let state = cx.new(|_cx| UiState::new());
        AppView { state }
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let show_dialog = state.show_connection_dialog;
        let tokens = state.tokens.clone();
        let _ = state;

        let mut root = div()
            .size_full()
            .relative()
            .bg(tokens.bg)
            .text_color(tokens.text)
            .font_family(tokens.font_ui.clone())
            .on_action(cx.listener(Self::on_new_query))
            .on_action(cx.listener(Self::on_open_conn))
            .on_action(cx.listener(Self::on_toggle_ai))
            .on_action(cx.listener(Self::on_toggle_history))
            .on_action(cx.listener(Self::on_toggle_sidebar))
            .on_action(cx.listener(Self::on_close_tab))
            .on_action(cx.listener(Self::on_toggle_theme))
            .child(render_main_window(&self.state, cx));

        if show_dialog {
            root = root.child(connection_dialog::render_overlay(tokens, &self.state, cx));
        }

        root
    }
}

impl AppView {
    fn on_new_query(&mut self, _: &NewQueryTab, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            let tab = QueryTab::new("Query");
            state.add_tab(tab);
        });
        cx.notify();
    }

    fn on_open_conn(&mut self, _: &OpenConnectionDialog, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            state.show_connection_dialog = true;
        });
        cx.notify();
    }

    fn on_toggle_ai(&mut self, _: &ToggleAiPanel, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            state.ai_panel_open = !state.ai_panel_open;
            if state.ai_panel_open { state.history_panel_open = false; }
        });
        cx.notify();
    }

    fn on_toggle_history(&mut self, _: &ToggleHistoryPanel, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            state.history_panel_open = !state.history_panel_open;
            if state.history_panel_open { state.ai_panel_open = false; }
        });
        cx.notify();
    }

    fn on_toggle_sidebar(&mut self, _: &ToggleSidebar, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            state.sidebar_visible = !state.sidebar_visible;
        });
        cx.notify();
    }

    fn on_close_tab(&mut self, _: &CloseTab, _window: &mut Window, cx: &mut Context<Self>) {
        let tab_id = self.state.read(cx).active_tab_id.clone();
        if let Some(id) = tab_id {
            self.state.update(cx, |state, _cx| { state.close_tab(&id); });
            cx.notify();
        }
    }

    fn on_toggle_theme(&mut self, _: &ToggleTheme, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            let new_mode = match state.theme_mode {
                crate::theme::ThemeMode::Dark => crate::theme::ThemeMode::Light,
                crate::theme::ThemeMode::Light => crate::theme::ThemeMode::Dark,
                crate::theme::ThemeMode::System => crate::theme::ThemeMode::Dark,
            };
            state.set_theme(new_mode);
        });
        cx.notify();
    }
}
