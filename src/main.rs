
extern crate clap;
extern crate gtk;
extern crate rand;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use clap::{Arg, App};
use gtk::prelude::*;
use rand::{thread_rng, Rng};



fn main() {
    let matches = App::new("fix-display-dots")
        .version("1.0")
        .author("anekos")
        .about("Display dots remover")
        .arg(Arg::with_name("size")
             .short("s")
             .long("size")
             .help("Color block size")
             .takes_value(true))
        .arg(Arg::with_name("interval")
             .help("Draw interval")
             .short("i")
             .long("interval")
             .help("Color block size")
             .takes_value(true))
        .get_matches();

    let size = matches.value_of("size").map(|it| it.parse().unwrap()).unwrap_or(10);
    let interval = matches.value_of("interval").map(|it| it.parse().unwrap()).unwrap_or(100);

    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_border_width(0);
    window.set_role("fix-display-dots");
    window.set_position(gtk::WindowPosition::Center);

    let dw = gtk::DrawingArea::new();
    window.add(&dw);

    window.show_all();

    dw.connect_draw(move |this, ctx| {
        let mut rng = thread_rng();

        let alloc = this.get_allocation();
        let (w, h) = (alloc.width, alloc.height);

        for x in (0..w).step_by(size) {
            for y in (0..h).step_by(size) {
                ctx.set_source_rgb(rng.gen(), rng.gen(), rng.gen());

                let x = x as f64;
                let y = y as f64;
                ctx.rectangle(x, y, x + 100.0, y + 100.0);
                ctx.fill();
            }
        }
        Inhibit(false)
    });

    window.connect_delete_event(|_, _| {
        exit(0);
    });

    loop {
        while gtk::events_pending() {
            gtk::main_iteration();
        }
        dw.queue_draw();
        sleep(Duration::from_millis(interval));
    }
}
