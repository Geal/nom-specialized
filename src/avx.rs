use std::arch::x86_64::*;
use nom::HexDisplay;

fn avx(i: &[u8]) -> () {
    println!("input:\n{}", i.to_hex(16));
    //let input = _mm256_broadcastsi128_si256(i.as_ptr() as *const _ );
    /*let input = unsafe {
        _mm256_loadu2_m128i(
            i.as_ptr() as *const _,
            i.as_ptr() as *const _)
    };*/
    let input = load(i);
    let d = dump(input);
    println!("dumped:\n{}", &d.to_hex(16));

    let mask = [0u8, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3,
                0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];
    let shuf_mask = load(&mask[..]);
    let shuffled = unsafe { _mm256_shuffle_epi8(input, shuf_mask) };
    print_hex("shuffled", shuffled);
    //println!("shuffled:\n{}", &(dump(shuffled)).to_hex(16));

    let compare_string = b"AcceContDateConAForwHostUserUp34";
    let cmp_mask = load(&compare_string[..]);
    print_hex("cmp_mask", cmp_mask);
    //let high_mask: u32 = 0b1000_1000_1000_1000_0000_0000_0000_0000;
    //let high_mask: u32 = 0b1000_1000_1000_1000_0000_0000_0000_0000;
    //let low_mask: u32  = 0b0001_0001_0001_0001_0000_0000_0000_0000;
    let high_mask: u32 = 0b0000_0000_0000_0000_1000_1000_1000_1000;
    let low_mask: u32  = 0b0000_0000_0000_0000_0001_0001_0001_0001;

    let cmpres = unsafe { _mm256_cmpeq_epi8(shuffled, cmp_mask) };
    print_hex("cmpres", cmpres);
    //print_bin("cmpres", cmpres);

    let maskres = unsafe { _mm256_movemask_epi8(cmpres) };
    println!("maskres: LE {:x?}, BE: {:x?}", maskres.to_le_bytes(), maskres.to_be_bytes());
    //print_u32("maskres_before reorder", maskres as u32);
    //let maskres = reorder_mask(maskres as u32);
    print_u32("maskres_u32", maskres as u32);
    print_u32("hi", high_mask);
    print_u32("low", low_mask);
    let tmp_mask = maskres as u32 & !high_mask;
    print_u32("mask & !hi", tmp_mask);
    let tmp_mask2 = tmp_mask + low_mask;
    print_u32("(mask & !hi) + low", tmp_mask2);
    let tmp_mask3 = tmp_mask2 & maskres as u32;
    print_u32("((mask & !hi) + low) & mask", tmp_mask3);
    let res = tmp_mask3 & high_mask;
    print_u32("((mask & !hi) + low) & mask & hi", res);

    println!("lzcnt: {}", unsafe { _lzcnt_u32(res) });
}

fn reorder_mask(i:u32) -> u32 {
    let d = i.to_le_bytes();
    let mut d2 = [0u8; 4];
    d2[0] = d[1];
    d2[1] = d[0];
    d2[2] = d[3];
    d2[3] = d[2];

    u32::from_le_bytes(d2)
}

fn load(i: &[u8]) -> __m256i {
    unsafe {
        _mm256_loadu2_m128i(
            (&i[16..]).as_ptr() as *const _,
            i.as_ptr() as *const _,
        )
    }
}

fn load16(i: &[u8]) -> __m256i {
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

fn print_hex(prefix: &str, i: __m256i) {
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
}
