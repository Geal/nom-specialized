//! Lookup table helpers
//!
//! when using the `take_while*` combinators, performance can be improved
//! by replacing the matching function with a lookup table. However,
//! writing those tables by hand is cumbersome, so we can instead use
//! the  [make_lookup_table] macro to generate it at compile time.
/* Copyright (C) 2020 Geoffroy Couprie */

/*
 * this code oes not work yet because of this error: error[E0658]: function pointers cannot appear in constant functions. see https://github.com/rust-lang/rust/issues/57563 the feature can be activated by #![feature(const_fn_fn_ptr_basics)]
const fn make_lookup_table(f: fn(u8) -> bool) -> [bool; 256] {
    let mut array = [false; 256];

    let mut i = 0u8;
    while i <= 255 {
        array[i as usize] = f(i);
        i += 1;
    }

    array
}
*/

#[macro_export]
macro_rules! make_lookup_table (
  ($f: expr) => ({
    let mut array = [false; 256];

    let mut i = 0u16;
    while i <= 255 {
        array[i as usize] = $f(i as u8);
        i += 1;
    }

    array
  })
);

#[cfg(test)]
mod tests {
    const fn is_header_value_token(c: u8) -> bool {
        return c == '\t' as u8 || (c > 31 && c != 127);
    }
    const LOOKUP_TABLE: [bool; 256] = make_lookup_table!(is_header_value_token);
    //const LOOKUP_TABLE: [bool; 256] = make_lookup_table(is_header_value_token);
    #[test]
    fn print() {
        //panic!("LOOKUP_TABLE:\n{:#?}", LOOKUP_TABLE);
    }
}
