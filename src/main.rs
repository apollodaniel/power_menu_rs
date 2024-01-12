use std::any::Any;

use gtk::{prelude::{ApplicationExt, ApplicationExtManual, WidgetExt, ContainerExt, BoxExt, ButtonExt}, gdk::{ffi::GDK_KEY_Escape, keys::constants::Escape}};
fn main() {
    let application = gtk::Application::builder().application_id("org.apollo.power_menu_rs").build();

    const BUTTON_SIZE: i32 = 80;
    const MARGIN: i32 = 32;

    application.connect_activate(|app|{
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Power menu")
            .default_height((BUTTON_SIZE * 3)+(32*4))
            .default_width(320)
            .resizable(false)
            .build();
        
        let container = gtk::Box::new(gtk::Orientation::Vertical, MARGIN,);
        container.set_margin(MARGIN);
        container.center_widget();

        let shutdown_button = gtk::Button::with_label("Shutdown");
        let reboot_button = gtk::Button::with_label("Reboot");
        let lock_button = gtk::Button::with_label("Lock");

        shutdown_button.connect_clicked(|_| shutdown());
        reboot_button.connect_clicked(|_| reboot());
        lock_button.connect_clicked(|_| lock());

        window.connect_key_press_event(|_, key| {
            if let Some(key) =  key.keycode() {
                if key == 9{
                    std::process::exit(0);
                }
            }
            gtk::glib::Propagation::Stop
        });

        shutdown_button.set_height_request(BUTTON_SIZE);
        reboot_button.set_height_request(BUTTON_SIZE);
        lock_button.set_height_request(BUTTON_SIZE);

        container.add(&shutdown_button);
        container.add(&reboot_button);
        container.add(&lock_button);

        window.add(&container);
        window.show_all();    
    });

    
    
    application.run();
}

fn shutdown(){
    ..=std::process::Command::new("shutdown")
        .arg("now")
        .spawn();
    std::process::exit(0);
}
fn reboot(){
    ..=std::process::Command::new("shutdown")
    .arg("-r")
    .arg("now")
    .spawn();
    std::process::exit(0);
}
fn lock(){
    ..=std::process::Command::new("dm-tool")
    .arg("lock")
    .spawn();
    std::process::exit(0);
}
