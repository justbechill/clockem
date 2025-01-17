use chrono::Local;
use gio::prelude::*;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn load_css() {
    let display = gdk4::Display::default().expect("Could not get default display.");
    let provider = gtk4::CssProvider::new();
    let priority = gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("style.css"));
    gtk4::style_context_add_provider_for_display(&display, &provider, priority);
}

fn current_time() -> String {
    format!("{}", Local::now().format("%I:%M:%S %p"))
}

fn current_date() -> String {
    format!("{}", Local::now().format("%b %d, %Y"))
}

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn build_ui(application: &gtk4::Application) {
    // Create a normal GTK window however you like
    let window = gtk4::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Bottom);

    //Clock stuffs
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let top = gtk4::Label::new(Some("12:01:34 AM"));
    top.add_css_class("clock-top");
    vbox.append(&top);

    let bottom = gtk4::Label::new(Some("Jan 17, 2025"));
    bottom.add_css_class("clock-bottom");
    vbox.append(&bottom);

    // we are using a closure to capture the label (else we could also use a normal
    // function)
    let tick = move || {
        top.set_text(&current_time());
        bottom.set_text(&current_date());
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);

    // Window formatting
    window.set_title(Some("Clock"));
    window.set_default_size(1550, 400);

    // GTK4 removed the ability to just set a window's position so we have to move the clock around by messing with alignment and margins...
    vbox.set_halign(gtk4::Align::Start);
    vbox.set_valign(gtk4::Align::Center);

    // Set widgets and show window
    window.set_child(Some(&vbox));
    window.show()
}

fn main() {
    let application = gtk4::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        load_css();
        build_ui(app);
    });

    application.run();
}
