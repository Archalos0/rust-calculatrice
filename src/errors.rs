use std::fmt;

pub struct ErrorNumberParenthesis;

impl fmt::Display for ErrorNumberParenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an odd number of parenthesis")
    }
}

impl fmt::Debug for ErrorNumberParenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file : {}, line {} }}", file!(), line!())
    }
}