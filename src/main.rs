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

extern crate xkbcommon;
extern crate xcb;
use xkbcommon::xkb;
use xcb;
use xkb::x11::ffi as XKBCONST;
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
    
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let first_xkb_event: u8 = 0;
  
    xkb::x11::setup_xkb_extension(conn, 
                                  xkb::x11::XKB_X11_MIN_MAJOR_XKB_VERSION,
                                  xkb::x11::XKB_X11_MIN_MINOR_XKB_VERSION,
                                  xkb::x11::XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS,
                                  0, 0, &first_xkb_event, 0);
    let device_id = xkb::x11::get_core_keyboard_device_id(&conn);    

   
}
