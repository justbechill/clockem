use gio::prelude::*;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use pango::ffi::pango_attr_list_insert;

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn build_ui(application: &gtk4::Application) {
    // Create a normal GTK window however you like
    let window = gtk4::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Overlay);

    // Push other windows out of the way
    window.auto_exclusive_zone_enable();

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 40);
    window.set_margin(Edge::Right, 40);
    window.set_margin(Edge::Top, 20);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, false),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    //Clock stuffs
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let time = gtk4::Label::new(Some("12:01:34 AM"));

    let mut df = pango::FontDescription::new();
    df.set_family("JetBrains Mono");
    let attr = pango::AttrFontDesc::new(&df);
    let attrlist = pango::AttrList::new();
    attrlist.insert(attr);

    time.set_attributes(Some(&attrlist));
    //time.set_markup("<span font=\"Jetbrains Mono\">Your mother</span>");

    vbox.append(&time);

    // Set up a widget
    let label = gtk4::Label::new(Some(""));
    label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    window.set_child(Some(&vbox));
    window.show()
}

fn main() {
    let application = gtk4::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
