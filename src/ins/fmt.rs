use super::VarBuf;

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

#[derive(Debug)]
pub enum ArgSpecifier {
    Register,
    Wide,
    Offset16,
    Signed,
}

#[derive(Debug)]
pub struct CmdArg {
    pub var: char,
    pub format: Vec<ArgSpecifier>,
}

#[derive(Debug)]
enum CmdFormat {
    Reg(u8),
    Val(u32),
    Signed(i32),
}

impl CmdArg {
    pub fn format(&self, buf: &VarBuf) -> String {
        let mut format = CmdFormat::Val(buf.get(self.var).unwrap());
        for sp in &self.format {
            match sp {
                ArgSpecifier::Register => {
                    match format {
                        CmdFormat::Val(v) => {
                            if v <= 31 {
                                format = CmdFormat::Reg(v as u8);
                            }
                        },
                        _ => panic!("failed"),
                    }
                },
                ArgSpecifier::Wide => {
                    match format {
                        CmdFormat::Val(v) => {
                            format = CmdFormat::Val(v * 2);
                        },
                        CmdFormat::Signed(v) => {
                            format = CmdFormat::Signed(v * 2);
                        },
                        _ => panic!("failed"),
                    }
                },
                ArgSpecifier::Offset16 => {
                    match format {
                        CmdFormat::Val(v) => {
                            format = CmdFormat::Val(v + 16);
                        }
                        _ => panic!("failed"),
                    }
                },
                ArgSpecifier::Signed => {
                    match format {
                        CmdFormat::Val(v) => {
                            let v = decode_signed(v, buf.bits(self.var).unwrap());
                            format = CmdFormat::Signed(v)
                        }
                        _ => panic!("failed"),
                    }
                },
            }
        }

        match format {
            CmdFormat::Reg(v) => format!("r{}", v),
            CmdFormat::Val(v) => format!("0x{:02x}", v),
            CmdFormat::Signed(v) => {
                if v >= 0 {
                    format!(".+{}", v)
                } else {
                    format!(".-{}", -v)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct InsCmd {
    pub name: String,
    pub args: Vec<CmdArg>,
}

impl InsCmd {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn write_cmd(&self, buf: &VarBuf, builder: &mut String) {
        builder.push_str(&self.name);
        let mut arg_strs: Vec<String> = vec![];
        for arg in &self.args {
            arg_strs.push(arg.format(buf));
        }

        let s = arg_strs.join(", ");
        if !s.is_empty() {
            builder.push_str(" ");
        }
        builder.push_str(&s);
        builder.push_str("\n");
    }
}

pub fn parse_cmd_arg(arg: &str) -> CmdArg {
    let mut iter = arg.chars().rev();
    let var = iter.next().unwrap();

    let mut format: Vec<ArgSpecifier> = vec![];
    for sp in iter {
        let cmd_format = match sp {
            'R' => ArgSpecifier::Register,
            'W' => ArgSpecifier::Wide,
            'U' => ArgSpecifier::Offset16,
            'S' => ArgSpecifier::Signed,
            _ => panic!("error!"),
        };
        format.push(cmd_format);
    }
    CmdArg { var, format }
}

pub fn parse_cmd(cmd: &str) -> InsCmd {
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
