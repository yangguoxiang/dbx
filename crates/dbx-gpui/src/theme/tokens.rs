use gpui::*;

/// Design tokens for the DBX UI color system.
#[derive(Clone, Debug)]
pub struct ColorTokens {
    // Background
    pub bg: Hsla,
    pub bg_secondary: Hsla,
    pub bg_tertiary: Hsla,
    pub bg_hover: Hsla,
    pub bg_active: Hsla,

    // Surface
    pub surface: Hsla,
    pub surface_raised: Hsla,
    pub surface_overlay: Hsla,

    // Text
    pub text: Hsla,
    pub text_secondary: Hsla,
    pub text_muted: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,

    // Border
    pub border: Hsla,
    pub border_focus: Hsla,
    pub border_variant: Hsla,

    // Accent
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_active: Hsla,
    pub accent_muted: Hsla,

    // Semantic
    pub success: Hsla,
    pub success_muted: Hsla,
    pub warning: Hsla,
    pub warning_muted: Hsla,
    pub error: Hsla,
    pub error_muted: Hsla,
    pub info: Hsla,
    pub info_muted: Hsla,

    // Misc
    pub shadow: Hsla,
    pub scrollbar: Hsla,
    pub scrollbar_hover: Hsla,
    pub link: Hsla,
    pub selection: Hsla,

    // Spacing
    pub radius_sm: Pixels,
    pub radius_md: Pixels,
    pub radius_lg: Pixels,

    // Typography
    pub font_ui: SharedString,
    pub font_mono: SharedString,
    pub text_size_xs: Pixels,
    pub text_size_sm: Pixels,
    pub text_size_base: Pixels,
    pub text_size_lg: Pixels,
}

pub fn light_theme() -> ColorTokens {
    ColorTokens {
        bg: hsla(0.0, 0.0, 1.0, 1.0),
        bg_secondary: hsla(0.0, 0.0, 0.96, 1.0),
        bg_tertiary: hsla(0.0, 0.0, 0.93, 1.0),
        bg_hover: hsla(0.0, 0.0, 0.95, 1.0),
        bg_active: hsla(0.0, 0.0, 0.9, 1.0),

        surface: hsla(0.0, 0.0, 1.0, 1.0),
        surface_raised: hsla(0.0, 0.0, 1.0, 1.0),
        surface_overlay: hsla(0.0, 0.0, 1.0, 1.0),

        text: hsla(0.0, 0.0, 0.1, 1.0),
        text_secondary: hsla(0.0, 0.0, 0.35, 1.0),
        text_muted: hsla(0.0, 0.0, 0.55, 1.0),
        text_disabled: hsla(0.0, 0.0, 0.7, 1.0),
        text_accent: hsla(220.0, 0.65, 0.5, 1.0),

        border: hsla(0.0, 0.0, 0.9, 1.0),
        border_focus: hsla(220.0, 0.65, 0.5, 1.0),
        border_variant: hsla(0.0, 0.0, 0.85, 1.0),

        accent: hsla(220.0, 0.65, 0.5, 1.0),
        accent_hover: hsla(220.0, 0.65, 0.45, 1.0),
        accent_active: hsla(220.0, 0.65, 0.4, 1.0),
        accent_muted: hsla(220.0, 0.4, 0.9, 1.0),

        success: hsla(142.0, 0.71, 0.45, 1.0),
        success_muted: hsla(142.0, 0.4, 0.9, 1.0),
        warning: hsla(38.0, 0.92, 0.5, 1.0),
        warning_muted: hsla(38.0, 0.4, 0.9, 1.0),
        error: hsla(0.0, 0.84, 0.6, 1.0),
        error_muted: hsla(0.0, 0.4, 0.9, 1.0),
        info: hsla(200.0, 0.75, 0.55, 1.0),
        info_muted: hsla(200.0, 0.4, 0.9, 1.0),

        shadow: hsla(0.0, 0.0, 0.0, 0.1),
        scrollbar: hsla(0.0, 0.0, 0.85, 1.0),
        scrollbar_hover: hsla(0.0, 0.0, 0.75, 1.0),
        link: hsla(220.0, 0.65, 0.5, 1.0),
        selection: hsla(220.0, 0.65, 0.5, 1.0),

        radius_sm: px(4.0),
        radius_md: px(8.0),
        radius_lg: px(12.0),

        font_ui: "Inter".into(),
        font_mono: "JetBrains Mono".into(),
        text_size_xs: px(11.0),
        text_size_sm: px(13.0),
        text_size_base: px(14.0),
        text_size_lg: px(16.0),
    }
}

pub fn dark_theme() -> ColorTokens {
    ColorTokens {
        bg: hsla(240.0, 0.08, 0.08, 1.0),
        bg_secondary: hsla(240.0, 0.08, 0.12, 1.0),
        bg_tertiary: hsla(240.0, 0.08, 0.16, 1.0),
        bg_hover: hsla(240.0, 0.04, 0.18, 1.0),
        bg_active: hsla(240.0, 0.04, 0.24, 1.0),

        surface: hsla(240.0, 0.08, 0.12, 1.0),
        surface_raised: hsla(240.0, 0.08, 0.15, 1.0),
        surface_overlay: hsla(240.0, 0.08, 0.2, 1.0),

        text: hsla(240.0, 0.05, 0.9, 1.0),
        text_secondary: hsla(240.0, 0.04, 0.6, 1.0),
        text_muted: hsla(240.0, 0.04, 0.45, 1.0),
        text_disabled: hsla(240.0, 0.04, 0.35, 1.0),
        text_accent: hsla(220.0, 0.7, 0.6, 1.0),

        border: hsla(240.0, 0.08, 0.2, 1.0),
        border_focus: hsla(220.0, 0.7, 0.6, 1.0),
        border_variant: hsla(240.0, 0.08, 0.25, 1.0),

        accent: hsla(220.0, 0.7, 0.6, 1.0),
        accent_hover: hsla(220.0, 0.7, 0.55, 1.0),
        accent_active: hsla(220.0, 0.7, 0.5, 1.0),
        accent_muted: hsla(220.0, 0.3, 0.2, 1.0),

        success: hsla(142.0, 0.6, 0.5, 1.0),
        success_muted: hsla(142.0, 0.3, 0.15, 1.0),
        warning: hsla(38.0, 0.8, 0.5, 1.0),
        warning_muted: hsla(38.0, 0.3, 0.15, 1.0),
        error: hsla(0.0, 0.7, 0.55, 1.0),
        error_muted: hsla(0.0, 0.3, 0.15, 1.0),
        info: hsla(200.0, 0.7, 0.5, 1.0),
        info_muted: hsla(200.0, 0.3, 0.15, 1.0),

        shadow: hsla(0.0, 0.0, 0.0, 0.3),
        scrollbar: hsla(240.0, 0.08, 0.28, 1.0),
        scrollbar_hover: hsla(240.0, 0.08, 0.35, 1.0),
        link: hsla(220.0, 0.7, 0.6, 1.0),
        selection: hsla(220.0, 0.3, 0.3, 0.4),

        radius_sm: px(4.0),
        radius_md: px(8.0),
        radius_lg: px(12.0),

        font_ui: "Inter".into(),
        font_mono: "JetBrains Mono".into(),
        text_size_xs: px(11.0),
        text_size_sm: px(13.0),
        text_size_base: px(14.0),
        text_size_lg: px(16.0),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}
