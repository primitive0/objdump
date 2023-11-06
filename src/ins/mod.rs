mod var_buf;
pub use var_buf::VarBuf;

mod mask;
pub use mask::InsMask;

mod fmt;
pub use fmt::{InsCmd, CmdArg, ArgSpecifier};

// todo: smallvec

#[derive(Debug)]
pub struct InsPattern {
    mask: InsMask,
    cmd: InsCmd,
}

impl InsPattern {
    pub fn do_match(&self, word: u32) -> Option<VarBuf> {
        self.mask.do_match(word)
    }

    pub fn len(&self) -> usize {
        self.mask.bits() / 8
    }

    pub fn cmd(&self) -> &InsCmd {
        &self.cmd
    }
}

fn parse_pattern(s: &str) -> InsPattern {
    let pair: Vec<&str> = s.split("=>").collect();
    assert_eq!(pair.len(), 2);
    let mask = mask::parse_mask(pair[0].trim());
    let cmd = fmt::parse_cmd(pair[1].trim());
    InsPattern { mask, cmd }
}

pub fn parse_instructions(contents: &str) -> Vec<InsPattern> {
    let mut patterns: Vec<InsPattern> = vec![];
    for line in contents.split("\n") {
        if line == "" {
            continue;
        }
        patterns.push(parse_pattern(line));
    }
    patterns
}
