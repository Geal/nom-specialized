#[macro_use]
extern crate bencher;
extern crate nom;
extern crate nom_specialized;

use bencher::Bencher;
use nom::{IResult, Parser, Err, Needed};
use nom::error::{Error, ErrorKind, ParseError};
use nom::bytes::streaming::tag;

fn nom_parser(i: &[u8]) -> IResult<&[u8], u8> {
    nom::branch::alt((
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
    let mut index = 0usize;

    match i.get(0) {
        Some(b'A') => {
            match i.get(1) {
                Some(b'c') => {
                    // FIXME: not handling the Incomplete case
                    if (&i[2..]).starts_with(&b"cept"[..]) {
                        match i.get(7) {
                            Some(b'-') => {
                                nom::branch::alt((
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
                nom::branch::alt((
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
        Some(b'U') => nom::branch::alt((
                        tag(&b"ser-Agent"[..]).map(|_| 12u8),
                        tag(&b"pgrade"[..]).map(|_| 13u8),
                        ))(&i[1..]),
        Some(b'V') => tag(&b"Via"[..]).map(|_| 14u8).parse(&i[1..]),
        Some(b'X') => tag(&b"X-Forwarded-For"[..]).map(|_| 15u8).parse(&i[1..]),
        Some(_) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag))),
        None => Err(Err::Incomplete(Needed::Unknown)),
    }

}

fn multitag_Accept_nom(bench: &mut Bencher) {
    let input = b"Accept:";

    let res: IResult<_, _> = nom_parser(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 2)));

    bench.bytes = 6;
    bench.iter(|| nom_parser(&input[..]))
}

fn multitag_Accept_naive(bench: &mut Bencher) {
    let input = b"Accept:";

    let res: IResult<_, _> = naive(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 2)));

    bench.bytes = 6;
    bench.iter(|| naive(&input[..]))
}

fn multitag_Content_Length_nom(bench: &mut Bencher) {
    let input = b"Content-Length:";

    let res: IResult<_, _> = nom_parser(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 5)));

    bench.bytes = 14;
    bench.iter(|| nom_parser(&input[..]))
}

fn multitag_Content_Length_naive(bench: &mut Bencher) {
    let input = b"Content-Length:";

    let res: IResult<_, _> = naive(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 5)));

    bench.bytes = 14;
    bench.iter(|| naive(&input[..]))
}

fn multitag_Upgrade_nom(bench: &mut Bencher) {
    let input = b"Upgrade:";

    let res: IResult<_, _> = nom_parser(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 13)));

    bench.bytes = 7;
    bench.iter(|| nom_parser(&input[..]))
}

fn multitag_Upgrade_naive(bench: &mut Bencher) {
    let input = b"Upgrade:";

    let res: IResult<_, _> = naive(&input[..]);
    assert_eq!(res, Ok((&b":"[..], 13)));

    bench.bytes = 7;
    bench.iter(|| naive(&input[..]))
}
benchmark_group!(
    benches,
    multitag_Accept_nom,
    multitag_Accept_naive,
    multitag_Content_Length_nom,
    multitag_Content_Length_naive,
    multitag_Upgrade_nom,
    multitag_Upgrade_naive,
);
benchmark_main!(benches);
