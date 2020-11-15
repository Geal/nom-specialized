use std::arch::x86_64::*;
use nom::{IResult, HexDisplay, Needed, Err, error::{ParseError, ErrorKind}};

pub fn multitag<'a, Error: ParseError<&'a [u8]>>(tags:&[&[u8]])
  -> impl Fn(&'a [u8]) -> IResult<&'a [u8], usize, Error>{

  let Masks { cmp, shuf_mask, high_mask, low_mask, ids } = prepare(tags);

  move |i: &'a[u8]| {
      if i.len() < 16 {
          return Err(Err::Incomplete(Needed::Unknown));
      }

    let input = load16(i);
    let cmp_mask = load(&cmp);
    let shuf_mask = load(&shuf_mask[..]);
    let shuffled = unsafe { _mm256_shuffle_epi8(input, shuf_mask) };
    let cmpres = unsafe { _mm256_cmpeq_epi8(shuffled, cmp_mask) };
    let maskres = unsafe { _mm256_movemask_epi8(cmpres) };
    let tmp_mask = maskres as u32 & !high_mask;
    let (tmp_mask2, _) = tmp_mask.overflowing_add(low_mask);
    let tmp_mask3 = tmp_mask2 & maskres as u32;
    let res = tmp_mask3 & high_mask;

    let cnt = unsafe { _lzcnt_u32(res) };

    if cnt < 31 {
        let idx = ids[(31 - cnt) as usize];
        if idx == 0xFFu8 {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)))
        } else {
            Ok((&i[4..], idx as usize))
        }
    } else {
        Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag)))
    }
  }
}

pub fn process(m: &MasksVec, input: __m256i) -> u8 {
    let shuffled = unsafe { _mm256_shuffle_epi8(input, m.shuf_mask) };
    let cmpres = unsafe { _mm256_cmpeq_epi8(shuffled, m.cmp) };
    let maskres = unsafe { _mm256_movemask_epi8(cmpres) };
    let tmp_mask = maskres as u32 & !m.high_mask;
    let (tmp_mask2, _) = tmp_mask.overflowing_add(m.low_mask);
    let tmp_mask3 = tmp_mask2 & maskres as u32;
    let res = tmp_mask3 & m.high_mask;

    let cnt = unsafe { _lzcnt_u32(res) };

    if cnt < 31 {
        let idx = m.ids[(31 - cnt) as usize];
        idx
    } else {
        0xFFu8
    }
}

pub struct Masks {
  cmp: [u8; 32],
  shuf_mask: [u8; 32],
  high_mask: u32,
  low_mask: u32,
  ids: [u8; 32],
}

pub const fn prepare(strings: &[&[u8]]) -> Masks {
    let mut cmp = [0u8; 32];
    let mut shuf_mask = [0u8; 32];
    let mut high_mask = 0u32;
    let mut low_mask = 0u32;
    let mut ids = [0xFFu8; 32];

    let mut index = 0usize;
    let mut s_index = 0usize;
    loop {
        if s_index == strings.len() {
            break;
        }

        let s = strings[s_index];

        let mut cmp_index = index;
        loop {
            if cmp_index == index+s.len() {
                break;
            }

            cmp[cmp_index] = s[cmp_index - index];
            cmp_index += 1;
        }

        let mut i = 0usize;
        loop {
            if i == s.len() {
                break;
            }
            shuf_mask[index+i] = i as u8;

            i += 1;
        }

        high_mask |= 1 << index + s.len() - 1;
        low_mask |= 1 << index;

        ids[(index + s.len()) - 1] = s_index as u8;

        /*
        println!("cmp: {:x?}", cmp);
        println!("shuf_mask: {:x?}", shuf_mask);
        print_u32("high mask", high_mask);
        print_u32("low mask", low_mask);
        */

        index += s.len();
        s_index += 1;
    }

    //println!("cmpstring: {}", std::str::from_utf8(&cmp[..]).unwrap());
    //println!("ids: {:?}", ids);

    Masks { cmp, shuf_mask, high_mask, low_mask, ids }
}

pub struct MasksVec {
  cmp: __m256i,
  shuf_mask: __m256i,
  high_mask: u32,
  low_mask: u32,
  ids: [u8; 32],
}

impl MasksVec {
    pub fn from(m: Masks) -> Self {
        let Masks { cmp, shuf_mask, high_mask, low_mask, ids } = m;
        let cmp = load(&cmp[..]);
        let shuf_mask = load(&shuf_mask[..]);

        MasksVec { cmp, shuf_mask, high_mask, low_mask, ids }
    }
}

fn avx(i: &[u8]) -> () {
    println!("input:\n{}", i.to_hex(16));
    let input = load16(i);
    let d = dump(input);
    println!("dumped:\n{}", &d.to_hex(16));

    let strings = [&b"Acce"[..], &b"ConA"[..], &b"Date"[..], &b"Cont"[..], &b"Forw"[..], &b"Host"[..], &b"User"[..], &b"Upgr"[..]];
    let Masks { cmp, shuf_mask, high_mask, low_mask, ids } = prepare(&strings[..]);
    let cmp_mask = load(&cmp);
    print_hex("cmp_mask", cmp_mask);
    let shuf_mask = load(&shuf_mask[..]);
    let shuffled = unsafe { _mm256_shuffle_epi8(input, shuf_mask) };
    print_hex("shuffled", shuffled);

    let cmpres = unsafe { _mm256_cmpeq_epi8(shuffled, cmp_mask) };
    print_hex("cmpres", cmpres);
    //print_bin("cmpres", cmpres);

    let maskres = unsafe { _mm256_movemask_epi8(cmpres) };
    println!("maskres: LE {:x?}, BE: {:x?}", maskres.to_le_bytes(), maskres.to_be_bytes());
    print_u32("maskres_u32", maskres as u32);
    print_u32("hi", high_mask);
    print_u32("low", low_mask);
    let tmp_mask = maskres as u32 & !high_mask;
    print_u32("mask & !hi", tmp_mask);
    let (tmp_mask2, _) = tmp_mask.overflowing_add(low_mask);
    print_u32("(mask & !hi) + low", tmp_mask2);
    let tmp_mask3 = tmp_mask2 & maskres as u32;
    print_u32("((mask & !hi) + low) & mask", tmp_mask3);
    let res = tmp_mask3 & high_mask;
    print_u32("((mask & !hi) + low) & mask & hi", res);

    let cnt = unsafe { _lzcnt_u32(res) };
    println!("lzcnt: {}", cnt);

    println!("found? {:?}", std::str::from_utf8(strings[ids[(31 - cnt) as usize] as usize]).unwrap());
}

fn load(i: &[u8]) -> __m256i {
    unsafe {
        _mm256_loadu2_m128i(
            (&i[16..]).as_ptr() as *const _,
            i.as_ptr() as *const _,
        )
    }
}

pub fn load16(i: &[u8]) -> __m256i {
    unsafe {
        _mm256_loadu2_m128i(
            i.as_ptr() as *const _,
            i.as_ptr() as *const _)
    }
}

fn dump(i: __m256i) -> [u8; 32] {
    let mut res = [0u8; 32];

    unsafe {
        _mm256_storeu2_m128i(
            (&mut res[16..]).as_mut_ptr() as *mut _,
            res.as_mut_ptr() as *mut _,
            i
        );
    }

    res
}

pub fn print_hex(prefix: &str, i: __m256i) {
    println!("{}:\n{}", prefix, &(dump(i)).to_hex(16));
}

fn print_bin(prefix: &str, i: __m256i) {
    let d = dump(i);

    let mut s = String::new();
    for byte in d.iter() {
        for i in 0..8 {
            let c = if (byte & (1 << 7 - i)) != 0 {
                '1'
            } else {
                '_'
            };
            s.push(c);
        }
        s.push(' ');
    }
    println!("{}:\n{}", prefix, s);
}

fn print_u32(suffix: &str, i: u32) {
    let d = i.to_le_bytes();
    println!("u32 bytes:{:x?}", d);
    let mut s = String::new();
    for idx in 0..32 {
        s.push(if ((i >> idx) & 1) == 1 { '1' } else { '_' });
    }
    println!("{}\t{}", s, suffix);
}

fn push_byte(s: &mut String, byte: u8) {
    for i in 0..8 {
        let c = if (byte & (1 << (7-i))) != 0 {
            '1'
        } else {
            '_'
        };
        s.push(c);
    }
}

fn print_u32_be(suffix: &str, i: u32) {
    let d = i.to_be_bytes();
    println!("u32 bytes:{:x?}", d);
    let mut s = String::new();
    push_byte(&mut s, d[3]);
    push_byte(&mut s, d[2]);
    push_byte(&mut s, d[1]);
    push_byte(&mut s, d[0]);
    println!("{}\t{}", s, suffix);
}

fn push_byte_be(s: &mut String, byte: u8) {
    for i in 0..8 {
        let c = if (byte & (1 << (7-i))) != 0 {
            '1'
        } else {
            '_'
        };
        s.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avx_test() {
        avx(&b"Content-Length: 1234\r\nHost: hello.com"[..]);

        panic!();
    }

    #[test]
    fn prepare_test() {
        let strings = [&b"Acce"[..], &b"Cont"[..], &b"Date"[..], &b"ConA"[..], &b"Forw"[..], &b"Host"[..], &b"User"[..], &b"Up34"[..]];
        prepare(&strings[..]);
        panic!();
    }
}
