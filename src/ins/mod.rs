mod parser;
use std::collections::HashMap;

pub use parser::parse_instructions;

// todo: smallvec

#[derive(Debug, Clone)]
enum CmdFormat {
    Register,
    Wide,
    Offset16,
    Signed,
}

#[derive(Debug, Clone)]
struct CmdArg {
    var: char,
    format: Vec<CmdFormat>,
}

#[derive(Debug, Clone)]
pub struct InsCmd {
    name: String,
    args: Vec<CmdArg>,
}

impl InsCmd {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn write_cmd(&self, buf: &VarBuf, builder: &mut String) {
        enum ArgType {
            Reg(u8),
            Val(u32),
            Signed(i32),
        }

        builder.push_str(&self.name);
        let mut arg_strs: Vec<String> = vec![];
        for arg in &self.args {
            let mut a = ArgType::Val(buf.get(arg.var).unwrap());
            for format in &arg.format {
                match format {
                    CmdFormat::Register => {
                        match a {
                            ArgType::Val(v) => {
                                if v <= 31 {
                                    a = ArgType::Reg(v as u8);
                                }
                            },
                            _ => panic!("failed"),
                        }
                    },
                    CmdFormat::Wide => {
                        match a {
                            ArgType::Val(v) => {
                                a = ArgType::Val(v * 2);
                            },
                            ArgType::Signed(v) => {
                                a = ArgType::Signed(v * 2);
                            },
                            _ => panic!("failed"),
                        }
                    },
                    CmdFormat::Offset16 => {
                        match a {
                            ArgType::Val(v) => {
                                a = ArgType::Val(v + 16);
                            }
                            _ => panic!("failed"),
                        }
                    },
                    CmdFormat::Signed => {
                        match a {
                            ArgType::Val(v) => {
                                let v = decode_signed(v, buf.len(arg.var).unwrap());
                                a = ArgType::Signed(v)
                            }
                            _ => panic!("failed"),
                        }
                    },
                }
            }

            match a {
                ArgType::Reg(v) => {
                    let f = format!("r{}", v);
                    arg_strs.push(f);
                },
                ArgType::Val(v) => {
                    let f = format!("{:02x}", v);
                    arg_strs.push(f);
                },
                ArgType::Signed(v) => {
                    let f = if v >= 0 {
                        format!(".+{}", v)
                    } else {
                        format!(".-{}", -v)
                    };
                    arg_strs.push(f);
                },
            }
        }

        let s = arg_strs.join(", ");
        if !s.is_empty() {
            builder.push_str(" ");
        }
        builder.push_str(&s);
        builder.push_str("\n");
    }
}

#[derive(Debug)]
pub struct InsPattern {
    mask: Vec<u8>,
    cmd: InsCmd,
}

// todo: передвинуть выше
// todo: zero copy
#[derive(Debug)]
pub struct VarBuf {
    v2b: HashMap<char, Vec<u8>>,
}

impl VarBuf {
    fn new() -> Self {
        VarBuf { v2b: HashMap::new() }
    }

    fn push_bit(&mut self, var: char, bit: u8) {
        if self.v2b.contains_key(&var) {
            self.v2b.get_mut(&var).unwrap().push(bit);
        } else {
            self.v2b.insert(var, vec![bit]);
        }
    }

    fn get(&self, var: char) -> Option<u32> {
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

    fn len(&self, var: char) -> Option<usize> {
        self.v2b.get(&var).map(|bits| bits.len())
    }
}

impl InsPattern {
    pub fn match_pattern(&self, word: u32) -> Option<VarBuf> {
        let mut buf = VarBuf::new();

        for i in 0..(self.mask.len()) {
            let bit = {
                // last bit index
                let b = self.mask.len() - 1;
                ((word >> ((b - i) as u32)) & 1) as u8
            };
            // println!("name: {}, bit: {}", self.cmd.name, bit);

            match self.mask[i] {
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

    pub fn len(&self) -> usize {
        self.mask.len() / 8
    }

    // todo: remove clone
    pub fn cmd(&self) -> InsCmd {
        self.cmd.clone()
    }
}

// todo: remove
#[derive(Debug)]
pub struct InsTable {
    pub patterns: Vec<InsPattern>,
}

// todo: move from here
fn decode_signed(i: u32, len: usize) -> i32 {
    fn mk1(len: usize) -> u32 {
        let mut res = 0;
        for i in 0..len {
            res |= 1 << i;
        }
        res
    }

    if i >> (len - 1) == 1 {
        let not = i ^ mk1(len);
        -((not + 1) as i32)
    } else {
        i as i32
    }
}
