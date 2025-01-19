use crate::Wallpaper;
use gtk4::prelude::*;
use gtk4_layer_shell::{Layer, LayerShell};

pub fn build(application: &gtk4::Application, wp_config: Wallpaper) {
    // SET UP WINDOW AS A LAYER
    let wp_window = gtk4::ApplicationWindow::new(application);
    wp_window.init_layer_shell();
    wp_window.set_layer(Layer::Background);

    // COVER FULL SCREEN
    wp_window.fullscreen();

    // CREATE PICTURE WIDGET
    let home = std::env::var("HOME").expect("Could not load wallpaper.");

    let mut wp_dir = wp_config.directory;
    wp_dir = wp_dir.replace("~", &home);

    let file = gio::File::for_path(&wp_dir);
    let picture = gtk4::Picture::for_file(&file);

    // BOTTOM MARGIN THING BECAUSE OF WAYBAR
    picture.set_margin_bottom(wp_config.bottom_margin);

    wp_window.set_child(Some(&picture));


    wp_window.set_title(Some("clockem-wallpaper"));
    wp_window.show();
}
