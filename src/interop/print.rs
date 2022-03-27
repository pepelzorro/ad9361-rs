//! Printing methods

use core::slice;
use core::str;

/// Write the char to stdout
#[no_mangle]
pub extern "C" fn putchar(c: cty::c_int) {
    info!("{}", c as u8 as char);
}

/// Write the given string to stdout, appending a newline.
#[no_mangle]
pub extern "C" fn puts(s: *const cty::c_char) {
    let slice = unsafe {
        let len = strlen::strlen(s);
        let ptr = s as *const u8;
        slice::from_raw_parts(ptr, len as usize + 1)
    };
    info!("{}", str::from_utf8(slice).unwrap().trim());
}

pub(crate) mod strlen {
    cpp! {{
        #include <string.h>
        #include <stdint.h>
        #include <limits.h>

        #define ALIGN (sizeof(size_t))
        #define ONES ((size_t)-1/UCHAR_MAX)
        #define HIGHS (ONES * (UCHAR_MAX/2+1))
        #define HASZERO(x) (((x)-ONES) & ~(x) & HIGHS)
    }}

    // A strlen implementation
    pub unsafe fn strlen(string: *const cty::c_char) -> usize {
        cpp! ([string as "char *"] -> usize as "size_t" {
            const char *s = string;
            const char *a = s;
            const size_t *w;
            for (; (uintptr_t)s % ALIGN; s++) if (!*s) return s-a;
            for (w = (const size_t *)s; !HASZERO(*w); w++);
            for (s = (const char *)w; *s; s++);
            return s-a;
        })
    }
}
