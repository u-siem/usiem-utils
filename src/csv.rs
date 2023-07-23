pub struct CsvLexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}
fn transform_escape_char(ch: char) -> Result<char, ()> {
    match ch {
        'n' => Ok('\n'),
        't' => Ok('\t'),
        'r' => Ok('\r'),
        '0' => Ok('\0'),
        '*' => Err(()),
        _ => Ok(ch),
    }
}

impl CsvLexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut CsvLexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch != ','  {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_string = |l: &mut CsvLexer| -> Vec<char> {
            let mut is_escape = false;
            let mut to_ret = Vec::with_capacity(32);
            while l.position < l.input.len() && !(l.ch == '"' && !is_escape) {
                if l.ch == '\\' {
                    if is_escape {
                        to_ret.push('\\');
                    }
                    is_escape = !is_escape;
                } else {
                    if is_escape {
                        match transform_escape_char(l.ch) {
                            Ok(ch) => to_ret.push(ch),
                            Err(_) => {
                                // The \\ character is used
                                to_ret.push('\\');
                                to_ret.push(l.ch);
                            }
                        };
                    } else {
                        to_ret.push(l.ch);
                    }
                    is_escape = false;
                }
                l.read_char();
            }
            to_ret
        };

        let tok: Token;
        match self.ch {
            ',' => {
                tok = Token::COMMA;
            },
            '"' => {
                self.read_char();
                let data = read_string(self);
                tok = Token::String(data.iter().collect())
            }
            _ => {
                let data: Vec<char> = read_identifier(self);
                tok = Token::String(data.iter().collect())
            }
        }
        self.read_char();
        tok
    }
}


#[derive(Debug, PartialEq)]
pub enum Token {
    COMMA,
    String(String)
}