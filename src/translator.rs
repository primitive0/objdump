use crate::ins::InsTable;

pub fn translate(table: &InsTable, data: &[u8]) -> String {
    let mut res = String::new();

    let mut cursor = 0usize;
    'c: while cursor < data.len() {
        for pattern in &table.patterns {
            let Some(word) = read_word(&data[cursor..], pattern.len()) else {
                if data.is_empty() {
                    break 'c;
                } else {
                    continue;
                }
            };
            if let Some(buf) = pattern.match_pattern(word) {
                pattern.cmd().write_cmd(&buf, &mut res);
                cursor += pattern.len();
                continue 'c;
            }
        }
    }
    res
}

fn read_word(data: &[u8], len: usize) -> Option<u32> {
    assert!(len <= 4);

    let mut res = 0u32;
    for i in (0..len).rev() {
        res |= (*data.get(i)? as u32) << (8 * (len - i - 1));
    }
    Some(res)
}
