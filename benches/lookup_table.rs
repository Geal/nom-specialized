#[macro_use]
extern crate bencher;
extern crate nom;
extern crate nom_specialized;

use bencher::Bencher;
use nom::bytes::streaming::take_while1;
use nom::IResult;
use rand::distributions::{Alphanumeric, Uniform};
use rand::Rng;

const fn is_alphabetic(c: u8) -> bool {
    (c >= 0x41 && c <= 0x5A) || (c >= 0x61 && c <= 0x7A)
}

const fn is_digit(c: u8) -> bool {
    c >= 0x30 && c <= 0x39
}

const fn is_alphanumeric(c: u8) -> bool {
    is_alphabetic(c) || is_digit(c)
}

const fn lut_generator(c: u8) -> u8 {
    let mut res = 0u8;

    if is_alphabetic(c) {
        res |= 1;
    }
    if is_digit(c) {
        res |= 2;
    }

    res
}

const LOOKUP_TABLE: [u8; 256] = nom_specialized::make_lookup_table!(lut_generator);

fn is_alphabetic_lut(c: u8) -> bool {
    (LOOKUP_TABLE[c as usize] & 1) != 0
}

fn is_digit_lut(c: u8) -> bool {
    (LOOKUP_TABLE[c as usize] & 2) != 0
}

fn is_alphanumeric_lut(c: u8) -> bool {
    (LOOKUP_TABLE[c as usize] & 3) != 0
}

fn alphabetic(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_alphabetic)(i)
}

fn alphabetic_lut(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_alphabetic_lut)(i)
}

fn digit(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_digit)(i)
}

fn digit_lut(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_digit_lut)(i)
}

fn alphanumeric(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_alphanumeric)(i)
}

fn alphanumeric_lut(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_alphanumeric_lut)(i)
}

fn alphabetic_1024_nom(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u8> = std::iter::repeat(())
        .map(|_| rng.sample(Uniform::new('A' as u8, '[' as u8)))
        .take(1023)
        .collect();
    v.push(b';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphabetic(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| alphabetic(&v[..]))
}

fn alphabetic_1024_lut(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u8> = std::iter::repeat(())
        .map(|_| rng.sample(Uniform::new('A' as u8, '[' as u8)))
        .take(1023)
        .collect();
    v.push(b';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphabetic_lut(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| alphabetic_lut(&v[..]))
}

fn alphabetic_16384_nom(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u8> = std::iter::repeat(())
        .map(|_| rng.sample(Uniform::new('A' as u8, '[' as u8)))
        .take(16383)
        .collect();
    v.push(b';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphabetic(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..16383])));

    bench.bytes = 16384;
    bench.iter(|| alphabetic(&v[..]))
}

fn alphabetic_16384_lut(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u8> = std::iter::repeat(())
        .map(|_| rng.sample(Uniform::new('A' as u8, '[' as u8)))
        .take(16383)
        .collect();
    v.push(b';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphabetic_lut(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..16383])));

    bench.bytes = 16384;
    bench.iter(|| alphabetic_lut(&v[..]))
}

fn alphanumeric_1024_nom(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: String = std::iter::repeat(())
        .map(|_| rng.sample(Alphanumeric))
        .take(1023)
        .collect();
    v.push(';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphanumeric(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| alphanumeric(&v[..]))
}

fn alphanumeric_1024_lut(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: String = std::iter::repeat(())
        .map(|_| rng.sample(Alphanumeric))
        .take(1023)
        .collect();
    v.push(';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphanumeric_lut(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..1023])));

    bench.bytes = 1024;
    bench.iter(|| alphanumeric_lut(&v[..]))
}

fn alphanumeric_16384_nom(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: String = std::iter::repeat(())
        .map(|_| rng.sample(Alphanumeric))
        .take(16383)
        .collect();
    v.push(';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphanumeric(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..16383])));

    bench.bytes = 16384;
    bench.iter(|| alphanumeric(&v[..]))
}

fn alphanumeric_16384_lut(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut v: String = std::iter::repeat(())
        .map(|_| rng.sample(Alphanumeric))
        .take(16383)
        .collect();
    v.push(';');
    let v: Vec<_> = v.into();

    let res: IResult<_, _> = alphanumeric_lut(&v[..]);
    assert_eq!(res, Ok((&b";"[..], &v[..16383])));

    bench.bytes = 16384;
    bench.iter(|| alphanumeric_lut(&v[..]))
}

benchmark_group!(
    benches,
    alphabetic_1024_nom,
    alphabetic_1024_lut,
    alphabetic_16384_nom,
    alphabetic_16384_lut,
    alphanumeric_1024_nom,
    alphanumeric_1024_lut,
    alphanumeric_16384_nom,
    alphanumeric_16384_lut,
);
benchmark_main!(benches);
