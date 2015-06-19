
extern crate linuxdvb;

use linuxdvb::{Frontend,ReadWriteMode,BlockMode};

fn main() {
    let frontend = Frontend::open("/dev/dvb/adapter0/frontend0", ReadWriteMode::ReadOnly, BlockMode::NonBlocking).unwrap();
    println!("{:?}", frontend.get_info().unwrap());
}