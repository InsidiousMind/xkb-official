extern crate xkbcommon;
extern crate xcb;
use xkbcommon::xkb;

struct Keyboard {
    conn: xcb::Connection,
    first_xkb_event: u8,
}

fn main() {
    println!("Hello, world!");
}
