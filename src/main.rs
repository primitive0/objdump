use std::collections::HashMap;
use std::{env, path::PathBuf};
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

macro_rules! instruction_matcher {
    () => {
        
    };
}

struct MatchCtx {
    reg: Option<u32>,
    dst: Option<u32>,
}

instruction_matcher!({
    "0000 0001 dddd rrrr" => (ctx: MatchCtx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Movw(dst, reg)
    }
    "0000 11rd dddd rrrr" => (ctx: MatchCtx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Add(dst, reg)
    }
    "1001 11rd dddd rrrr" => (ctx: MatchCtx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Mul(dst, reg)
    }
    "0010 01rd dddd rrrr" => (ctx: MatchCtx) {
        let dst = ctx.dst.unwrap();
        let reg = ctx.reg.unwrap();
        Instruction::Eor(dst, reg)
    }
    "1001 0101 0000 1000" => (ctx: MatchCtx) {
        Instruction::Ret
    }
});

fn foo(words: &[u8]) {
    for p in words.chunks_exact(2) {
        let x = p[0] as u16;
        let y = p[1] as u16;
        let v = (x << 8) | y;

        let pattern: String = "...".chars()
            .filter(|ch| *ch != ' ')
            .collect();
        if pattern.len() != 16 {
            panic!("bad pattern");
        }

        let dbits: Vec<u8> = vec![];
        let rbits: Vec<u8> = vec![];

        for i in (0..16).rev().map(|v| v as u16) {
            
        }
    }
}

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

    for i in data {
        println!("{:08b}", i);
    }
}
