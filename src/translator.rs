use crate::ins::InsPattern;

pub fn translate(patterns: &[InsPattern], data: &[u8]) -> String {
    let mut res = String::new();

    let mut cursor = 0usize;
    'c: while cursor < data.len() {
        for pattern in patterns {
            let Some(word) = read_word(&data[cursor..], pattern.len()) else {
                // ????
                if data.is_empty() {
                    break 'c;
                } else {
                    continue;
                }
            };
            if let Some(buf) = pattern.do_match(word) {
                res.push_str(&format!("{:>4x}: ", cursor));
                pattern.cmd().write_cmd(&buf, &mut res);
                cursor += pattern.len();
                continue 'c;
            }
        }
    }
    res
}

// fn read_word1(data: &[u8]) -> Option<u16> {
//     let b1 = *data.get(0)? as u16;
//     let b2 = *data.get(1)? as u16;
//     let mut res = (b2 << 8) | b1;
//     res = res.swap_bytes(); // todo: remove
//     Some(res)
// }

fn read_word(data: &[u8], len: usize) -> Option<u32> {
    assert!(len <= 4);

    let mut res = 0u32;
    for i in (0..len).rev() {
        res |= (*data.get(i)? as u32) << (8 * (len - i - 1));
    } 
    Some(res)
}
