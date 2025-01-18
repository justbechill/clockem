use crate::Clock;
use chrono::Local;
use gtk4::prelude::*;
use gtk4_layer_shell::{Layer, LayerShell};

/**
Configures, renders, and updates clock widget.
*/
pub fn build(application: &gtk4::Application, clock_config: Clock) {
    // SET UP WINDOW AS A LAYER
    let clock_window = gtk4::ApplicationWindow::new(application);
    clock_window.init_layer_shell();
    clock_window.set_layer(Layer::Bottom);

    // LABEL STUFFS
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let top = gtk4::Label::new(Some("12:01:34 AM"));
    let bottom = gtk4::Label::new(Some("Jan 17, 2025"));

    top.add_css_class("clock-top");
    bottom.add_css_class("clock-bottom");

    container.append(&top);
    container.append(&bottom);

    // WINDOW FORMATTING
    clock_window.set_title(Some("clockem-clock"));

    // GTK4 removed the ability to just set a window's position so we have to move the clock around by messing with the window's size and aligment...
    if clock_config.position_x < 0 { container.set_halign(gtk4::Align::Start); }
    else { container.set_valign(gtk4::Align::End); }

    if clock_config.position_y < 0 { container.set_valign(gtk4::Align::Start); }
    else { container.set_valign(gtk4::Align::End); }

    let position_x = clock_config.position_x.abs() * 2;
    let position_y = clock_config.position_y.abs() * 2;

    clock_window.set_default_size(position_x, position_y);

    // SHOW CLOCK
    clock_window.set_child(Some(&container));
    clock_window.show();

    // UPDATING CLOCK
    let tick = move || {
        top.set_text(&format!(
            "{}",
            Local::now().format(&clock_config.top_format)
        ));
        bottom.set_text(&format!(
            "{}",
            Local::now().format(&clock_config.bottom_format)
        ));
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    glib::timeout_add_seconds_local(1, tick);
}
