use super::{InsPattern, InsCmd, CmdArg, CmdFormat, InsTable};

fn parse_mask(s: &str) -> Vec<u8> {
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
    mask
}

fn parse_cmd_arg(arg: &str) -> CmdArg {
    let mut iter = arg.chars().rev();
    let var = iter.next().unwrap();
    
    let mut format: Vec<CmdFormat> = vec![];
    for sp in iter {
        let cmd_format = match sp {
            'R' => CmdFormat::Register,
            'W' => CmdFormat::Wide,
            'U' => CmdFormat::Offset16,
            'S' => CmdFormat::Signed,
            _ => panic!("error!"),
        };
        format.push(cmd_format);
    }
    CmdArg { var, format }
}

fn parse_cmd(cmd: &str) -> InsCmd {
    fn next(c: &str, i: usize) -> (&str, &str) {
        (&c[..i], &c[i..])
    }
    fn skip(c: &str) -> &str {
        &c[1..]
    }

    // cursor
    let mut c = cmd;

    let name;
    match c.find(' ') {
        Some(i) => {
            (name, c) = next(c, i);
            c = skip(c);
        }
        None => (name, c) = (c, ""),
    };

    let mut args: Vec<CmdArg> = vec![];
    loop {
        let arg = match c.find(',') {
            Some(i) => {
                let arg;
                (arg, c) = next(c, i);
                c = skip(c);

                loop {
                    match c.chars().next() {
                        Some(ch) => {
                            if ch != ' ' {
                                break;
                            }
                            c = skip(c);
                        }
                        _ => {},
                    }
                }

                arg
            },
            None if c.is_empty() => break,
            _ => {
                let arg = c;
                c = "";
                arg
            }
        };

        args.push(parse_cmd_arg(arg));
    }

    InsCmd {
        name: name.to_string(),
        args,
    }
}

fn parse_pattern(s: &str) -> InsPattern {
    let pair: Vec<&str> = s.split("=>").collect();
    assert_eq!(pair.len(), 2);
    let mask = parse_mask(pair[0].trim());
    let cmd = parse_cmd(pair[1].trim());
    InsPattern { mask, cmd }
}

pub fn parse_instructions(contents: &str) -> InsTable {
    let mut patterns: Vec<InsPattern> = vec![];
    for line in contents.split("\n") {
        if line == "" {
            break;
        }
        patterns.push(parse_pattern(line));
    }
    InsTable { patterns }
}
