mod
idenfier {
    pub const CLASS: u8 = 0x01;
}

const EQUAL      : u8 = 0x01;
const DOUBLE_QUOT: u8 = 0x02;

impl
CommonLexerFunc for Token {
    fn
    find( &self, letter: char ) -> u8 {
        match letter  {
            '='  => EQUAL,
            '"'  => DOUBLE_QUOT,
            '/'  => SLASH,
            _ => UNDEF
        }
    }

}
