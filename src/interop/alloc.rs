//! Special-purpose allocator
//!
//! This module is a special-purpose allocator for the adi no-os ad9361 driver.
//!
//! Allocations shorter than 8 bytes are made from a scratchpad. Only one
//! allocation is made at a time.
//!
//! Larger allocations are made from a heap. Only the most recent allocation is
//! mutable, while previous allocations are immutable. The only exception is
//! that freeing the allocation at the very start of the heap results in all
//! allocations being freed.
//!
//! The allocator is *not* re-entrant.
//!
//! The behaviour of this allocator is verified against the ad9361 driver by
//! test.

use core::mem::MaybeUninit;
use core::ptr;

static mut HEAP_START: *mut u32 = ptr::null_mut();
static mut HEAP_TOP: *mut u32 = ptr::null_mut();
static mut HEAP_PREVIOUS: *mut u32 = ptr::null_mut();
static mut HEAP_END: *mut u32 = ptr::null_mut();

static mut SCRATCHPAD: MaybeUninit<[u8; 8]> = MaybeUninit::uninit();
static mut SCRATCHPAD_ALLOCATED: u8 = 0;

pub unsafe fn init_admalloc(heap_start: *mut u32, heap_len: usize) {
    HEAP_START = heap_start;
    HEAP_TOP = HEAP_START;
    HEAP_END = heap_start.add(heap_len);
    SCRATCHPAD_ALLOCATED = 0;
}

#[no_mangle]
pub unsafe extern "C" fn admalloc(size: usize) -> *mut u32 {
    if size < 8 {
        // allocate from scratchpad
        debug_assert!(
            SCRATCHPAD_ALLOCATED == 0,
            "AD936x: attempt to double-allocate scratchpad"
        );
        SCRATCHPAD_ALLOCATED = 1;
        SCRATCHPAD.as_mut_ptr() as *mut _
    } else {
        // allocate from heap
        assert!(!HEAP_TOP.is_null(), "AD936x: admalloc was not initialized");

        let words = (size + 3) / 4;
        HEAP_PREVIOUS = HEAP_TOP;
        HEAP_TOP = HEAP_TOP.add(words);
        assert!(
            HEAP_TOP.offset_from(HEAP_END) <= 0,
            "AD936x: Heap exhausted, memory allocation failed"
        );

        debug!("AD936x: allocated {} bytes in {} words", size, words);

        HEAP_PREVIOUS
    }
}
#[no_mangle]
pub unsafe extern "C" fn adcalloc(nmemb: usize, size: usize) -> *mut u32 {
    let mem = admalloc(nmemb * size);

    ptr::write_bytes(mem, 0, nmemb * size);
    mem
}
#[no_mangle]
pub unsafe extern "C" fn adfree(ptr: *mut u32) {
    if ptr.is_null() {
        warn!("AD936x: Tried to free null pointer");
    } else if ptr == SCRATCHPAD.as_mut_ptr() as *mut _ {
        SCRATCHPAD_ALLOCATED = 0;
    } else if ptr == HEAP_START {
        // deallocate everything
        HEAP_TOP = HEAP_START;

        debug!("AD936x: deallocated everything");
    } else if ptr == HEAP_PREVIOUS {
        // deallocate last allocation
        HEAP_TOP = HEAP_PREVIOUS;

        debug!("AD936x: deallocated last allocation");
    }
}
