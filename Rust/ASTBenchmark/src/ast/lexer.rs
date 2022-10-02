use super::{
    element::{self, Token},
    common::{CommonLexerFunc}
};

const EOS: u8 = 0xFF; // End of String

pub struct 
Engine {
    state: usize,    // Current State
    curr : usize,    // Current Token Position
    last : usize,    // Last    Token Position
    code : String, // Code to Tokenize
    el   : Token
}

impl Engine {

    pub fn 
    tokenize_this( code: &str )
    -> Self {
        Self {
            state: 0,
            last : 0,
            curr : 0,
            code : String::from( code ),
            el   : Token::new(' ')
        }
    }

    pub fn
    next_token( &mut self ) {
        self.last = self.curr;
        self.curr += 1;
    }

    pub fn 
    curr_token( &self )
    -> Option<char> {
        self.code.chars().nth( self.curr )
    }

    pub fn 
    last_token( &self )
    -> Option<char> {
        self.code.chars().nth( self.last )
    }

    pub fn
    end_of( &self, wrapper_content_sign: char ) -> bool {

        match self.curr_token() {
            Some( content_flag ) => {
                if content_flag != wrapper_content_sign &&
                self.curr <= self.code.len() {
                    return false
                }
                true
            }
            None => true
        }

    }

    pub fn
    is_whitespace( &mut self ) -> bool { 

        'is_whitespace: loop {

            match self.curr_token() {
                Some( letter ) => {
                    if 
                    letter == ' '        || letter == '\u{00a0}' ||
                    letter == '\u{1680}' || letter == '\u{200a}' ||
                    letter == '\u{2028}' || letter == '\u{2029}' ||
                    letter == '\u{feff}' {
                        self.next_token();
                        continue 'is_whitespace    
                    }
                        return true
                },
                None => return false
            }

        }

    }

    pub fn 
    token( &mut self ) -> u8 {
        
        self.is_whitespace();
        
        match self.curr_token() {
            Some(token) => {
                self.next_token();
                
                self.el.from(token);
                if self.el.less() {
                    return element::LESS
                }
                if self.el.more() {
                    return element::MORE
                }
                if self.el.slash() {
                    return element::SLASH
                }
        
                element::UNDEF
                
            }, None => {
                EOS
            } 
        }
        
    }

}




