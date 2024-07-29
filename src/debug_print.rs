use std::io::{stdout, Write};

pub fn debug_print(debug: i8, to_print: &str) {
    if debug == 1 {
        let _ = stdout().flush();
        println!("DEBUG: {}", to_print.to_string());
        let _ = stdout().flush();
    } 
}

pub static DEBUG: i8 = 1;

// NOTE: to use debug_print:
// use crate::{debug_print, debug_print::DEBUG};