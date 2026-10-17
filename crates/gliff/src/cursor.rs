//! Hiding the local pointer while a headless session's frames already
//! carry the remote cursor. Goes through `NSCursor`'s reference-counted
//! hide/unhide: the count is per-application and the window server drops
//! it when the app exits, so a crash or a missed unhide can't strand the
//! user without a pointer. A transparent widget cursor does not work —
//! macOS never applies it, and resets widget cursors to the arrow while
//! a button is held.
#![cfg(target_os = "macos")]

use std::ffi::{c_char, c_void};

type Id = *mut c_void;
type Sel = *const c_void;

#[link(name = "objc", kind = "dylib")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> Id;
    fn sel_registerName(name: *const c_char) -> Sel;
    fn objc_msgSend(receiver: Id, sel: Sel);
}

/// Hide the pointer until a matching [`unhide`]; nested hides nest.
pub fn hide() {
    // SAFETY: +[NSCursor hide] takes no arguments and returns void.
    unsafe {
        objc_msgSend(
            objc_getClass(c"NSCursor".as_ptr()),
            sel_registerName(c"hide".as_ptr()),
        )
    }
}

/// Undo one [`hide`].
pub fn unhide() {
    // SAFETY: +[NSCursor unhide] takes no arguments and returns void.
    unsafe {
        objc_msgSend(
            objc_getClass(c"NSCursor".as_ptr()),
            sel_registerName(c"unhide".as_ptr()),
        )
    }
}
