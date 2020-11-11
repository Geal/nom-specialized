#[macro_use]
extern crate bencher;
extern crate nom;
extern crate nom_specialized;

use bencher::Bencher;
use nom::IResult;
use nom::error::Error;

fn tag_4_nom(bench: &mut Bencher) {
    let input = b"ABCDABCDABCDABCDabcd";

    let parser = nom::bytes::streaming::tag(&b"ABCD"[..]);
    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b"ABCDABCDABCDabcd"[..], &b"ABCD"[..])));

    bench.bytes = 4;
    bench.iter(|| parser(&input[..]))
}

fn tag_4_sse2(bench: &mut Bencher) {
    let input = b"ABCDABCDABCDABCDabcd";

    let parser = nom_specialized::combinators::tag_sse2(&b"ABCD"[..]);
    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b"ABCDABCDABCDabcd"[..], &b"ABCD"[..])));

    bench.bytes = 4;
    bench.iter(|| parser(&input[..]))
}

fn tag_16_nom(bench: &mut Bencher) {
    let input = b"ABCDABCDABCDABCDabcd";

    let parser = nom::bytes::streaming::tag(&b"ABCDABCDABCDABCD"[..]);
    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b"abcd"[..], &b"ABCDABCDABCDABCD"[..])));

    bench.bytes = 16;
    bench.iter(|| parser(&input[..]))
}

fn tag_16_sse2(bench: &mut Bencher) {
    let input = b"ABCDABCDABCDABCDabcd";

    let parser = nom_specialized::combinators::tag_sse2(&b"ABCDABCDABCDABCD"[..]);
    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b"abcd"[..], &b"ABCDABCDABCDABCD"[..])));

    bench.bytes = 16;
    bench.iter(|| parser(&input[..]))
}

fn tag_32_nom(bench: &mut Bencher) {
    let input = b"ABCDABCDABCDABCDABCDABCDABCDABCDabcd";

    let parser = nom::bytes::streaming::tag(&b"ABCDABCDABCDABCDABCDABCDABCDABCD"[..]);
    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b"abcd"[..], &b"ABCDABCDABCDABCDABCDABCDABCDABCD"[..])));

    bench.bytes = 32;
    bench.iter(|| parser(&input[..]))
}

fn tag_32_sse2(bench: &mut Bencher) {
    let input = b"ABCDABCDABCDABCDABCDABCDABCDABCDabcd";

    let parser = nom_specialized::combinators::tag_sse2(&b"ABCDABCDABCDABCDABCDABCDABCDABCD"[..]);
    let res: IResult<_, _> = parser(&input[..]);
    assert_eq!(res, Ok((&b"abcd"[..], &b"ABCDABCDABCDABCDABCDABCDABCDABCD"[..])));

    bench.bytes = 32;
    bench.iter(|| parser(&input[..]))
}

benchmark_group!(
    benches,
    tag_4_nom,
    tag_4_sse2,
    tag_16_nom,
    tag_16_sse2,
    tag_32_nom,
    tag_32_sse2,
);
benchmark_main!(benches);
