use std::collections::HashMap;
use std::env;
use std::fs;

// type Word = u16;

fn read_ihex(s: &str) -> Vec<u8> {
    let mut c2b: HashMap<char, u8> = HashMap::new();
    for (i, ch) in ('0'..='9').into_iter().enumerate() {
        c2b.insert(ch, i as u8);
    }
    for (i, ch) in ('A'..='F').into_iter().enumerate() {
        c2b.insert(ch, (10 + i) as u8);
    }

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
            let p1 = *c2b.get(&pair[0]).unwrap();
            let p2 = *c2b.get(&pair[1]).unwrap();
            let v = (p1 << 4) | p2;
            res.push(v);
        }
    }
    res
}

struct MatchCtx {
    reg: Option<u32>,
    dst: Option<u32>,
    k: Option<u32>,
    port: Option<u32>,
    len: usize,
}

fn read_word(data: &[u8], len: usize) -> Option<u32> {
    assert!(len <= 4);

    let mut res = 0u32;
    for i in (0..len).rev() {
        res |= (*data.get(i)? as u32) << (8 * (len - i - 1));
    }
    Some(res)
}

fn do_match_instruction(words: &[u8], pattern: &str) -> Option<MatchCtx> {
    let pattern: String = pattern.chars()
        .filter(|ch| *ch != ' ')
        .collect();

    let bytes_required: usize = {
        const ALLOWED_SIZES: [usize; 3] = [8, 16, 32];
        if !ALLOWED_SIZES.contains(&pattern.len()) {
            panic!("bad pattern");
        }
        pattern.len() / 8
    };

    let word = read_word(words, bytes_required)?;
    // println!("{:032b}", word);

    let mut rbits: Vec<u8> = vec![];
    let mut dbits: Vec<u8> = vec![];
    let mut kbits: Vec<u8> = vec![];
    let mut pbits: Vec<u8> = vec![];

    let nth_bit = |n: usize| {
        let bits = bytes_required * 8;
        ((word >> ((bits - n - 1) as u32)) & 1) as u8
    };

    for i in 0..(pattern.len()) {
        let bit = nth_bit(i);
        match pattern.chars().skip(i).next().unwrap() {
            '0' => {
                if bit != 0 {
                    return None;
                }
            },
            '1' => {
                if bit != 1 {
                    return None;
                }
            },
            'r' => {
                rbits.push(bit);
            }
            'd' => {
                dbits.push(bit);
            },
            'k' => {
                kbits.push(bit);
            },
            'P' => {
                pbits.push(bit);
            }
            _ => panic!("bad pattern"),
        }
    }
    
    fn bits2u32(bits: &[u8]) -> Option<u32> {
        if bits.is_empty() {
            return None;
        }

        let mut res = 0u32;
        for (pos, bit) in bits.iter().rev().enumerate() {
            res |= (*bit as u32) << (pos as u32);
        }
        Some(res)
    }

    let reg = bits2u32(&rbits);
    let dst = bits2u32(&dbits);
    let k = bits2u32(&kbits);
    let port = bits2u32(&pbits);

    Some(MatchCtx { reg, dst, k, port, len: bytes_required })
}

macro_rules! instruction_matcher {
    ({
        $(
            $pattern:expr => ($ctx_name:ident) $body:block
        )*
    }) => {
        fn match_instruction(words: &[u8]) -> Option<(Instruction, usize)> {
            $(
                match do_match_instruction(words, $pattern) {
                    Some(ctx) => {
                        let $ctx_name = ctx;
                        let result: Instruction = $body;
                        return Some((result, $ctx_name.len));
                    }
                    None => {}
                }
            )*

            None
        }
    };
}

#[derive(Debug)]
enum Instruction {
    Movw {
        dst: u32,
        reg: u32,
    },
    Add {
        dst: u32,
        reg: u32
    },
    Mul {
        dst: u32,
        reg: u32,
    },
    Eor {
        dst: u32,
        reg: u32,
    },
    Jmp {
        pos: u32,
    },
    Out {
        reg: u32,
        port: u32,
    },
    Ldi {
        dst: u32,
        val: u32,
    },
    Call {
        addr: u32,
    },
    Ret,
}

instruction_matcher!({
    "0000 0001 dddd rrrr" => (ctx) {
        let dst = ctx.dst.unwrap() * 2; // only even registers allowed
        let reg = ctx.reg.unwrap() * 2;
        Instruction::Movw { dst, reg }
    }
    "0000 11rd dddd rrrr" => (ctx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Add { dst, reg }
    }
    "1001 11rd dddd rrrr" => (ctx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Mul { dst, reg }
    }
    "0010 01rd dddd rrrr" => (ctx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Eor { dst, reg }
    }
    "1001 010k kkkk 110k kkkk kkkk kkkk kkkk" => (ctx) {
        let k = ctx.k.unwrap();
        Instruction::Jmp { pos: k }
    }
    "1011 1PPr rrrr PPPP" => (ctx) {
        let reg = ctx.reg.unwrap();
        let port = ctx.port.unwrap();
        Instruction::Out { reg, port }
    }
    "1110 kkkk dddd kkkk" => (ctx) {
        let dst = ctx.dst.unwrap();
        let val = ctx.k.unwrap();
        Instruction::Ldi { dst, val }
    }
    "1001 0101 0000 1000" => (ctx) {
        Instruction::Ret
    }
    "1001 010k kkkk 111k kkkk kkkk kkkk kkkk" => (ctx) {
        let addr = ctx.k.unwrap();
        Instruction::Call { addr }
    }
});

fn main() {
    let in_file = env::args().skip(1).next()
        .expect("expected 1 argument to program");
    let contents = fs::read_to_string(in_file).unwrap();
    let words = read_ihex(&contents);

    // to le
    let mut data = words;
    for pair in data.chunks_exact_mut(2) {
        let f = pair[0];
        let l = pair[1];
        pair[0] = l;
        pair[1] = f;
    }

    let mut cur = 0usize;
    while cur != data.len() {
        let (ins, offset) = match_instruction(&data[cur..])
            .expect("failed to match instruction");
        println!("{:?}", ins);
        cur += offset;
    }

    // for i in data {
    //     println!("{:08b}", i);
    // }
}
