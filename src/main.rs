use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    println!("before the thread");
    thread::spawn(|| {
        println!("Hello from in the thread");
    });

    println!("sync, arc, and mutex example");

    let data = Arc::new(Mutex::new(vec![1u32, 2, 3, 4, 5, 6]));

    for i in 0..6 {
        let data = data.clone();
        thread::spawn(move || {
            println!("\n\tentering thread {}", i);
            let mut data = data.lock().unwrap();
            data[i] += 1;
            println!("\n\tleaving thread {}", i);
        });
    }
    thread::sleep_ms(50);


    println!("\nUsing Channels");

    let channel_data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();

    for j in 0..10 {
        let (data, tx) = (channel_data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            println!("\tsend... {}", j);
            tx.send(());
        });
    }

    for i in 0..10 {
        println!("\trecv {}", i);
        rx.recv();
    }

    println!("\nActually passing data using channels");
    let (tx1, rx1) = mpsc::channel();
    for i in 1..12 {
        let tx = tx1.clone();
        thread::spawn(move || {
            println!("\tpass data using channels: {}", i);
            tx.send(i);
        });
    }
    for _ in 1..12 {
        let out = rx1.recv().ok().expect("Could not receive answer");
        println!("\t receiving {}", out);
    }
}
