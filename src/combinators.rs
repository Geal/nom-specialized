/* Copyright (C) 2020 Geoffroy Couprie */
use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult, Needed,
};

pub fn take_while0_unrolled<'a, F, Error: ParseError<&'a [u8]>>(
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

        if i == len {
            Err(Err::Incomplete(Needed::Unknown))
        } else {
            let (prefix, suffix) = input.split_at(i);
            Ok((suffix, prefix))
        }
    }
}

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
#[cfg(feature = "sse2")]
pub fn take_while0_sse2<'a, 'b: 'a, F>(
    predicate: F,
    ranges: &'b [u8],
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8], ()>
where
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

        if i == input.len() {
            Err(Err::Incomplete(Needed::Unknown))
        } else {
            let (prefix, suffix) = input.split_at(i);
            Ok((suffix, prefix))
        }
    }
}

#[inline(always)]
#[cfg(feature = "sse2")]
pub fn take_while1_sse2<'a, 'b: 'a, F, Error>(
    predicate: F,
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

pub fn tag_unrolled<'a, 'b: 'a, Error: ParseError<&'a [u8]>>(
    tag: &'b[u8]
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8], Error>
{
    move |input: &'a [u8]| {
        let mut i = 0usize;
        let len = std::cmp::min(tag.len(), input.len());
        let mut found = false;

        loop {
            if len - i < 8 {
                break;
            }

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;

            if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                found = true;
                break;
            }
            i = i + 1;
        }

        if !found {
            loop {
                if unsafe { *tag.get_unchecked(i) != *input.get_unchecked(i) } {
                    break;
                }
                i = i + 1;
                if i == len {
                    break;
                }
            }
        }

        if i == tag.len() {
            let (prefix, suffix) = input.split_at(i);
            Ok((suffix, prefix))
        } else {
            if input.len() > i {
                Err(Err::Error(Error::from_error_kind(input, ErrorKind::Tag)))
            } else {
                Err(Err::Incomplete(Needed::new(tag.len() - i)))
            }
        }
    }
}

#[inline(always)]
#[cfg(feature = "sse2")]
pub fn tag_sse2<'a, 'b: 'a, Error: ParseError<&'a [u8]>>(
    tag: &'b[u8],
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8], Error>
{
    move |input: &'a [u8]| {
        use std::arch::x86_64::{
            _mm_cmpestri, _mm_loadu_si128, _SIDD_CMP_EQUAL_EACH, _SIDD_LEAST_SIGNIFICANT,
            _SIDD_UBYTE_OPS, _SIDD_NEGATIVE_POLARITY,
        };

        let mut found = false;

        let mut index = 0;
        loop {
            let current_tag = &tag[index..];
            let current_tag_len = std::cmp::min(current_tag.len(), 16) as i32;
            let current_slice = &input[index..];
            let current_slice_len = std::cmp::min(current_slice.len(), 16) as i32;

            let idx = unsafe {
                _mm_cmpestri(
                    _mm_loadu_si128(current_tag.as_ptr() as *const _),
                    current_tag_len,
                    _mm_loadu_si128(current_slice.as_ptr() as *const _),
                    current_slice_len,
                    _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_EQUAL_EACH | _SIDD_UBYTE_OPS | _SIDD_NEGATIVE_POLARITY,
                    )
            };

            index += idx as usize;
            if idx < 16 {
                found = true;
                break;
            }

            if current_tag.len() <= 16 || current_slice.len() < 16 {
                break;
            }
        }

        if index == tag.len() {
            Ok((&input[index..], &input[..index]))
        } else {
            if found {
                Err(Err::Error(Error::from_error_kind(input, ErrorKind::Tag)))
            } else {
                Err(Err::Incomplete(Needed::new(tag.len() - index)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "sse2")]
    fn simd_test() {
        use std::str::from_utf8;
        fn is_token(c: u8) -> bool {
            c > 0x20 && c < 0x7F
        }

        let range = b"\0 \x7F\x7F";
        let input = b"/abcd/efgh/ijkl/pouet/ 1234579";
        let input = b"/abcd/efgh/ij kl/pouet/ 1234579";
        let res: IResult<&[u8], &[u8]> = take_while1_sse2(is_token, range)(input);

        let (i, o) = res.unwrap();
        assert_eq!(from_utf8(i).unwrap(), " kl/pouet/ 1234579");
        assert_eq!(from_utf8(o).unwrap(), "/abcd/efgh/ij");
    }

    #[test]
    #[cfg(feature = "sse2")]
    fn tag_simd_test() {
        use std::arch::x86_64::{
            _mm_cmpestri, _mm_cmpestrm, _mm_loadu_si128, _SIDD_CMP_EQUAL_EACH, _SIDD_LEAST_SIGNIFICANT,
            _SIDD_UBYTE_OPS, _SIDD_NEGATIVE_POLARITY
        };

        let tag1 = "ABCDABCDABCDABCD";
        let tag2 = "ABCDABCDABCD";
        let tag3 = "ABCDABCDABCDABCDEFGH";

        let slice1 = "ABCDABCDABCDABCD";
        let slice2 = "ABcdabcdabcdabcd";
        let slice3 = "ABCDabcd";
        let slice4 = "ABCDABCDABCDABCDABCD";

        let idx = unsafe {
            println!("tag1-> {:?}", _mm_loadu_si128(tag1.as_ptr() as *const _));
            _mm_cmpestri(
                _mm_loadu_si128(tag1.as_ptr() as *const _),
                tag1.len() as i32,
                _mm_loadu_si128(slice1.as_ptr() as *const _),
                slice1.len() as i32,
                _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_EQUAL_EACH | _SIDD_UBYTE_OPS | _SIDD_NEGATIVE_POLARITY,
            )
        };
        println!("comparing tag \"{}\" to input \"{}\" gave {:?}, should be {}",
          tag1, slice1, idx, 16);

        let idx = unsafe {
            _mm_cmpestri(
                _mm_loadu_si128(tag1.as_ptr() as *const _),
                tag1.len() as i32,
                _mm_loadu_si128(slice2.as_ptr() as *const _),
                slice2.len() as i32,
                _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_EQUAL_EACH | _SIDD_UBYTE_OPS | _SIDD_NEGATIVE_POLARITY,
            )
        };
        println!("comparing tag \"{}\" to input \"{}\" gave {:?}, should be {}",
          tag1, slice2, idx, 2);

        let idx = unsafe {
            _mm_cmpestri(
                _mm_loadu_si128(tag2.as_ptr() as *const _),
                tag2.len() as i32,
                _mm_loadu_si128(slice1.as_ptr() as *const _),
                slice1.len() as i32,
                _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_EQUAL_EACH | _SIDD_UBYTE_OPS | _SIDD_NEGATIVE_POLARITY,
            )
        };
        println!("comparing tag \"{}\" to input \"{}\" gave {:?}, should be {}",
          tag2, slice1, idx, 12);

        let idx = unsafe {
            _mm_cmpestri(
                _mm_loadu_si128(tag3.as_ptr() as *const _),
                tag3.len() as i32,
                _mm_loadu_si128(slice3.as_ptr() as *const _),
                slice3.len() as i32,
                _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_EQUAL_EACH | _SIDD_UBYTE_OPS | _SIDD_NEGATIVE_POLARITY,
            )
        };
        println!("comparing tag \"{}\" to input \"{}\" gave {:?}, should be {}",
          tag3, slice3, idx, 4);

        let idx = unsafe {
            _mm_cmpestri(
                _mm_loadu_si128(tag3.as_ptr() as *const _),
                tag3.len() as i32,
                _mm_loadu_si128(slice4.as_ptr() as *const _),
                slice4.len() as i32,
                _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_EQUAL_EACH | _SIDD_UBYTE_OPS | _SIDD_NEGATIVE_POLARITY,
            )
        };
        println!("comparing tag \"{}\" to input \"{}\" gave {:?}, should be {}",
          tag3, slice4, idx, 16);
    }
}
