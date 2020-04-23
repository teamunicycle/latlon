use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;

#[derive(Debug)]
pub(crate) struct ParseErrorInternal;

impl From<ParseFloatError> for ParseErrorInternal {
    fn from(_: ParseFloatError) -> Self {
        ParseErrorInternal
    }
}

impl Display for ParseErrorInternal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Parse error")
    }
}

#[derive(Debug)]
pub struct GeoParseError<T: AsRef<str> + Display>(pub T);

impl<T: AsRef<str> + Display> Display for GeoParseError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing coordinates from {}", self.0)
    }
}
