use std::collections::HashMap;
use once_cell::sync::Lazy;

static C2B: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    let mut c2b = HashMap::new();
    for (i, ch) in ('0'..='9').into_iter().enumerate() {
        c2b.insert(ch, i as u8);
    }
    for (i, ch) in ('A'..='F').into_iter().enumerate() {
        c2b.insert(ch, (10 + i) as u8);
    }
    c2b
});

pub fn read_ihex(s: &str) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    for mut l in s.split("\n") {
        if l == ":00000001FF" {
            break;
        }

        const PREFIX_LEN: usize = 1 + 8; // : + meta
        const SUFFIX_LEN: usize = 2; // checksum
        l = &l[PREFIX_LEN..(l.len() - SUFFIX_LEN)];
        for pair in l.chars().collect::<Vec<char>>().chunks(2) {
            if pair.len() != 2 {
                panic!("read_ihex: bad input");
            }
            let p1 = *C2B.get(&pair[0]).unwrap();
            let p2 = *C2B.get(&pair[1]).unwrap();
            let v = (p1 << 4) | p2;
            res.push(v);
        }
    }
    res
}
