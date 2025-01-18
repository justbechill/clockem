//TODO:
// AUTOMATICALLY CREATE CONFIG/CSS FILE
// CHANGE CONFIG LANGUAGE

mod clock;

use std::fs;
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

    let default_css = String::from(include_str!("default_style.css"));

    let home = std::env::var("HOME").expect("Could not get home directory.");
    let path = home + "/.config/clockem/style.css";

    let file_str = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            log::warn!("Could not load css: {}", e);
            default_css
        }
    };

    provider.load_from_data(&file_str);
    gtk4::style_context_add_provider_for_display(&display, &provider, priority);
}

fn load_json() -> Config {
    let default_config = String::from(include_str!("default_config.json"));

    let home = std::env::var("HOME").expect("Could not get home directory.");
    let path = home + "/.config/clockem/config.json";

    let file_str = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            log::warn!("Could not load config: {}", e);
            default_config
        }
    };

    let config: Config = serde_json::from_str(&file_str).expect("Could not parse config");
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
        application.connect_activate(|_app| {});
    }

    application.run();
}
