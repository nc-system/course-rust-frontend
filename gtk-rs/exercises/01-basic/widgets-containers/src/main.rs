
use gtk4 as gtk;
use gtk::{
    gdk, gio,
    glib::{self, clone},
    prelude::*,
};
// use gtk::prelude::*;
// use gtk::{glib, Application, ApplicationWindow, Button };


fn main() -> glib::ExitCode {
    
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.clipboard")
        .build();
    application.connect_activate(build_ui);
    application.run()

}


fn build_ui(application: &gtk::Application) {

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Widgets")
        .default_width(660)
        .default_height(420)
        .build();

    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    // Container
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    // Title
    let title = gtk::Label::builder()
        .label("Label")
        .halign(gtk::Align::Start)
        .build();
    title.add_css_class("title-2");
    container.append(&title);

    // Entry
    let entry = gtk::Entry::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    container.append(&entry);

    // Button
    let button = gtk::Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .label("Click Me!")
        .build();

    container.append(&button);

    // container.append(&texture_container);
    window.set_child(Some(&container));
    window.present();
}
