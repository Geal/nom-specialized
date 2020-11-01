#[macro_use]
extern crate bencher;
extern crate nom;
extern crate nom_specialized;

use bencher::Bencher;
use nom::IResult;
use nom::error::Error;

fn take_while1_1024_nom(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(1023).collect::<Vec<_>>();
    v.push(b'b');

    let parser = nom::bytes::streaming::take_while1(|c| c == b'a');
    let res: IResult<_, _> = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_1024_unrolled(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(1023).collect::<Vec<_>>();
    v.push(b'b');

    let parser = nom_specialized::combinators::take_while1_unrolled(|c| c == b'a');
    let res: IResult<_, _> = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_1024_sse2(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(1023).collect::<Vec<_>>();
    v.push(b'b');

    let ranges = b"\0`b\xFF";

    let parser = nom_specialized::combinators::take_while1_sse2::<_, Error<&[u8]>>(|c| c == b'a', &ranges[..]);
    let res = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_50_nom(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(49).collect::<Vec<_>>();
    v.push(b'b');

    let parser = nom::bytes::streaming::take_while1(|c| c == b'a');
    let res: IResult<_, _> = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..49])));

    bench.bytes = 50;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_50_unrolled(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(49).collect::<Vec<_>>();
    v.push(b'b');

    let parser = nom_specialized::combinators::take_while1_unrolled(|c| c == b'a');
    let res: IResult<_, _> = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..49])));

    bench.bytes = 50;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_50_sse2(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(49).collect::<Vec<_>>();
    v.push(b'b');

    let ranges = b"\0`b\xFF";

    let parser = nom_specialized::combinators::take_while1_sse2::<_, Error<&[u8]>>(|c| c == b'a', &ranges[..]);
    let res = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..49])));

    bench.bytes = 1024;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_16384_nom(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(16383).collect::<Vec<_>>();
    v.push(b'b');

    let parser = nom::bytes::streaming::take_while1(|c| c == b'a');
    let res: IResult<_, _> = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..16383])));

    bench.bytes = 16384;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_16384_unrolled(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(16383).collect::<Vec<_>>();
    v.push(b'b');

    let parser = nom_specialized::combinators::take_while1_unrolled(|c| c == b'a');
    let res: IResult<_, _> = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..16383])));

    bench.bytes = 16384;
    bench.iter(|| parser(&v[..]))
}

fn take_while1_16384_sse2(bench: &mut Bencher) {
    let mut v = std::iter::repeat(b'a').take(16383).collect::<Vec<_>>();
    v.push(b'b');

    let ranges = b"\0`b\xFF";

    let parser = nom_specialized::combinators::take_while1_sse2::<_, Error<&[u8]>>(|c| c == b'a', &ranges[..]);
    let res = parser(&v[..]);
    assert_eq!(res, Ok((&b"b"[..], &v[..16383])));

    bench.bytes = 1024;
    bench.iter(|| parser(&v[..]))
}

benchmark_group!(
    benches,
    take_while1_50_nom,
    take_while1_50_unrolled,
    take_while1_50_sse2,
    take_while1_1024_nom,
    take_while1_1024_unrolled,
    take_while1_1024_sse2,
    take_while1_16384_nom,
    take_while1_16384_unrolled,
    take_while1_16384_sse2,
);
benchmark_main!(benches);
