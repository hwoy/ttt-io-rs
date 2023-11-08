extern crate ttt_io_sys;

use std::os::raw::{c_char, c_int, c_uint};

pub fn getch(bsize: u32, dkey: char) -> char {
    let mut buffer = std::iter::repeat(0u8)
        .take(bsize as usize)
        .collect::<Box<[u8]>>();
    unsafe {
        ttt_io_sys::ox_getch(
            buffer.as_mut_ptr() as *mut c_char,
            bsize as c_int,
            dkey as c_char,
        ) as u8 as char
    }
}

pub fn printttable_squar(val1: u32, val2: u32, ch1: char, ch2: char, square: &str, blank: char) {
    use std::ffi::CString;
    let square = CString::new(square).unwrap();
    unsafe {
        ttt_io_sys::ox_printtable(
            val1 as c_uint,
            val2 as c_uint,
            ch1 as c_char,
            ch2 as c_char,
            square.as_ptr() as *const c_char,
            blank as c_char,
        )
    }
}

#[cfg(feature = "builtin_table")]
pub static SQUARE: &'static str = "\
+---------------+---------------+---------------+\n\
|		|		|		|\n\
|		|		|		|\n\
|	%c	|	%c	|	%c	|\n\
|		|		|		|\n\
|		|		|		|\n\
+---------------+---------------+---------------+\n\
|		|		|		|\n\
|		|		|		|\n\
|	%c	|	%c	|	%c	|\n\
|		|		|		|\n\
|		|		|		|\n\
+---------------+---------------+---------------+\n\
|		|		|		|\n\
|		|		|		|\n\
|	%c	|	%c	|	%c	|\n\
|		|		|		|\n\
|		|		|		|\n\
+---------------+---------------+---------------+\n";

#[cfg(feature = "builtin_table")]
pub fn printttable(val1: u32, val2: u32, ch1: char, ch2: char, blank: char) {
    crate::printttable_squar(val1, val2, ch1, ch2, SQUARE, blank);
}
