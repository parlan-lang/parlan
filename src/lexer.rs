// the lexer implementation //

#![allow(dead_code)]


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TkType {
// Keywords
    If,
    Else,
    Extern,
    Func,
    Return,
    Var,
    While,
    Break,
    Continue,
    
// Delimiters
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Colon,
    Semicolon,
    Comma,

// Operators
    Plus,
    Minus,
    Star,
    Slash,
    Assing,
    Lt,
    Gt,
    Eq,
    Ne,
    Le,
    Ge,
    And,
    Or,
    Not,
    VaArgs,

// Types
    IntT,
    FloatT,
    BoolT,
    StringT,
    VoidT,

// Literals
    IntL,
    FloatL,
    True,
    False,
    StringL,
    Id,
    
    Eof, // sentinel value

    Err // error value
}

pub struct Tk {
    pub tk_type: TkType,
    pub start:   usize,
    pub end:     usize,
    line:    usize
}

impl Tk {
    pub fn new(tk_type: TkType, start: usize, end: usize, line: usize) -> Self {
        return Tk {
            tk_type,
            start,
            end,
            line
        }
    }

    // debug implementations for Tk //
    pub fn print(&self, source: &str) {
        eprintln!("Token [{}] {:?} >> `{}`", self.line, self.tk_type, source.get(self.start..self.end).or(Some("EOF")).unwrap());
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    source:   String,
    pos:      usize,
    line:     usize,
    chars:    Vec<char>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        return Lexer {
            source: source.clone(),
            pos: 0,
            line: 1,
            chars: source.chars().collect()
        };
    }

    // auxiliar functions //

    /// checks if some identifier that starts at `start` is a keyword
    fn check_identifier(&self, start: usize) -> TkType {
        match self.source.get(start..self.pos).unwrap() {
            "var" => TkType::Var,
            "if" => TkType::If,
            "else" => TkType::Else,
            "extern" => TkType::Extern,
            "func" => TkType::Func,
            "return" => TkType::Return,
            "while" => TkType::While,
            "break" => TkType::Break,
            "continue" => TkType::Continue,
            "int" => TkType::IntT,
            "float" => TkType::FloatT,
            "bool" => TkType::BoolT,
            "str" => TkType::StringT,
            "void" => TkType::VoidT,
            "and" => TkType::And,
            "or" => TkType::Or,
            "not" => TkType::Not,
            "true" => TkType::True,
            "false" => TkType::False,
            _ => TkType::Id
        }
    }

    fn check_character(&self) -> TkType {
        if self.chars.len() - self.pos >= 3 {
            if self.chars[self.pos..self.pos+3] == ['.','.','.'] {
                return TkType::VaArgs
            }
        }
        match self.source.get(self.pos..self.pos + 1).unwrap() {
            "(" => TkType::Lparen,
            ")" => TkType::Rparen,
            "{" => TkType::Lbrace,
            "}" => TkType::Rbrace,
            ":" => TkType::Colon,
            ";" => TkType::Semicolon,
            "," => TkType::Comma,
            "+" => TkType::Plus,
            "-" => TkType::Minus,
            "*" => TkType::Star,
            "/" => TkType::Slash,
            _ => TkType::Err,
        }
    }

    // main function
    pub fn next_token(&mut self) -> Tk {
        while self.pos < self.source.len() {
            if self.chars[self.pos].is_whitespace() {
                if self.chars[self.pos] == '\n' { self.line += 1 }
                self.pos += 1;
            } else if self.chars[self.pos] == '/' && self.chars[self.pos+1] == '/' {
                self.pos += 2;
                while self.chars[self.pos] != '\n' { self.pos += 1 }
                self.pos += 1;
                self.line += 1;
            } else {
                break;
            }
        }

        if self.pos >= self.source.len() {
            return Tk::new(TkType::Eof, self.pos, 0, self.line);
        }

        match self.chars[self.pos] {
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = self.pos;
                while self.pos < self.source.len() && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_') { self.pos += 1; }

                return Tk::new(self.check_identifier(start), start, self.pos, self.line)
            }
            '0'..='9' => {
                let start = self.pos;
                let mut dot = false;
                while self.pos < self.source.len() && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.') {
                    if self.chars[self.pos] == '.' {
                        if dot {
                            eprintln!(r#"[ERROR] invalid number with 2 decimal points at line {}"#, self.line);
                            panic!("panicking due to error while lexing");
                        } else {
                            self.pos += 1;
                            dot = true;
                        }
                    } else {
                        self.pos += 1;
                    }
                }

                if dot {
                    return Tk::new(TkType::FloatL, start, self.pos, self.line);
                } else {
                    return Tk::new(TkType::IntL, start, self.pos, self.line);
                }
            }
            '"' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < self.source.len() && self.chars[self.pos] != '"' && self.chars[self.pos] != '\n' { self.pos += 1; }
                self.pos += 1; // second `"`

                return Tk::new(TkType::StringL, start, self.pos-1, self.line);
            }
            _ => {
                if self.chars[self.pos] == '>' {
                    if self.chars[self.pos+1] == '=' {
                        self.pos += 2;
                        return Tk::new(TkType::Ge, self.pos-2, self.pos, self.line);
                    } else {
                        self.pos += 1;
                        return Tk::new(TkType::Gt, self.pos-1, self.pos, self.line);
                    }
                } else if self.chars[self.pos] == '<' {
                    if self.chars[self.pos+1] == '=' {
                        self.pos += 2;
                        return Tk::new(TkType::Le, self.pos-2, self.pos, self.line);
                    } else {
                        self.pos += 1;
                        return Tk::new(TkType::Lt, self.pos-1, self.pos, self.line);
                    }
                } else if self.chars[self.pos] == '=' {
                    if self.chars[self.pos+1] == '=' {
                        self.pos += 2;
                        return Tk::new(TkType::Eq, self.pos-2, self.pos, self.line);
                    } else {
                        self.pos += 1;
                        return Tk::new(TkType::Assing, self.pos-1, self.pos, self.line);
                    }
                } else if self.chars[self.pos] == '!' && self.chars[self.pos+1] == '=' {
                    self.pos += 2;
                    return Tk::new(TkType::Ne, self.pos-2, self.pos, self.line);
                } else {
                    let ttype = self.check_character();
                    if ttype != TkType::Err && ttype != TkType::VaArgs {
                        self.pos += 1;
                        return Tk::new(ttype, self.pos-1, self.pos, self.line);
                    } else if ttype == TkType::VaArgs {
                        self.pos += 3;
                        return Tk::new(ttype, self.pos-3, self.pos, self.line)
                    } else {
                        eprintln!("[ERROR] unknown start of token `{}`", self.chars[self.pos]);
                        panic!()
                    }
                }
            }
        }        
    }

    // returns the current token but does not change the position or line
    pub fn peek_token(&mut self) -> Tk {
        let cpos = self.pos; // current position
        let cline = self.line; // current line
        let tk = self.next_token();
        self.pos = cpos;
        self.line = cline;
        return tk;
    }
}
