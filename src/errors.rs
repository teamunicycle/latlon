use std::fmt;
use std::fmt::{Display, Formatter, Debug};
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

pub struct GeoParseError<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Display for GeoParseError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing coordinates from {}", self.0.as_ref())
    }
}

impl<T : AsRef<str>> Debug for GeoParseError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("GeoParseError")
            .field(&self.0.as_ref())
            .finish()
    }
}

impl<T : AsRef<str>> std::error::Error for GeoParseError<T> {}
