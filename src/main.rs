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
    len: usize,
}

fn do_match_instruction(words: &[u8], pattern: &str) -> Option<MatchCtx> {
    let x = words[0] as u16;
    let y = words[1] as u16;
    let v = (x << 8) | y;

    let pattern: String = pattern.chars()
        .filter(|ch| *ch != ' ')
        .collect();
    if pattern.len() != 16 {
        panic!("bad pattern");
    }

    let mut rbits: Vec<u8> = vec![];
    let mut dbits: Vec<u8> = vec![];

    fn nth_bit(v: u16, n: usize) -> u16 {
        (v >> (15 - n as u16)) & 1
    }

    for i in 0..16 {
        match pattern.chars().skip(i).next().unwrap() {
            '0' => {
                if nth_bit(v, i) != 0 {
                    return None;
                }
            },
            '1' => {
                if nth_bit(v, i) != 1 {
                    return None;
                }
            },
            'r' => {
                rbits.push(nth_bit(v, i) as u8);
            }
            'd' => {
                dbits.push(nth_bit(v, i) as u8);
            }
            _ => panic!("bad pattern"),
        }
    }
    
    fn bits2u32(bits: &[u8]) -> Option<u32> {
        if bits.is_empty() {
            return None;
        }

        let mut res = 0u32;
        for (pos, bit) in bits.iter().enumerate() {
            res |= (*bit as u32) << (pos as u32);
        }
        Some(res)
    }

    let reg = bits2u32(&rbits);
    let dst = bits2u32(&dbits);

    Some(MatchCtx { reg, dst, len: 2 })
}

macro_rules! instruction_matcher {
    ({
        $(
            $pattern:expr => ($ctx_name:ident) $body:block
        )*
    }) => {
        fn match_instruction(words: &[u8]) -> Option<Instruction> {
            $(
                match do_match_instruction(words, $pattern) {
                    Some(ctx) => {
                        let $ctx_name = ctx;
                        let result: Instruction = $body;
                        return Some(result);
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
    Ret,
}

instruction_matcher!({
    "0000 0001 dddd rrrr" => (ctx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
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
    "1001 0101 0000 1000" => (ctx) {
        Instruction::Ret
    }
});


fn main() {
    let in_file = env::args().skip(1).next().unwrap();
    let contents = fs::read_to_string(in_file).unwrap();
    let words = read_ihex(&contents);

    // to le
    let mut data = words[56..72].to_vec();
    for pair in data.chunks_exact_mut(2) {
        let f = pair[0];
        let l = pair[1];
        pair[0] = l;
        pair[1] = f;
    }

    let mut cur = 0usize;
    while cur != data.len() {
        let ins = match_instruction(&data[cur..])
            .expect("failed to match instruction");
        println!("{:?}", ins);
        cur += 2;
    }

    // for i in data {
    //     println!("{:08b}", i);
    // }
}
