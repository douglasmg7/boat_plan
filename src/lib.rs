mod boat;
mod si;

use cairo::{Context, PdfSurface};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
// use cairo::{Context, Format, ImageSurface};
use si::Length;
// use std::fs::File;

pub fn run() {
    let mut boat = boat::Boat::new("Sail cruiser".to_string());
    boat.set_loa(Length::from_foot(13.0));
    boat.set_b_max(Length::from_foot(4.0));
    println!("{}", boat);

    let ratios = boat::Ratios::new(&boat);
    println!("\n{}", ratios);

    // let surface =
    // ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn’t create a surface!");
    let surface = PdfSurface::new(600., 600., "/home/douglasmg7/Downloads/output.pdf")
        .expect("Couldn’t create a surface!");
    let context = Context::new(&surface);
    context.set_source_rgb(1.0, 1.0, 1.0);
    //  Fills the context with the current source value.
    context.paint();
    // Set to black.
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.line_to(100.0, 100.0);
    context.line_to(500.0, 100.0);
    context.line_to(500.0, 500.0);
    context.stroke();

    // let mut file =
    // File::create("/home/douglasmg7/Downloads/output.png").expect("Couldn’t create file.");
    // surface.write(&mut file).expect("Couldn’t write to png");
    surface.flush();

    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(550, 100);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
