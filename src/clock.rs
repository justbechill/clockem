use crate::Clock;
use chrono::Local;
use gtk4::{prelude::*, Align};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

/**
Configures, renders, and updates clock widget.
*/
pub fn build(application: &gtk4::Application, clock_config: Clock) {
    // SET UP WINDOW AS A LAYER
    let clock_window = gtk4::ApplicationWindow::new(application);
    clock_window.init_layer_shell();
    clock_window.set_layer(Layer::Background);
    clock_window.set_namespace("clockem-clock");

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

    let position_x = clock_config.position_x.unwrap_or(0);
    let position_y = clock_config.position_y.unwrap_or(0);

    // SETTING MARGIN AND ANCHOR EDGES
    let mut anchors = Vec::new();

    if let Some(s) = clock_config.y_align {
        match s.as_str() {
            "center" => {
                anchors.push((Edge::Left, true));
                container.set_valign(Align::Center)
            }
            "bottom" => {
                anchors.push((Edge::Left, true));
                anchors.push((Edge::Bottom, true));
            }
            _ => {
                anchors.push((Edge::Left, true));
                anchors.push((Edge::Top, true));
            }
        }
    } else {
        anchors.push((Edge::Left, true));
        anchors.push((Edge::Top, true));
    }

    for (anchor, state) in anchors {
        clock_window.set_anchor(anchor, state);
    }

    clock_window.set_margin(Edge::Left, position_x);
    clock_window.set_margin(Edge::Top, position_y);

    // SET TEXT ALIGN
    if let Some(s) = clock_config.text_align {
        match s.as_str() {
            "center" => {
                top.set_halign(Align::Center);
                bottom.set_halign(Align::Center);
            }
            "right" => {
                top.set_halign(Align::End);
                bottom.set_halign(Align::End);
            }
            _ => {
                top.set_halign(Align::Start);
                bottom.set_halign(Align::Start);
            }
        }
    } else {
        top.set_halign(Align::Start);
        bottom.set_halign(Align::Start);
    }

    // SHOW CLOCK
    clock_window.set_child(Some(&container));
    clock_window.show();

    // UPDATING CLOCK
    let top_format = clock_config.top_format.unwrap_or("%H:%M:%S".to_string());
    let bottom_format = clock_config
        .bottom_format
        .unwrap_or("%b %d, %Y".to_string());

    let tick = move || {
        top.set_text(&format!("{}", Local::now().format(&top_format)));
        bottom.set_text(&format!("{}", Local::now().format(&bottom_format)));
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    let _ = &tick();
    glib::timeout_add_seconds_local(clock_config.update_interval.unwrap_or(1), tick);
}
