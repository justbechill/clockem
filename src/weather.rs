use crate::Weather;
use gtk4::{prelude::*, Align};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use weer_api::{
    chrono::{TimeZone, Utc},
    *,
};

pub fn build(application: &gtk4::Application, weather_config: Weather) {
    // SET UP WINDOW AS A LAYER
    let weather_window = gtk4::ApplicationWindow::new(application);
    weather_window.init_layer_shell();
    weather_window.set_layer(Layer::Background);
    weather_window.set_namespace("clockem-weather");

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

    let position_x = weather_config.position_x.unwrap_or(0);
    let position_y = weather_config.position_y.unwrap_or(0);

    // TESTING WINDOW POSITION STUFFS
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

    // SHOW WEATHER
    weather_window.set_child(Some(&container));
    weather_window.show();

    // GET WEATHER DATA
    let client = Client::new(
        weather_config.api_key.unwrap_or("".to_string()).as_str(),
        true,
    );

    let result = client
        .realtime()
        .query(Query::City(
            weather_config.location.unwrap_or("London".to_string()),
        ))
        .call();

    match result {
        Ok(data) => {}
        Err(e) => {
            top.set_text(
                weather_config
                    .error_message
                    .unwrap_or("Weather Error".to_string())
                    .as_str(),
            );
            eprintln!("Weather error: {}", e);
        }
    }
}
