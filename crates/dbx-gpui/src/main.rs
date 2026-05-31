mod app;
mod state;
mod actions;
mod theme;
mod i18n;
mod shell;
mod connection;
#[allow(unused)]
mod widgets;
#[allow(unused)]
mod editor;
#[allow(unused)]
mod grid;
#[allow(unused)]
mod schema;
#[allow(unused)]
mod ai;
#[allow(unused)]
mod redis;
#[allow(unused)]
mod mongo;
#[allow(unused)]
mod diagram;
#[allow(unused)]
mod diff;
#[allow(unused)]
mod export;
#[allow(unused)]
mod import;
#[allow(unused)]
mod transfer;
#[allow(unused)]
mod chart;
#[allow(unused)]
mod history;
#[allow(unused)]
mod settings;
#[allow(unused)]
mod plugins;
#[allow(unused)]
mod dialogs;
#[allow(unused)]
mod util;

use gpui::*;

fn main() {
    let app = gpui_platform::application();
    app.run(|cx: &mut App| {
        cx.open_window::<app::AppView>(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1400.0), px(900.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("DBX".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| app::AppView::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
