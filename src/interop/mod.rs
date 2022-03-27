//! Interop

use core::mem;
use core::ptr;
use core::slice;

use crate::bindings;
use embedded_hal::{blocking, digital};

mod alloc;
pub use alloc::*;

mod delay;
pub use delay::*;

mod print;

mod errno {
    // Simple implementation of errno
    static ERRNO: cty::c_int = 0;
    #[no_mangle]
    pub extern "C" fn __errno() -> *const cty::c_int {
        &ERRNO
    }
}

cpp! {{
    extern "C" {
        // Needed to get <stdbool.h> in common.h
        #define __STDC_VERSION__ 199901

        #include "ad9361.h"
        #include "ad9361_api.h"

        int32_t ad9361_hdl_loopback(struct ad9361_rf_phy *phy, bool enable)
        {
            (void)phy;
            (void)enable;

            return 0;
        }
    }
}}

/// Calculate the quotient and the remainder of an integer division.
///
/// The quotient is placed in the first parameter, the remainder is returned.
///
/// extern "C" uint64_t do_div(uint64_t* n,
///   uint64_t base);
#[no_mangle]
pub extern "C" fn do_div(n: *mut u64, base: u64) -> u64 {
    unsafe {
        let modulus: u64 = *n % base;
        *n /= base;

        modulus
    }
}

// -------- Digital Tune --------

/// * Digital tune.
/// * @param phy The AD9361 state structure.
/// * @param max_freq Maximum frequency.
/// * @param flags Flags: BE_VERBOSE, BE_MOREVERBOSE, DO_IDELAY, DO_ODELAY.
/// * @return 0 in case of success, negative error code otherwise.
/// */
/// int32_t ad9361_dig_tune(struct ad9361_rf_phy *phy, uint32_t max_freq,
///   enum dig_tune_flags flags)
#[no_mangle]
pub extern "C" fn ad9361_dig_tune(
    _phy: bindings::ad9361_rf_phy,
    _max_freq: u32,
    _flags: bindings::dig_tune_flags,
) -> i32 {
    0 // Not implemented
}

// -------- SPI --------

/// Wrapper method for SPI transfer calls
///
/// During initialisation, we create pointers to the specialised versions of
/// this wrapper method
pub fn spi_wr_method<SPI: blocking::spi::Transfer<u8>>(
    outer: &mut SPI,
    data: &mut [u8],
) -> i32 {
    match outer.transfer(data) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// int32_t spi_init(struct spi_desc **desc,
///   const struct spi_init_param *param);
#[no_mangle]
pub extern "C" fn spi_init(
    desc: *mut *const bindings::spi_desc,
    param: *const bindings::spi_init_param,
) -> i32 {
    // We expect initialisation to be completed already

    // Use spi_init_param as the spi_desc
    unsafe {
        // Structures must have the same layout
        *desc = param as *const bindings::spi_desc;
    }

    0
}

/// int32_t spi_write_and_read(struct spi_desc *desc,
///   uint8_t *data,
///   uint16_t bytes_number);
#[no_mangle]
pub extern "C" fn spi_write_and_read(
    descriptor: *mut bindings::spi_desc,
    data: *mut u8,
    number_of_bytes: u16,
) -> i32 {
    // Unpack
    let (f_ptr, slf, bytes) = unsafe {
        // Function Pointer
        let f_ptr: fn(&mut (), &mut [u8]) -> i32 =
            mem::transmute((*descriptor).platform_ops);
        // Self
        let slf: &mut () = &mut *((*descriptor).extra as *mut _);
        // Slice
        let bytes = slice::from_raw_parts_mut(data, number_of_bytes as usize);

        (f_ptr, slf, bytes)
    };

    // Call function pointer
    f_ptr(slf, bytes)
}

/// int32_t spi_remove(struct spi_desc *desc);
#[no_mangle]
pub extern "C" fn spi_remove(_descriptor: *mut bindings::spi_desc) -> i32 {
    0 // Not implemented
}

// -------- GPIO --------

/// Wrapper method for SPI transfer calls
///
/// During initialisation, we create pointers to the specialised versions of
/// this wrapper method
pub fn gpio_set_method<GPIO: digital::v2::OutputPin>(
    outer: &mut GPIO,
    value: u8,
) -> i32 {
    let result = match value {
        0 => outer.set_low(),
        _ => outer.set_high(),
    };
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// int32_t gpio_get(struct gpio_desc **desc,
///   const struct gpio_init_param *param);
#[no_mangle]
pub extern "C" fn gpio_get(
    desc: *mut *const bindings::gpio_desc,
    param: *const bindings::gpio_init_param,
) -> i32 {
    // Use gpio_init_param as the gpio_desc
    unsafe {
        if (*param).number < 0 {
            *desc = ptr::null();
        } else {
            // Structures must have the same layout
            *desc = param as *const bindings::gpio_desc;
        }
    }
    0
}

/// int32_t gpio_get_optional(struct gpio_desc **desc,
///   const struct gpio_init_param *param);
#[no_mangle]
pub extern "C" fn gpio_get_optional(
    desc: *mut *const bindings::gpio_desc,
    param: *const bindings::gpio_init_param,
) -> i32 {
    // Use gpio_init_param as the gpio_desc
    unsafe {
        if (*param).number < 0 {
            *desc = ptr::null();
        } else {
            // Structures must have the same layout
            *desc = param as *const bindings::gpio_desc;
        }
    }
    0
}

/// int32_t gpio_direction_input(struct gpio_desc *desc,
///   uint8_t value);
#[no_mangle]
pub extern "C" fn gpio_direction_input(
    _desc: *mut bindings::gpio_desc,
    _value: u8,
) -> i32 {
    0 // Assume inputs configured already
}

/// int32_t gpio_direction_output(struct gpio_desc *desc,
///   uint8_t value);
#[no_mangle]
pub extern "C" fn gpio_direction_output(
    _desc: *mut bindings::gpio_desc,
    _value: u8,
) -> i32 {
    0 // Assume outputs configured already
}

/// int32_t gpio_set_value(struct gpio_desc *desc,
///   uint8_t value);
#[no_mangle]
pub extern "C" fn gpio_set_value(
    descriptor: *mut bindings::gpio_desc,
    value: u8,
) -> i32 {
    let descriptor = unsafe { *descriptor };

    // Unpack
    let (f_ptr, slf) = unsafe {
        // Function Pointer
        let f_ptr: fn(&mut (), u8) -> i32 =
            mem::transmute(descriptor.platform_ops);
        // Self
        let slf: &mut () = &mut *(descriptor.extra as *mut _);

        (f_ptr, slf)
    };

    if (slf as *mut ()).is_null() {
        trace!("set_value! {} = {} (unconnected)", descriptor.number, value);
        0
    } else {
        trace!("set_value! {} = {}", descriptor.number, value);
        f_ptr(slf, value)
    }
}

/// int32_t gpio_get_value(struct gpio_desc *desc,
///   uint8_t *value);
#[no_mangle]
pub extern "C" fn gpio_get_value(
    descriptor: *mut bindings::gpio_desc,
    value: *mut u8,
) -> i32 {
    let descriptor = unsafe { *descriptor };

    trace!("get_value! {}", descriptor.number);

    unsafe {
        (*value) = 0; // Not implemented
    }
    0
}

/// int32_t gpio_remove(struct gpio_desc *desc);
#[no_mangle]
pub extern "C" fn gpio_remove(_descriptor: *mut bindings::gpio_desc) -> i32 {
    0 // Not implemented
}
