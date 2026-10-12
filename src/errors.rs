use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct ImbalancedBrackets;

impl Display for ImbalancedBrackets {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Program ended with imbalanced brackets")
    }
}

impl Error for ImbalancedBrackets {}
