use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

pub struct StringParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> StringParser<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }

    pub fn dismiss_str(&mut self, dismissable: &str) {
        for c in dismissable.chars() {
            self.dismiss_char(c)
        }
    }

    pub fn dismiss_char(&mut self, dismissable: char) {
        if self.next_char() != dismissable {
            panic!("Expected {dismissable} found other item when dismissing item")
        }
    }

    pub fn parse_num<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut value = String::from(self.next_char());
        while let Some('0'..='9') = self.peek() {
            value.push(self.next_char());
        }
        value.parse().unwrap()
    }

    pub fn parse_num_vec_until_newline<T>(&mut self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut numbers = vec![];
        loop {
            match self.peek() {
                Some(' ') => self.dismiss_any_whitespace(),
                Some('0'..='9') => {
                    numbers.push(self.parse_num::<T>());
                }
                Some('\r' | '\n') => {
                    self.next_char();
                    if let Some('\r' | '\n') = self.peek() {
                        self.next_char();
                    }
                    break;
                }
                Some(_) => {
                    panic!()
                }
                None => break,
            }
        }
        numbers
    }

    pub fn dismiss_any_whitespace(&mut self) {
        while let Some(' ') = self.peek() {
            self.next_char();
        }
    }

    fn next_char(&mut self) -> char {
        self.chars
            .next()
            .expect("Tried to get a char none was left")
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}
