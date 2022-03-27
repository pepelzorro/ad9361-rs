//! Delay methods

use core::mem;
use core::ptr;

use embedded_hal::blocking;

// During initialisation, we create pointers to the specialised versions of
// these wrapper methods

/// Wrapper method for millisecond delay
pub fn delay_ms_method<DELAY: blocking::delay::DelayMs<u32>>(
    outer: &mut DELAY,
    delay: u32,
) {
    outer.delay_ms(delay);
}
/// Wrapper method for microsecond delay
pub fn delay_us_method<DELAY: blocking::delay::DelayUs<u32>>(
    outer: &mut DELAY,
    delay: u32,
) {
    outer.delay_us(delay);
}

// Static pointers to the most recently initialised delay object

pub static mut DELAY_US: *mut () = ptr::null_mut();
pub static mut DELAY_MS: *mut () = ptr::null_mut();
pub static mut DELAY_OBJECT: *mut () = ptr::null_mut();

/// void mdelay(uint32_t msecs);
#[no_mangle]
pub extern "C" fn mdelay(delay: u32) {
    trace!("delay_ms! {}", delay);

    unsafe {
        assert!(!DELAY_MS.is_null());
        assert!(!DELAY_OBJECT.is_null());

        let method: fn(&mut (), u32) = mem::transmute(DELAY_MS);
        method(&mut *DELAY_OBJECT, delay);
    }
}

/// void udelay(uint32_t usecs);
#[no_mangle]
pub extern "C" fn udelay(delay: u32) {
    trace!("delay_us! {}", delay);

    unsafe {
        assert!(!DELAY_US.is_null());
        assert!(!DELAY_OBJECT.is_null());

        let method: fn(&mut (), u32) = mem::transmute(DELAY_US);
        method(&mut *DELAY_OBJECT, delay);
    }
}
