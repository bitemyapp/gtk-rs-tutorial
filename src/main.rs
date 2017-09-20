#![crate_type = "bin"]

extern crate gtk;
extern crate pango;

use gtk::prelude::*;


use std::{thread, time};
use std::io::BufReader;
use std::fs::File;

use gtk::prelude::*;
use gtk::Builder;
use Continue;

use std::cell::RefCell;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn gen_window(rx: Receiver<String>) -> gtk::Window {
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
    let mut text_view1 = text_view.clone();

    send_button.connect_clicked(move |_| {
        let mut new_input = text_input.get_text().unwrap_or("".to_string());
        text_input.set_text("");
        let text_buffer = text_view.get_buffer().expect("Couldn't get window");
        let mut end_iter = text_buffer.get_end_iter();
        new_input.push_str("\n");
        text_buffer.insert(&mut end_iter, &new_input);
    });

    // let mut rx1 = rx.clone();
    let mut rx1 = rx;
    // let mut rc_rx = RefCell::new(rx);
    idle_add(move || {
        // gen_window();
        read_server(&mut rx1, &mut text_view1);
        let sleep_time = time::Duration::from_secs(1);
        thread::sleep(sleep_time);
        Continue(true)
    });

    // idle_add(|| {
    //     gen_window();
    //     let sleep_time = time::Duration::from_millis(2000);
    //     thread::sleep(sleep_time);
    //     Continue(true)
    // });

    window.show_all();
    window
}

// fn read_server() -> Continue + 'static {
fn read_server(mut rx: &Receiver<String>, mut text_view: &gtk::TextView) -> Continue {
// fn read_server(mut rc_rx: RefCell<Receiver<String>>, mut text_view: &gtk::TextView) -> Continue {
    let sleep_time = time::Duration::from_secs(1);
    thread::sleep(sleep_time);
    // let mut ids = Vec::with_capacity(NTHREADS as usize);
    // for _ in 0..NTHREADS {
    //     // The `recv` method picks a message from the channel
    //     // `recv` will block the current thread if there are no messages available
    //     ids.push(rx.recv());
    // }
    // let mut rx = rc_rx.borrow_mut();

    // let buffer = rx.recv().unwrap();
    // let mut buffer =
    let mut buffer = String::with_capacity(25);

    buffer.push_str(&rx.recv().unwrap());
    text_view.get_buffer().expect("Couldn't get window").set_text(&buffer);

    // buffer

    Continue(true)
}

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let thread_tx = tx.clone();
    thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel

        // ignore the Result
        // let _ = stream.write(&[1]);
        // let mut buffer = [0; 10];
        let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();
        let mut buffer = String::new();
        let res = stream.read_to_string(&mut buffer); // ignore here too
        println!("{:?}", res);
        println!("{:?}", buffer);
        thread_tx.send(buffer).unwrap();
    });
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    gen_window(rx);

    // let handler = thread::spawn(|| {
    //     loop {
    //         let sleep_time = time::Duration::from_secs(5);
    //         thread::sleep(sleep_time);
    //         gen_window();
    //     }
    // });
    gtk::main();
}
