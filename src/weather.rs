use crate::formatting::parse_weather_string;
use crate::Weather;
use gtk4::{prelude::*, Align};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use weer_api::*;

pub fn build(application: &gtk4::Application, weather_config: Weather) {
    // SET UP WINDOW AS A LAYER
    let weather_window = gtk4::ApplicationWindow::new(application);
    weather_window.init_layer_shell();
    weather_window.set_layer(Layer::Background);
    weather_window.set_namespace("clockem-weather");
    weather_window.add_css_class("weather");

    // LABEL STUFFS
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let top = gtk4::Label::new(Some("Weather"));
    let bottom = gtk4::Label::new(Some(""));

    top.add_css_class("weather-top");
    bottom.add_css_class("weather-bottom");

    container.append(&top);
    container.append(&bottom);

    // WINDOW FORMATTING
    weather_window.set_title(Some("clockem-weather"));

    if let Some(width) = weather_config.width {
        weather_window.set_default_width(width);
    }

    if let Some(height) = weather_config.height {
        weather_window.set_default_height(height);
    }

    let position_x = weather_config.position_x.unwrap_or(0);
    let position_y = weather_config.position_y.unwrap_or(0);

    // SET MARGINS AND ANCHOR EDGES
    let mut anchors = Vec::new();

    if let Some(s) = weather_config.y_align {
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
    }

    for (anchor, state) in anchors {
        weather_window.set_anchor(anchor, state);
    }

    weather_window.set_margin(Edge::Left, position_x);
    weather_window.set_margin(Edge::Top, position_y);

    // SET TEXT ALIGN
    if let Some(s) = weather_config.text_align {
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

    // SHOW WEATHER
    weather_window.set_child(Some(&container));
    weather_window.show();

    // GET WEATHER DATA
    let client = Client::new(
        weather_config.api_key.unwrap_or("".to_string()).as_str(),
        true,
    );

    // UPDATE WEATHER
    let top_format = weather_config
        .top_format
        .unwrap_or("%S %F degrees F".to_string());
    let bottom_format = weather_config.bottom_format.unwrap_or("%L, %R".to_string());

    let location = weather_config.location.unwrap_or("London, GB".to_string());
    let error_message = weather_config
        .error_message
        .unwrap_or("Weather Error".to_string());
    let daynight_strings = weather_config
        .daynight_strings
        .unwrap_or(vec!["Day".to_string(), "Night".to_string()]);

    let tick = move || {
        let result = client
            .realtime()
            .query(Query::City(location.clone()))
            .call();

        match result {
            Ok(data) => {
                let top_parsed = parse_weather_string(&data, &top_format, &daynight_strings);
                let bottom_parsed = parse_weather_string(&data, &bottom_format, &daynight_strings);

                top.set_text(&top_parsed);
                bottom.set_text(&bottom_parsed);
            }
            Err(e) => {
                top.set_text(&error_message);
                eprintln!("Weather error: {}", e);
            }
        }

        glib::ControlFlow::Continue
    };

    let _ = &tick();
    glib::timeout_add_seconds_local(weather_config.update_interval.unwrap_or(5) * 60, tick);
}
