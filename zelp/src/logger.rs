use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::sync::Once;

/// Ref: https://www.sitepoint.com/rust-global-variables/
static mut DEBUG: Option<bool> = None;
static INIT: Once = Once::new();

pub fn set_debug_flag<'a>(debug_flag: Option<bool>) -> bool {
    INIT.call_once(|| {
        // Since this access is inside a call_once, before any other accesses, it is safe
        unsafe {
            *DEBUG.borrow_mut() = Some(debug_flag.unwrap());
        }
    });
    // As long as this function is the only place with access to the static variable,
    // giving out a read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    unsafe { DEBUG.unwrap() }
}

fn debug_flag<'a>() -> bool {
    unsafe { DEBUG.unwrap() }
}

pub fn debug<T: Debug + ?Sized>(s: &T) {
    if debug_flag() {
        println!("{:?}", s);
    }
}
