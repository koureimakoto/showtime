use super::{
    element,
    common::{CommonLexerFunc, Token}
};


const EOS: u8 = 0xFF;



pub struct 
Engine {
    state: usize,    // Current State
    curr : usize,    // Current Token Position
    last : usize,    // Last    Token Position
    code : String, // Code to Tokenize
}

impl Engine {
    pub fn 
    tokenize_this( code: &str ) -> Self {
        Self {
            state: 0,
            last : 0,
            curr : 0,
            code : String::from( code )
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
    find( letter: char ) {
        let el = element::Token.find(letter);

    }

    pub fn 
    token( &mut self ) -> u8 {
        
        match self.curr_token() {
            Some(token) => {
                self.next_token();
                1
                
            }, None => {
                EOS
            } 
        }
        
    }

}




