mod clock;
mod wallpaper;

use std::{fs, io::Write, io::Result};
use gio::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Clock {
    enabled: bool,
    top_format: String,
    bottom_format: String,
    position_x: i32,
    position_y: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct Wallpaper {
    enabled: bool,
    directory: String,
    bottom_margin: i32,
}

#[derive(Debug, Deserialize)]
struct Config {
    clock: Clock,
    wallpaper: Wallpaper,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn load_css() -> Result<()> {
    // CSS STUFFS
    let display = gdk4::Display::default().expect("Could not get default display.");
    let provider = gtk4::CssProvider::new();
    let priority = gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // READING STYLE FILE
    let default_css = String::from(include_str!("../default-configs/style.css"));

    let home = std::env::var("HOME").expect("Could not get home directory.");
    let style_path = home + "/.config/clockem/style.css";

    if !path_exists(&style_path) {
        let mut file = std::fs::File::create(&style_path)?;
        file.write_all(default_css.as_bytes())?;
    }

    let file_str = fs::read_to_string(&style_path)?;

    // MORE CSS STUFFS
    provider.load_from_data(&file_str);
    gtk4::style_context_add_provider_for_display(&display, &provider, priority);

    Ok(())
}

fn load_json() -> Result<Config> {
    let default_config = String::from(include_str!("../default-configs/config.toml"));

    let home = std::env::var("HOME").expect("Could not get home directory.");
    let config_path = home + "/.config/clockem/config.toml";

    if !path_exists(&config_path) {
        let mut file = std::fs::File::create(&config_path)?;
        file.write_all(default_config.as_bytes())?;
    }

    let file_str = fs::read_to_string(config_path)?;

    let config: Config = toml::from_str(&file_str).unwrap();
    Ok(config)
}

fn main() -> Result<()> {
    // CREATE CONFIG DIRECTORY IF IT DOESN'T EXIST
    let dir_path = std::env::var("HOME").expect("Could not get home directory.") + "/.config/clockem";
    if !path_exists(&dir_path) {
        match fs::create_dir(&dir_path) {
            Ok(_text) => {},
            Err(e) => {
                log::warn!("Could not create config directory: {}", e)
            }
        }
    }

    let application = gtk4::Application::new(Some("com.clockem"), Default::default());
    let config = load_json()?;

    if config.wallpaper.enabled {
        application.connect_activate(move |app| {
            let _ = load_css();
            crate::wallpaper::build(app, config.wallpaper.clone());
        });
    } else {
        application.connect_activate(|_app| {});
    }

    if config.clock.enabled {
        application.connect_activate(move |app| {
            let _ = load_css();
            crate::clock::build(app, config.clock.clone());
        });
    } else {
        application.connect_activate(|_app| {});
    }

    application.run();

    Ok(())
}
