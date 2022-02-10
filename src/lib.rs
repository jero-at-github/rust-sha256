use crossbeam::channel::Sender;
use rand::Rng;
use sha256::digest;

fn gen_hash() -> String {
    let mut rng = rand::thread_rng();
    let input: u32 = rng.gen();
    digest(input.to_string())
}

pub fn search(n_zeros: i32, sender: Sender<String>) {
    let mut sufix: String = String::from("");
    for _ in 0..n_zeros {
        sufix += "0";
    }
    let mut val = gen_hash();

    loop {
        if val.ends_with(&sufix) {
            sender.send(val).unwrap();
            drop(sender);
            break;
        };

        val = gen_hash();
    }
}
