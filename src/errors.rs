use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct ImbalancedBrackets;

impl Display for ImbalancedBrackets {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for ImbalancedBrackets {}

#[derive(Clone, Debug)]
pub struct TapeBoundsOverflow;

impl Display for TapeBoundsOverflow {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for TapeBoundsOverflow {}

#[derive(Clone, Debug)]
pub struct TapeBoundsUnderflow;

impl Display for TapeBoundsUnderflow {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for TapeBoundsUnderflow {}
