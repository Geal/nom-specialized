/* Copyright (C) 2020 Geoffroy Couprie */
use nom::{
    error::{ErrorKind, Error, ParseError},
    Err, IResult, InputTakeAtPosition, Needed,
};

pub fn take_while1_unrolled<'a, F, Error: ParseError<&'a [u8]>>(
    cond: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8], Error>
where
    F: Fn(u8) -> bool,
{
    move |input: &'a [u8]| {
        let mut i = 0usize;
        let len = input.len();
        let mut found = false;

        loop {
            if len - i < 8 {
                break;
            }

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }
            i = i + 1;

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }
            i = i + 1;

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }
            i = i + 1;

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }
            i = i + 1;

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }
            i = i + 1;

            if !cond(unsafe { *input.get_unchecked(i) }) {
                found = true;
                break;
            }
            i = i + 1;
        }

        if !found {
            loop {
                if !cond(unsafe { *input.get_unchecked(i) }) {
                    break;
                }
                i = i + 1;
                if i == len {
                    break;
                }
            }
        }

        if i == 0 {
            Err(Err::Error(Error::from_error_kind(
                input,
                ErrorKind::TakeWhile1,
            )))
        } else if i == len {
            Err(Err::Incomplete(Needed::Unknown))
        } else {
            let (prefix, suffix) = input.split_at(i);
            Ok((suffix, prefix))
        }
    }
}

#[inline(always)]
pub fn take_while1_sse2<'a, 'b: 'a, F, Error>(
    mut predicate: F,
    ranges: &'b [u8],
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8], Error>
where
    Error: ParseError<&'a [u8]>,
    F: Fn(u8) -> bool,
{
    move |input: &'a [u8]| {
        use std::arch::x86_64::{
            _mm_cmpestri, _mm_loadu_si128, _SIDD_CMP_RANGES, _SIDD_LEAST_SIGNIFICANT,
            _SIDD_UBYTE_OPS,
        };

        let start = input.as_ptr() as usize;
        let mut i = input.as_ptr() as usize;
        let mut left = input.len();
        let mut found = false;

        if left >= 16 {
            let ranges16 = unsafe { _mm_loadu_si128(ranges.as_ptr() as *const _) };
            let ranges_len = ranges.len() as i32;
            loop {
                let sl = unsafe { _mm_loadu_si128(i as *const _) };

                let idx = unsafe {
                    _mm_cmpestri(
                        ranges16,
                        ranges_len,
                        sl,
                        16,
                        _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_RANGES | _SIDD_UBYTE_OPS,
                    )
                };

                if idx != 16 {
                    i += idx as usize;
                    found = true;
                    break;
                }

                i += 16;
                left -= 16;

                if left < 16 {
                    break;
                }
            }
        }

        let mut i = i - start;
        if !found {
            loop {
                if !predicate(unsafe { *input.get_unchecked(i) }) {
                    break;
                }
                i = i + 1;
                if i == input.len() {
                    break;
                }
            }
        }

        if i == 0 {
            Err(Err::Error(Error::from_error_kind(
                input,
                ErrorKind::TakeWhile1,
            )))
        } else if i == input.len() {
            Err(Err::Incomplete(Needed::Unknown))
        } else {
            let (prefix, suffix) = input.split_at(i);
            Ok((suffix, prefix))
        }
    }
}

#[test]
#[cfg(feature = "simd")]
fn simd_test() {
    fn is_token(c: u8) -> bool {
        c > 0x20 && c < 0x7F
    }

    let range = b"\0 \x7F\x7F";
    let input = b"/abcd/efgh/ijkl/pouet/ 1234579";
    let input = b"/abcd/efgh/ij kl/pouet/ 1234579";
    let res: IResult<&[u8], &[u8]> = take_while1_simd!(input, is_token, range);

    let (i, o) = res.unwrap();
    assert_eq!(from_utf8(i).unwrap(), " kl/pouet/ 1234579");
    assert_eq!(from_utf8(o).unwrap(), "/abcd/efgh/ij");
}
