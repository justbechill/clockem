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
    clock_window.add_css_class("clock");

    // WINDOW FORMATTING
    clock_window.set_title(Some("clockem-clock"));

    if let Some(width) = clock_config.width {
        clock_window.set_default_width(width);
    }

    if let Some(height) = clock_config.height {
        clock_window.set_default_height(height);
    }

    let position_x = clock_config.position_x.unwrap_or(0);
    let position_y = clock_config.position_y.unwrap_or(0);

    // SET JUSTIFICATION
    let mut align = Align::Start;

    if let Some(s) = clock_config.text_align {
        align = match s.as_str() {
            "center" => Align::Center,
            "right" => Align::End,
            _ => Align::Start,
        };
    }

    // CREATE WINDOW
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let top = crate::init_label("clock-top", align, &clock_config.top_format);
    let bottom = crate::init_label("clock-bottom", align, &clock_config.bottom_format);

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

    //BUILD WINDOW
    container.append(&top);
    container.append(&bottom);
    clock_window.set_child(Some(&container));

    // SHOW CLOCK
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
