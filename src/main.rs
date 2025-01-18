mod clock;

use gio::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Clock {
    enabled: bool,
    top_format: String,
    bottom_format: String,
    position_x: i32,
    position_y: i32,
}

#[derive(Debug, Deserialize)]
struct Config {
    clock: Clock,
}

fn load_css() {
    let display = gdk4::Display::default().expect("Could not get default display.");
    let provider = gtk4::CssProvider::new();
    let priority = gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("style.css"));
    gtk4::style_context_add_provider_for_display(&display, &provider, priority);
}

fn load_json() -> Config {
    let file = include_str!("config.json");
    let config: Config = serde_json::from_str(file).expect("Could not parse config.json");
    config
}

fn main() {
    let application = gtk4::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());
    let config = load_json();

    if config.clock.enabled {
        application.connect_activate(|app| {
            load_css();
            let config = load_json();
            crate::clock::build(app, config.clock);
        });
    } else {
        application.connect_activate(|app| {});
    }

    application.run();
}
