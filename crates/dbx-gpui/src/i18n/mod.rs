use std::collections::HashMap;

pub struct I18n {
    messages: HashMap<String, String>,
}

impl I18n {
    pub fn new(locale: Locale) -> Self {
        let messages = match locale {
            Locale::En => en::messages(),
            Locale::ZhCn => zh_cn::messages(),
        };
        I18n { messages }
    }

    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.messages.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    pub fn tf(&self, key: &str, _args: &[(&str, &str)]) -> String {
        self.t(key).to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Locale {
    En,
    ZhCn,
}

mod en;
mod zh_cn;
