use super::VarBuf;

#[derive(Debug)]
pub struct InsMask {
    inner: Vec<u8>,
}

impl InsMask {
    pub fn do_match(&self, word: u32) -> Option<VarBuf> {
        let mut buf = VarBuf::new();

        for i in 0..(self.inner.len()) {
            let bit = {
                // last bit index
                let b = self.inner.len() - 1;
                ((word >> ((b - i) as u32)) & 1) as u8
            };

            match self.inner[i] {
                0 => {
                    if bit != 0 {
                        return None;
                    }
                },
                1 => {
                    if bit != 1 {
                        return None;
                    }
                },
                v if v >= 2 && v < 2 + 26 => {
                    const START: u8 = 'a' as u32 as u8;
                    let var = char::from_u32((v - 2 + START) as u32).unwrap();
                    buf.push_bit(var, bit);
                }
                v if v >= 2 + 26 && v < 2 + 26 + 26 => {
                    const START: u8 = 'A' as u32 as u8;
                    let var = char::from_u32((v - 2 - 26 + START) as u32).unwrap();
                    buf.push_bit(var, bit);
                }
                _ => panic!("bad mask!"), // actually unreachable
            }
        }

        Some(buf)
    }

    pub fn bits(&self) -> usize {
        self.inner.len()
    }
}

pub fn parse_mask(s: &str) -> InsMask {
    const ALPHABET_LEN: u8 = 26;

    let mut mask: Vec<u8> = vec![];
    for ch in s.chars() {
        match ch {
            ' ' => continue,
            '0' => {
                mask.push(0);
            },
            '1' => {
                mask.push(1);
            },
            v if v.is_ascii_lowercase() => {
                let n = ((v as u32) - ('a' as u32)) as u8;
                mask.push(2 + n);
            },
            v if v.is_ascii_uppercase() => {
                let n = ((v as u32) - ('A' as u32)) as u8;
                mask.push(2 + ALPHABET_LEN + n);
            },
            _ => {},
        }
    }
    if mask.len() != 8 && mask.len() != 16 && mask.len() != 32 {
        panic!("bad pattern mask!");
    }
    InsMask { inner: mask }
}
