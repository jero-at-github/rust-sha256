#![allow(dead_code, unused_variables)]
extern crate core_affinity;

use chrono::Utc;
use core_affinity::CoreId;
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use sha256_calculator::search;
use std::thread;
use std::thread::JoinHandle;

const N_ZEROS: i32 = 8;

fn main() {
    // Retrieve the IDs of a<ll active CPU cores.
    let core_ids = core_affinity::get_core_ids().unwrap();
    let (sender, receiver) = channel::unbounded::<String>();
    let mut handler_params: Vec<(CoreId, Sender<String>, Receiver<String>)> = Vec::new();

    for core_id in core_ids {
        handler_params.push((core_id, sender.clone(), receiver.clone()));
    }

    drop(sender);

    let mut now = Utc::now();
    println!("{}", now);

    // Create a thread for each active CPU core.
    let handles = handler_params
        .into_iter()
        .map(|param| {
            thread::spawn(move || {
                // Pin this thread to a single CPU core.
                core_affinity::set_for_current(param.0);
                search(N_ZEROS, param.0, param.1, param.2);
            })
        })
        .collect::<Vec<JoinHandle<()>>>();

    for msg in receiver {
        println!("{}", msg);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    now = Utc::now();
    println!("{}", now);
}
