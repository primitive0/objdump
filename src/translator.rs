use anyhow::bail;

use crate::ins::InsPattern;

pub fn translate(patterns: &[InsPattern], data: &[u8]) -> anyhow::Result<String> {
    let mut res = String::new();

    let mut cursor = 0usize;
    'c: while cursor < data.len() {
        for pattern in patterns {
            if data.len() - cursor < pattern.len() {
                continue;
            }

            let word: u32;
            match pattern.len() {
                2 => {
                    word = read_word(&data[cursor..]) as u32;
                },
                4 => {
                    let b1 = read_word(&data[cursor..]) as u32;
                    let b2 = read_word(&data[(cursor + 2)..]) as u32;
                    word = (b1 << 16) | b2;
                },
                _ => panic!("malformed pattern: pattern length is {}", pattern.len()),
            }

            if let Some(buf) = pattern.do_match(word) {
                res.push_str(&format!("{:>4x}: ", cursor));
                pattern.cmd().write_cmd(&buf, &mut res);
                res.push_str("\n");

                cursor += pattern.len();
                continue 'c;
            }
        }

        bail!("failed to decode instruction");
    }
    Ok(res)
}

fn read_word(data: &[u8]) -> u16 {
    let b1 = *data.get(0).unwrap() as u16;
    let b2 = *data.get(1).unwrap() as u16;
    (b2 << 8) | b1
}
