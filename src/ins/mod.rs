use anyhow::bail;

mod mask;
pub use mask::InsMask;
pub use mask::VarBuf;

mod cmd;
pub use cmd::{InsCmd, CmdArg, ArgSpecifier};

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

fn parse_pattern(s: &str) -> anyhow::Result<InsPattern> {
    let pair: Vec<&str> = s.split("=>").collect();
    if pair.len() != 2 {
        bail!("failed to parse pattern: {}", s);
    }
    let mask = mask::parse_mask(pair[0].trim())?;
    let cmd = cmd::parse_cmd(pair[1].trim())?;
    Ok(InsPattern { mask, cmd })
}

pub fn parse_instructions(contents: &str) -> anyhow::Result<Vec<InsPattern>> {
    let mut patterns: Vec<InsPattern> = vec![];
    for line in contents.split("\n") {
        if line == "" {
            continue;
        }
        patterns.push(parse_pattern(line)?);
    }
    Ok(patterns)
}
