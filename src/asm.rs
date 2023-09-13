pub struct AsmBuilder {
    pub buf: String,
}

#[derive(Debug, Clone, Copy)]
pub enum InsArg {
    Reg(u32),
    Val(u32),
    Offset(i32),
}

impl AsmBuilder {
    pub fn new() -> Self {
        AsmBuilder {
            buf: "".to_string(),
        }
    }

    pub fn push_ins(&mut self, ins: &str, args: &[InsArg]) {
        self.buf.push_str(ins);
        self.buf.push_str("    ");

        if !args.is_empty() {
            let last_index = args.len() - 1;
            for (i, arg) in args.iter().cloned().enumerate() {
                match arg {
                    InsArg::Reg(v) => {
                        let f = format!("r{}", v);
                        self.buf.push_str(&f);
                    },
                    InsArg::Val(v) => {
                        let f = format!("{:02x}", v);
                        self.buf.push_str(&f);
                    },
                    InsArg::Offset(v) => {
                        let f = if v >= 0 {
                            format!(".+{}", v)
                        } else {
                            format!(".-{}", -v)
                        };
                        self.buf.push_str(&f);
                    },
                }

                if i != last_index {
                    self.buf.push_str(", ");
                }
            }
        }

        self.buf.push_str("\n");
    }
}
