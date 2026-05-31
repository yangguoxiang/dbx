use std::sync::atomic::{AtomicUsize, Ordering};

use gpui::*;

use crate::i18n::{I18n, Locale};
use crate::theme::{ColorTokens, ThemeMode};

pub type TabId = String;

static NEXT_TAB_ID: AtomicUsize = AtomicUsize::new(1);

fn next_tab_id() -> TabId {
    format!("tab-{}", NEXT_TAB_ID.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone, PartialEq)]
pub enum TabMode {
    Query,
    Data,
    Structure,
    ObjectSource,
    Redis,
    Mongo,
}

pub struct QueryTab {
    pub id: TabId,
    pub title: String,
    pub mode: TabMode,
    pub connection_id: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
    pub sql: String,
    pub is_executing: bool,
    pub table_name: Option<String>,
}

impl QueryTab {
    pub fn new(title: &str) -> Self {
        QueryTab {
            id: next_tab_id(),
            title: title.to_string(),
            mode: TabMode::Query,
            connection_id: None,
            database: None,
            schema: None,
            sql: String::new(),
            is_executing: false,
            table_name: None,
        }
    }
}

pub struct UiState {
    pub theme_mode: ThemeMode,
    pub tokens: ColorTokens,
    pub locale: Locale,
    pub i18n: I18n,

    pub connections: Vec<String>,
    pub connected_ids: Vec<String>,

    pub tabs: Vec<QueryTab>,
    pub active_tab_id: Option<TabId>,

    pub sidebar_visible: bool,
    pub sidebar_width: Pixels,
    pub ai_panel_open: bool,
    pub history_panel_open: bool,

    pub show_actions_menu: bool,
    pub show_connection_dialog: bool,
    pub show_ssh_section: bool,
    pub show_ssl_section: bool,

    pub status_message: String,

    pub toast_message: Option<String>,
}

impl UiState {
    pub fn new() -> Self {
        let locale = Locale::ZhCn;
        let theme_mode = ThemeMode::Dark;
        let tokens = crate::theme::dark_theme();
        let i18n = I18n::new(locale);

        UiState {
            theme_mode,
            tokens,
            locale,
            i18n,
            connections: Vec::new(),
            connected_ids: Vec::new(),
            tabs: Vec::new(),
            active_tab_id: None,
            sidebar_visible: true,
            sidebar_width: px(260.0),
            ai_panel_open: false,
            history_panel_open: false,
            show_actions_menu: false,
            show_connection_dialog: false,
            show_ssh_section: false,
            show_ssl_section: false,
            status_message: String::from("status.ready"),
            toast_message: None,
        }
    }

    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.i18n.t(key)
    }

    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.tokens = match mode {
            ThemeMode::Light => crate::theme::light_theme(),
            ThemeMode::Dark | ThemeMode::System => crate::theme::dark_theme(),
        };
    }

    pub fn active_tab(&self) -> Option<&QueryTab> {
        self.active_tab_id
            .as_ref()
            .and_then(|id| self.tabs.iter().find(|t| &t.id == id))
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut QueryTab> {
        self.active_tab_id
            .clone()
            .and_then(|id| self.tabs.iter_mut().find(|t| t.id == id))
    }

    pub fn add_tab(&mut self, tab: QueryTab) -> TabId {
        let id = tab.id.clone();
        self.tabs.push(tab);
        self.active_tab_id = Some(id.clone());
        id
    }

    pub fn close_tab(&mut self, tab_id: &str) {
        let index = self.tabs.iter().position(|t| t.id == tab_id);
        if let Some(i) = index {
            self.tabs.remove(i);
            if self.active_tab_id.as_deref() == Some(tab_id) {
                self.active_tab_id = self.tabs
                    .get(i.min(self.tabs.len().saturating_sub(1)))
                    .map(|t| t.id.clone())
                    .or_else(|| self.tabs.first().map(|t| t.id.clone()));
            }
        }
    }

    pub fn show_toast(&mut self, message: String) {
        self.toast_message = Some(message);
    }

    pub fn dismiss_toast(&mut self) {
        self.toast_message = None;
    }
}
