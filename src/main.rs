#![crate_type = "bin"]

extern crate gtk;
extern crate pango;

use gtk::prelude::*;


use std::{thread, time};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use gtk::prelude::*;
use gtk::Builder;
use Continue;

fn gen_window() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        // panic!("lol");
        gtk::main_quit();
        Inhibit(false)
    });

    let glade_src = include_str!("text_viewer.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Couldn't add from string");

    let window: gtk::Window = builder.get_object("window").expect("Couldn't get window");
    let send_button: gtk::Button =
        builder.get_object("send_button")
               .expect("Couldn't get builder");
    let text_input: gtk::Entry = builder.get_object("text_input")
                                        .expect("Couldn't get text_input");

    let text_view: gtk::TextView = builder.get_object("text_view")
                                          .expect("Couldn't get text_view");

    // let window1 = window.clone();
    send_button.connect_clicked(move |_| {
        let mut new_input = text_input.get_text().unwrap_or("".to_string());
        text_input.set_text("");
        let text_buffer = text_view.get_buffer().expect("Couldn't get window");
        let mut end_iter = text_buffer.get_end_iter();
        new_input.push_str("\n");
        text_buffer.insert(&mut end_iter, &new_input);
        // text_view.get_buffer().expect("Couldn't get window").set_text(&new_input);
    });

    // let label = gtk::Label::new("Some text");
    // let attr_list = pango::AttrList::new();

    // let mut attr = pango::Attribute::new_background(65535, 0, 0)
    //                                 .expect("Couldn't create new background");
    // attr.set_start_index(0);
    // attr.set_end_index(2);
    // attr_list.insert(attr);

    // let mut attr = pango::Attribute::new_underline(pango::Underline::Single)
    //                                 .expect("Couldn't create new underline");
    // attr.set_start_index(1);
    // attr.set_end_index(4);
    // attr_list.insert(attr);

    // let mut attr = pango::Attribute::new_strikethrough(true)
    //                                 .expect("Couldn't create new strikethrough");
    // attr.set_start_index(5);
    // attr_list.insert(attr);

    // let mut attr = pango::Attribute::new_scale(1.2)
    //                                 .expect("Couldn't create new scale");
    // attr.set_start_index(6);
    // attr_list.insert(attr);

    // label.set_attributes(&attr_list);
    // window.add(&label);

    // let button = gtk::Button::new_with_label("Click me!");
    // let button2 = gtk::Button::new_with_label("Click me too!");
    // window.add(&button);
    window.show_all();
    // window.add(&button2);
    window
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    gen_window();

    // idle_add(|| {
    //     gen_window();
    //     let sleep_time = time::Duration::from_millis(2000);
    //     thread::sleep(sleep_time);
    //     Continue(true)
    // });
    // let handler = thread::spawn(|| {
    //     loop {
    //         let sleep_time = time::Duration::from_secs(5);
    //         thread::sleep(sleep_time);
    //         gen_window();
    //     }
    // });
    gtk::main();
}
