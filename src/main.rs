#![allow(dead_code, unused_variables)]
extern crate core_affinity;

use core_affinity::CoreId;
use crossbeam::channel;
use crossbeam::channel::Sender;
use sha256_calculator::search;
use std::thread;

const N_ZEROS: i32 = 2;

fn main() {
    // Retrieve the IDs of a<ll active CPU cores.
    let core_ids = core_affinity::get_core_ids().unwrap();
    let (sender, reciever) = channel::unbounded::<String>();
    let mut handler_params: Vec<(CoreId, Sender<String>)> = Vec::new();

    for idx in 0..(core_ids.len()) {
        handler_params.push((core_ids[idx], sender.clone()));
    }

    // Create a thread for each active CPU core.
    let handles = handler_params
        .into_iter()
        .map(|param| {
            thread::spawn(move || {
                // Pin this thread to a single CPU core.
                core_affinity::set_for_current(param.0);
                search(N_ZEROS, param.1);
            })
        })
        .collect::<Vec<_>>();

    drop(sender);

    for msg in reciever {
        println!("Child thread: Received {}", msg);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
