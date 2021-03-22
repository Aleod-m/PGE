pub use std::io::Read;

pub struct TokenStream<'a, T: Read>
{
    inner : T,
}

impl Iterator<T> for TokenStream<T: Read> {
    type Item = TokenTree;

    pub fn next(&mut self) -> Option<TokenTree> {
        
    }
}

pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Litteral(Litteral),
}

pub struct Ident {
    ident : String,
}

pub enum Litteral {
    Bool(bool),
    Integer,
    Float,
}

pub struct Group {
    delimiter : Delimiter,
}

pub enum Delimiter {
    Parentethis,
    Brace,
    Bracket,
}

pub struct Punct {
    spacing : Spacing,
    c : char,
} 

pub enum Spacing {
    Alone,
    Joint,
}
