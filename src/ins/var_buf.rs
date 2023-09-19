use std::collections::HashMap;

// todo: zero copy
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
        let bits = self.v2b.get(&var).unwrap();
        if bits.is_empty() {
            return None;
        }
        let mut res = 0u32;
        for (pos, bit) in bits.iter().rev().enumerate() {
            res |= (*bit as u32) << (pos as u32);
        }
        Some(res)
    }

    pub fn bits(&self, var: char) -> Option<usize> {
        self.v2b.get(&var).map(|bits| bits.len())
    }
}
