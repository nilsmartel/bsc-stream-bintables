mod table_lake;
use std::{sync::mpsc::channel, thread::spawn};

use table_lake::TableLakeReader;

fn main() {
    let bintablefile = std::env::args()
        .nth(1)
        .expect("find bintablefile as first argument");

    let mut table = table_lake::BinTable::open(&bintablefile, 1000).unwrap();

    let (s, r) = channel();
    let handle = spawn(move || table.read(s));

    eprintln!("start streaming");

    let i = r.iter().count();

    eprintln!("read {i} entries");

    handle.join().expect("join thread");
}
