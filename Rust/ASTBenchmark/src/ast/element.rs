use super::common::{ Token, CommonLexerFunc} ;

pub const UNDEF  : u8 = 0x0;

pub const LESS   : u8 = 0x1;
pub const MORE   : u8 = 0x2;
pub const SLASH  : u8 = 0x4;

const LESS_SIMBOL : char = '<';
const MORE_SIMBOL : char = '>';
const SLASH_SIMBOL: char = '/';

impl
Token {
    pub fn
    less( &self ) -> bool {
        self.flag == LESS_SIMBOL
    }

    pub fn
    more( &self ) -> bool {
        self.flag == MORE_SIMBOL
    }

    pub fn
    slash( &self ) -> bool {
        self.flag == SLASH_SIMBOL
    }
}


impl
CommonLexerFunc for Token {

    fn 
    find( letter: char ) -> Self {
        Self{ flag: letter}
    }
    
}


