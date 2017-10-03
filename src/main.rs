/**
 * @author Andrew Plaza
 * Taken from official xkbcommon example
 * Note: This program only handles the core keyboard device for now.
 * It should be straigtforward to change struct keyboard to a list of
 * keyboards with device IDs, as in test/interactive-evdev.c. This would
 * require:
 *
 * - Initially listing the keyboard devices.
 * - Listening to device changes.
 * - Matching events to their devices.
 *
 * XKB itself knows about xinput1 devices, and most requests and events are
 * device-specific.
 *
 * In order to list the devices and react to changes, you need xinput1/2.
 * You also need xinput for the key press/release event, since the core
 * protocol key press event does not carry a device ID to match on.
 */
extern crate xcb;
extern crate xkbcommon;

use xcb::base;
use xkbcommon::xkb;
use xkbcommon::xkb::x11::{MIN_MAJOR_XKB_VERSION, MIN_MINOR_XKB_VERSION};
use std::cell::{Cell, RefCell};

struct Keyboard {
    conn: xcb::Connection,
    first_xkb_event: u8,
    context: xkb::Context,
    keymap: xkb::Keymap,
    state: RefCell<xkb::State>,
    device_id: i32,
}

impl Keyboard {

}

fn main() {
    let mut first_xkb_event: u8 = 0;
    /// get a new X11 Connection
    let (conn, screen) = xcb::Connection::connect(None).unwrap();
    /// This tells the X Server that we want to use XKB
    /// akin to libxkbcommon 'xkb_x11_setup_xkb_extension'
    /// returns a cookie which will tell us if xkb version is
    /// supported or not
    let cookie = xcb::xkb::use_extension(&conn, 
                            MIN_MAJOR_XKB_VERSION,
                            MIN_MINOR_XKB_VERSION);

}
