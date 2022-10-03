mod table_lake;
use std::{sync::mpsc::channel, thread::spawn};

use table_lake::TableLakeReader;

fn main() {
    let bintablefile = std::env::args()
        .nth(1)
        .expect("find bintablefile as first argument");

    let table = table_lake::BinTable::open(&bintablefile, 1000).unwrap();

    let (s, r) = channel();
    let handle = spawn(|| table.read(s));

    for (s, row) in r {
        println!("{s}: {:?}", row);
    }

    handle.join().expect("join thread");
}
