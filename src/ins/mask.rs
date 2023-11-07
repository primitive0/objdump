use anyhow::bail;

use std::collections::HashMap;

#[derive(Debug)]
pub struct VarBuf {
    pub v2b: HashMap<char, Vec<u8>>,
}

impl VarBuf {
    pub fn new() -> Self {
        VarBuf { v2b: HashMap::new() }
    }

    pub fn push_bit(&mut self, var: char, bit: u8) {
        if self.v2b.contains_key(&var) {
            self.v2b.get_mut(&var).unwrap().push(bit);
        } else {
            self.v2b.insert(var, vec![bit]);
        }
    }

    pub fn get(&self, var: char) -> Option<u32> {
        let bits = self.v2b.get(&var)?;
        if bits.is_empty() {
            return None;
        }
        let mut res = 0u32;
        for (pos, bit) in bits.iter().rev().enumerate() {
            res |= (*bit as u32) << (pos as u32);
        }
        Some(res)
    }

    pub fn bits_len(&self, var: char) -> Option<usize> {
        self.v2b.get(&var).map(|bits| bits.len())
    }
}

#[derive(Debug)]
pub struct InsMask {
    inner: Vec<u8>,
}

impl InsMask {
    pub fn do_match(&self, word: u32) -> Option<VarBuf> {
        let mut buf = VarBuf::new();

        for i in 0..(self.inner.len()) {
            // [xxxxxxxx] <- mask
            // |yyyyyyyy| <- word
            //  ^
            //  match mask[0] and word[last_bit]
            //
            //   ^
            //   match mask[1] and word[last_bit - 1]
            let bit = {
                let last_bit = self.inner.len() - 1;
                ((word >> ((last_bit - i) as u32)) & 1) as u8
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
                _ => panic!("bad mask!"),
            }
        }

        Some(buf)
    }

    pub fn bits(&self) -> usize {
        self.inner.len()
    }
}

pub fn parse_mask(s: &str) -> anyhow::Result<InsMask> {
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
            _ => bail!("bad character in mask: {}", s),
        }
    }
    if mask.len() != 16 && mask.len() != 32 {
        bail!("bad pattern mask: {}", s);
    }
    Ok(InsMask { inner: mask })
}
