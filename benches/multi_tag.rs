#[macro_use]
extern crate bencher;
#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate nom_specialized;

use bencher::Bencher;
use nom::{IResult, Parser, Err, Needed, HexDisplay};
use nom::error::{Error, ErrorKind, ParseError};
use nom::branch::alt;
//use nom::bytes::streaming::tag;
use nom_specialized::combinators::tag_unrolled as tag;
use nom_specialized::avx::*;

fn nom_parser(i: &[u8]) -> IResult<&[u8], u8> {
    alt((
            tag(&b"Accept-Charset"[..]).map(|_| 0u8),
            tag(&b"Accept-Encoding"[..]).map(|_| 1u8),
            tag(&b"Accept"[..]).map(|_| 2u8),
            tag(&b"Authorization"[..]).map(|_| 3u8),

            tag(&b"Content-Encoding"[..]).map(|_| 4u8),
            tag(&b"Content-Length"[..]).map(|_| 5u8),
            tag(&b"Date"[..]).map(|_| 6u8),
            tag(&b"Expect"[..]).map(|_| 7u8),

            tag(&b"Forwarded"[..]).map(|_| 8u8),
            tag(&b"Host"[..]).map(|_| 9u8),
            tag(&b"If-Modified-Since"[..]).map(|_| 10u8),
            tag(&b"Referer"[..]).map(|_| 11u8),

            tag(&b"User-Agent"[..]).map(|_| 12u8),
            tag(&b"Upgrade"[..]).map(|_| 13u8),
            tag(&b"Via"[..]).map(|_| 14u8),
            tag(&b"X-Forwarded-For"[..]).map(|_| 15u8),
    ))(i)
}

fn naive(i: &[u8]) -> IResult<&[u8], u8> {
    let (i, h) = nom::bytes::streaming::take_while1(|c: u8| {
        nom::character::is_alphabetic(c) || c == b'-'
    })(i)?;

    let value = if h == &b"Accept-Charset"[..] {
        0u8
    } else if h == &b"Accept-Encoding"[..] {
        1u8
    } else if h == &b"Accept"[..] {
        2u8
    } else if h == &b"Authorization"[..] {
        3u8
    } else if h == &b"Content-Encoding"[..] {
        4u8
    } else if h == &b"Content-Length"[..] {
        5u8
    } else if h == &b"Date"[..] {
        6u8
    } else if h == &b"Expect"[..] {
        7u8
    } else if h == &b"Forwarded"[..] {
        8u8
    } else if h == &b"Host"[..] {
        9u8
    } else if h == &b"If-Modified-Since"[..] {
        10u8
    } else if h == &b"Referer"[..] {
        11u8
    } else if h == &b"User-Agent"[..] {
        12u8
    } else if h == &b"Upgrade"[..] {
        13u8
    } else if h == &b"Via"[..] {
        14u8
    } else if h == &b"X-Forwarded-For"[..] {
        15u8
    } else {
        return Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)));
    };

    Ok((i, value))
}

fn manual(i: &[u8]) -> IResult<&[u8], u8> {
    match i.get(0) {
        Some(b'A') => {
            match i.get(1) {
                Some(b'c') => {
                    // FIXME: not handling the Incomplete case
                    if (&i[2..]).starts_with(&b"cept"[..]) {
                        match i.get(7) {
                            Some(b'-') => {
                                alt((
                                        tag(&b"Charset"[..]).map(|_| 0u8),
                                        tag(&b"Encoding"[..]).map(|_| 1u8),
                                        ))(&i[8..])
                            },
                            _ => Ok((&i[6..], 2))
                        }
                    } else {
                        Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)))
                    }
                }
                Some(b'u') => {
                    tag(&b"thorization"[..]).map(|_| 3u8).parse(&i[2..])
                },
                Some(_) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag))),
                None => Err(Err::Incomplete(Needed::Unknown)),
            }
        },
        Some(b'C') => {
            if (&i[1..]).starts_with(&b"ontent-"[..]) {
                alt((
                        tag(&b"Encoding"[..]).map(|_| 4u8),
                        tag(&b"Length"[..]).map(|_| 5u8),
                        ))(&i[8..])
            } else {
                Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)))
            }
        },
        Some(b'D') => tag(&b"ate"[..]).map(|_| 6u8).parse(&i[1..]),
        Some(b'E') => tag(&b"xpect"[..]).map(|_| 7u8).parse(&i[1..]),
        Some(b'F') => tag(&b"Forwarded"[..]).map(|_| 8u8).parse(&i[1..]),
        Some(b'H') => tag(&b"Host"[..]).map(|_| 9u8).parse(&i[1..]),
        Some(b'I') => tag(&b"If-Modified-Since"[..]).map(|_| 10u8).parse(&i[1..]),
        Some(b'R') => tag(&b"Referer"[..]).map(|_| 11u8).parse(&i[1..]),
        Some(b'U') => alt((
                        tag(&b"ser-Agent"[..]).map(|_| 12u8),
                        tag(&b"pgrade"[..]).map(|_| 13u8),
                        ))(&i[1..]),
        Some(b'V') => tag(&b"Via"[..]).map(|_| 14u8).parse(&i[1..]),
        Some(b'X') => tag(&b"X-Forwarded-For"[..]).map(|_| 15u8).parse(&i[1..]),
        Some(_) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag))),
        None => Err(Err::Incomplete(Needed::Unknown)),
    }
}

lazy_static! {
        static ref RE: regex::bytes::RegexSet = regex::bytes::RegexSet::new(&[
               r"Accept-Charset",
               r"Accept-Encoding",
               r"Accept",
               r"Authorization",
               r"Content-Encoding",
               r"Content-Length",
               r"Date",
               r"Expect",
               r"Forwarded",
               r"Host",
               r"If-Modified-Since",
               r"Referer",
               r"User-Agent",
               r"Upgrade",
               r"Via",
               r"X-Forwarded-For",
        ]).unwrap();
}

fn re(i: &[u8]) -> IResult<&[u8], u8> {
    match RE.matches(i).iter().next() {
        Some(index) => Ok((i, index as u8)),
        None => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag))),
    }
}

fn avx_parser<'a>() -> impl Fn(&'a[u8]) -> IResult<&'a[u8], u8> {
    let parser1 = multitag::<Error<&'a[u8]>>(&[
        &b"Acce"[..], &b"Auth"[..], &b"Cont"[..], &b"Date"[..],
        &b"Expe"[..], &b"Forw"[..], &b"Host"[..], &b"If-M"[..]
    ]);
    let parser2 = multitag::<Error<&'a[u8]>>(&[
        &b"Refe"[..], &b"User"[..], &b"Upgr"[..], &b"Via:"[..],
        &b"X-Forwarded-For"[..],
    ]);

    move |i: &[u8]| {
        if let Ok((i, idx)) = parser1(i) {
            match idx {
                0 => alt((
                      tag(&b"pt-Charset"[..]).map(|_| 0u8),
                      tag(&b"pt-Encoding"[..]).map(|_| 1u8),
                      tag(&b"pt"[..]).map(|_| 2u8),
                    ))(i),
                1 => tag(&b"orization"[..]).map(|_| 3u8).parse(i),
                2 => alt((
                      tag(&b"ent-Encoding"[..]).map(|_| 4u8),
                      tag(&b"ent-Length"[..]).map(|_| 5u8),
                    ))(i),
                4 => Ok((i, 6u8)),
                5 => tag(&b"ct"[..]).map(|_| 7u8).parse(i),
                6 => tag(&b"arded"[..]).map(|_| 8u8).parse(i),
                7 => Ok((i, 9u8)),
                8 => tag(&b"odified-Since"[..]).map(|_| 10u8).parse(i),
                _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)))
            }
        } else if let Ok((i, idx)) = parser2(i) {
            //println!("got idx: {}, i: {}", idx, std::str::from_utf8(i).unwrap());
            match idx {
                0 => tag(&b"rer"[..]).map(|_| 11u8).parse(i),
                1 => tag(&b"-Agent"[..]).map(|_| 12u8).parse(i),
                2 => tag(&b"ade"[..]).map(|_| 13u8).parse(i),
                3 => Ok((i, 14u8)),
                4 => Ok((i, 15u8)),
                _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag))),
            }
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)))
        }
    }
}

fn multitag_Accept_nom(bench: &mut Bencher) {
    let input = b"Accept:";

    let res: IResult<_, _> = nom_parser(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 2)));

    bench.bytes = 6;
    bench.iter(|| nom_parser(&input[..]))
}

fn multitag_Accept_manual(bench: &mut Bencher) {
    let input = b"Accept:";

    let res: IResult<_, _> = manual(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 2)));

    bench.bytes = 6;
    bench.iter(|| manual(&input[..]))
}

fn multitag_Accept_naive(bench: &mut Bencher) {
    let input = b"Accept:";

    let res: IResult<_, _> = naive(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 2)));

    bench.bytes = 6;
    bench.iter(|| naive(&input[..]))
}

fn multitag_Accept_re(bench: &mut Bencher) {
    let input = b"Accept:";

    let res: IResult<_, _> = re(&input[..]);
    assert_eq!(res, Ok((&input[..], 2)));

    bench.bytes = 6;
    bench.iter(|| re(&input[..]))
}

fn multitag_Accept_avx(bench: &mut Bencher) {
    let parser = avx_parser();

    // the parser needs 16 bytes
    let input = b"Accept: ABCDER\r\n";

    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b": ABCDER\r\n"[..], 2)));

    bench.bytes = 6;
    bench.iter(|| parser(&input[..]))
}

fn multitag_Content_Length_nom(bench: &mut Bencher) {
    let input = b"Content-Length:";

    let res: IResult<_, _> = nom_parser(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 5)));

    bench.bytes = 14;
    bench.iter(|| nom_parser(&input[..]))
}

fn multitag_Content_Length_manual(bench: &mut Bencher) {
    let input = b"Content-Length:";

    let res: IResult<_, _> = manual(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 5)));

    bench.bytes = 14;
    bench.iter(|| manual(&input[..]))
}

fn multitag_Content_Length_naive(bench: &mut Bencher) {
    let input = b"Content-Length:";

    let res: IResult<_, _> = naive(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 5)));

    bench.bytes = 14;
    bench.iter(|| naive(&input[..]))
}

fn multitag_Content_Length_re(bench: &mut Bencher) {
    let input = b"Content-Length:";

    let res: IResult<_, _> = re(&input[..]);
    assert_eq!(res, Ok((&input[..], 5)));

    bench.bytes = 14;
    bench.iter(|| re(&input[..]))
}

fn multitag_Content_Length_avx(bench: &mut Bencher) {
    let parser = avx_parser();

    let input = b"Content-Length: ";

    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b": "[..], 5)));

    bench.bytes = 14;
    bench.iter(|| parser(&input[..]))
}

fn multitag_Upgrade_nom(bench: &mut Bencher) {
    let input = b"Upgrade:";

    let res: IResult<_, _> = nom_parser(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 13)));

    bench.bytes = 7;
    bench.iter(|| nom_parser(&input[..]))
}

fn multitag_Upgrade_manual(bench: &mut Bencher) {
    let input = b"Upgrade:";

    let res: IResult<_, _> = manual(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 13)));

    bench.bytes = 7;
    bench.iter(|| manual(&input[..]))
}

fn multitag_Upgrade_naive(bench: &mut Bencher) {
    let input = b"Upgrade:";

    let res: IResult<_, _> = naive(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 13)));

    bench.bytes = 7;
    bench.iter(|| naive(&input[..]))
}

fn multitag_Upgrade_re(bench: &mut Bencher) {
    let input = b"Upgrade:";

    let res: IResult<_, _> = re(&input[..]);
    assert_eq!(res, Ok((&input[..], 13)));

    bench.bytes = 7;
    bench.iter(|| re(&input[..]))
}

fn multitag_Upgrade_avx(bench: &mut Bencher) {
    let parser = avx_parser();
    let input = b"Upgrade: ABCDE\r\n";

    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b": ABCDE\r\n"[..], 13)));

    bench.bytes = 7;
    bench.iter(|| parser(&input[..]))
}

benchmark_group!(
    benches,
    multitag_Accept_nom,
    multitag_Accept_manual,
    multitag_Accept_naive,
    multitag_Accept_re,
    multitag_Accept_avx,
    multitag_Content_Length_nom,
    multitag_Content_Length_manual,
    multitag_Content_Length_naive,
    multitag_Content_Length_re,
    multitag_Content_Length_avx,
    multitag_Upgrade_nom,
    multitag_Upgrade_manual,
    multitag_Upgrade_naive,
    multitag_Upgrade_re,
    multitag_Upgrade_avx,
);
benchmark_main!(benches);
