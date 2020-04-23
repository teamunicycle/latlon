#[macro_use]
extern crate lazy_static;

use geo_types::Point;

use regex::Regex;
use std::fmt::{Display, Formatter};
use std::convert::TryFrom;
use std::fmt;
use std::num::ParseFloatError;

pub mod errors {
    use std::fmt::{Display, Formatter};
    use std::fmt;
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
    pub struct GeoParseError<T : AsRef<str> + Display>(pub T);

    impl<T : AsRef<str> + Display> Display for GeoParseError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Error parsing coordinates from {}", self.0)
        }
    }
}

use crate::errors::ParseErrorInternal;
pub use crate::errors::GeoParseError;

// Two-sided patterns
lazy_static! {
    // 40° 26′ 46″ N 79° 58′ 56″ W
    // 40 26 46 N 79 58 56 W
    // -40 26 46 N -79 58 56 W
    // (spaces optional if not ambiguous)
    static ref RE_DMS_NS_DMS_EW: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?\s*
        (N|S)
        \s*
        [,;]?
        \s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?\s*
        (E|W)
        $
        ").unwrap();

    // N 40° 26′ 46″ W 79° 58′ 56″
    // N 40° 26’ 46″ W 79° 58’ 56″
    // N 40 26 46 W 79 58 56
    // N -40 26 46 W -79 58 56
    // (spaces optional if not ambiguous)
    static ref RE_NS_DMS_EW_DMS: Regex = Regex::new(r"(?x)
        ^
        (N|S)\s*
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?
        \s*
        [,;]?
        \s*
        (E|W)\s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?
        $
        ").unwrap();

    // 40° 26′ 46″ 79° 58′ 56″
    // 40 26 46 79 58 56 (strange, but not ambiguous)
    // -40 26 46; -79 58 56
    // (spaces optional if not ambiguous)
    static ref RE_DMS_DMS: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?\s*
        [,;]?
        \s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?
        $
        ").unwrap();

    // 40° 26.767' N 79° 58.933' W
    // 40° 26.767’ N 79° 58.933’ W
    // 40 26.767 N 79 58.933 W
    // 40 26,767 N 79 58,933 W
    // -40 26,767 N -79 58,933 W
    // (spaces optional if not ambiguous)
    static ref RE_DM_NS_DM_EW: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?\s*
        (N|S)
        \s*
        [,;]?
        \s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?\s*
        (E|W)
        $
        ").unwrap();

    // N 40° 26.767' W 79° 58.933'
    // N 40° 26.767’ W 79° 58.933’
    // N 40 26.767 W 79 58.933
    // N 40 26,767 W 79 58,933
    // N -40 26,767 W -79 58,933
    // (spaces optional if not ambiguous)
    static ref RE_NS_DM_EW_DM: Regex = Regex::new(r"(?x)
        ^
        (N|S)\s*
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?
        \s*
        [,;]?
        \s*
        (E|W)\s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?
        $
        ").unwrap();

    // 40° 26.767' 79° 58.933'
    // 40° 26.767’ 79° 58.933’
    // 40 26.767 79 58.933
    // 40 26,767 79 58,933
    // -40 26,767 -79 58,933
    // (spaces optional if not ambiguous)
    static ref RE_DM_DM: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?
        \s*
        [,;]?
        \s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?
        $
        ").unwrap();

    // N 40.446° W 79.982°
    // N 40.446 W 79.982
    // N 40,446 W 79,982
    // N -40,446 W -79,982
    // (spaces optional)
    static ref RE_NS_D_EW_D: Regex = Regex::new(r"(?x)
        ^
        (N|S)\s*
        (-?\d{1,2}(?:[.,]\d+)?)°?
        \s*
        [,;]?
        \s*
        (E|W)\s*
        (-?\d{1,3}(?:[.,]\d+)?)°?
        $
        ").unwrap();

    // 40.446° N 79.982° W
    // 40.446 N 79.982 W
    // 40,446 N 79,982 W
    // -40,446 N -79,982 W
    // (spaces optional)
    static ref RE_D_NS_D_EW: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2}(?:[.,]\d+)?)°?\s*
        (N|S)\s*
        [,;]?
        \s*
        (-?\d{1,3}(?:[.,]\d+)?)°?\s*
        (E|W)
        $
        ").unwrap();

    // 40.446° 79.982°
    // 40.446°, 79.982°
    // 40.446°; 79.982°
    // 40.446 79.982
    // 40.446; 79.982
    // 40.446, 79.982
    // 40,446 79,982
    // 40,446, 79,982
    // -40,446 -79,982
    // (spaces optional if not ambiguous)
    static ref RE_D_D: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2}(?:[.,]\d+)?)(?:°\s*[,;]?\s*|\s*[,;]\s*|\s+)
        (-?\d{1,3}(?:[.,]\d+)?)°?
        $
        ").unwrap();
}

// One-sided patterns
lazy_static! {
    // 40° 26′ 46″ N
    static ref RE_DMS_NSEW: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?\s*
        (N|S|E|W)
        $
        ").unwrap();

    // N 40° 26′ 46″
    static ref RE_NSEW_DMS: Regex = Regex::new(r"(?x)
        ^
        (N|S|E|W)\s*
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?
        $
        ").unwrap();

    // 40° 26′ 46″
    static ref RE_DMS: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,3})(?:°\s*|\s+)
        (\d{1,2})(?:[’′]\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[″”]?\s*
        $
        ").unwrap();

    // 40° 26.767' N
    static ref RE_DM_NSEW: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?\s*
        (N|S|E|W)
        $
        ").unwrap();

    // N 40° 26.767'
    static ref RE_NSEW_DM: Regex = Regex::new(r"(?x)
        ^
        (N|S|E|W)\s*
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?
        $
        ").unwrap();

    // 40° 26.767'
    static ref RE_DM: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2})(?:°\s*|\s+)
        (\d{1,2}(?:[.,]\d+)?)[’′]?
        $
        ").unwrap();

    // N 40.446°
    static ref RE_NSEW_D: Regex = Regex::new(r"(?x)
        ^
        (N|S|E|W)\s*
        (-?\d{1,2}(?:[.,]\d+)?)°?
        $
        ").unwrap();

    // 40.446° N
    static ref RE_D_NSEW: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2}(?:[.,]\d+)?)°?\s*
        (N|S|E|W)
        $
        ").unwrap();

    // 40.446°
    static ref RE_D: Regex = Regex::new(r"(?x)
        ^
        (-?\d{1,2}(?:[.,]\d+)?)°?
        $
        ").unwrap();
}

/// Parsed degrees, minutes, seconds
#[derive(Debug)]
struct DMS {
    d: f64,
    m: f64,
    s: f64
}

// North / South
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum NS {
    North,
    South
}

impl NS {
    /// Get the opposite
    fn invert(self) -> NS {
        match self {
            NS::North => NS::South,
            NS::South => NS::North,
        }
    }
}

impl Display for NS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NS::North => "N",
            NS::South => "S",
        })
    }
}

impl<'a> TryFrom<&str> for NS {
    type Error = ParseErrorInternal;

    fn try_from(value: &str) -> Result<Self, ParseErrorInternal> {
        match value {
            "N" => Ok(NS::North),
            "S" => Ok(NS::South),
            _ => Err(ParseErrorInternal)
        }
    }
}

/// East / West
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum EW {
    East,
    West
}

impl EW {
    fn invert(self) -> EW {
        match self {
            EW::East => EW::West,
            EW::West => EW::East,
        }
    }
}

impl Display for EW {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            EW::East => "E",
            EW::West => "W",
        })
    }
}

impl<'a> TryFrom<&str> for EW {
    type Error = ParseErrorInternal;

    fn try_from(value: &str) -> Result<Self, ParseErrorInternal> {
        match value {
            "E" => Ok(EW::East),
            "W" => Ok(EW::West),
            _ => Err(ParseErrorInternal)
        }
    }
}

/// Parse a string containing a pair of coordinates (latitude, longitude).
///
/// Positive latitude is North, positive longitude is East.
///
/// ## Supported formats (examples)
///
/// - `40° 26′ 46″ N 79° 58′ 56″ W`
/// - `N 40° 26′ 46″ W 79° 58′ 56″`
/// - `40° 26.767' N 79° 58.933' W`
/// - `40° 26′ 46″ 79° 58′ 56″`, `40° 26′ 46″, 79° 58′ 56″`, ...
/// - `N 40° 26.767' W 79° 58.933'`
/// - `40° 26.767' 79° 58.933'`, `40° 26.767', 79° 58.933'`, ...
/// - `N 40.446° W 79.982°`
/// - `40.446° N 79.982° W`
/// - `40.446° 79.982°`, `40.446,79.982`, etc.
///
/// ## Parser rules
/// - All formats support negative degrees (preceded by a minus sign).
/// - Whitespace is optional and ignored, except for formats that would become unparsable.
/// - Degree, minute and second symbols can be omitted.
/// - Unicode quotes (`’`, `”`) may be used in place of apostrophe and double quote (`'`, `"`)
///   for minutes and seconds.
/// - The two coordinates can be separated by comma (`,`), semicolon (`;`), whitespace (` `), or nothing
///   at all, if not ambiguous.
///
/// # Returns
/// Returns a `Point` with longitude as X and latitude as Y (natural map orientation), or
/// a parse error wrapping the source string (for zero-copy patterns)
pub fn parse<T : AsRef<str> + Display>(text : T) -> Result<Point<f64>, GeoParseError<T>> {
    let s = text.as_ref().trim();

    match do_parse(s) {
        Ok(p) => Ok(p),
        Err(_) => Err(GeoParseError(text)),
    }
}

/// Parse string as latitude (N/S). Positive latitude is North.
///
/// See `parse()` for supported formats.
pub fn parse_lat<T : AsRef<str> + Display>(text : T) -> Result<f64, GeoParseError<T>> {
    let s = text.as_ref().trim();

    match do_parse_lat(s) {
        Ok(p) => Ok(p),
        Err(_) => Err(GeoParseError(text)),
    }
}

/// Parse string as longitude (E/W). Positive longitude is East.
///
/// See `parse()` for supported formats.
pub fn parse_lng<T : AsRef<str> + Display>(text : T) -> Result<f64, GeoParseError<T>> {
    let s = text.as_ref().trim();

    match do_parse_lng(s) {
        Ok(p) => Ok(p),
        Err(_) => Err(GeoParseError(text)),
    }
}

trait ParseFloatWithComma {
    fn parse_allow_comma(self) -> Result<f64, ParseFloatError>;
}

impl<'a> ParseFloatWithComma for &'a str {
    fn parse_allow_comma(self) -> Result<f64, ParseFloatError> {
        if self.contains(',') {
            let fixed = self.replace(',', ".");
            fixed.parse()
        } else {
            self.parse()
        }
    }
}

fn build_point(lat: DMS, ns : NS, lng: DMS, ew : EW) -> Result<Point<f64>, ParseErrorInternal> {
    Ok(Point::new(
        build_lng(lng, ew)?,
        build_lat(lat, ns)?
    ))
}

fn build_lng(mut lng: DMS, mut ew : EW) -> Result<f64, ParseErrorInternal> {
    // the minus sign must go in front of the whole coordinate, not just degrees!
    if lng.d < 0. {
        ew = ew.invert();
        lng.d = -lng.d;
    }

    let mut lng_f: f64 = lng.d + (lng.m/60f64) + (lng.s/3600f64);

    if ew == EW::West {
        lng_f = -lng_f;
    }

    if lng_f > 180f64 || lng_f < -180f64 {
        return Err(ParseErrorInternal);
    }

    Ok(lng_f)
}

fn build_lat(mut lat: DMS, mut ns : NS) -> Result<f64, ParseErrorInternal> {
    // the minus sign must go in front of the whole coordinate, not just degrees!
    if lat.d < 0. {
        ns = ns.invert();
        lat.d = -lat.d;
    }

    let mut lat_f: f64 = lat.d + (lat.m/60f64) + (lat.s/3600f64);

    if ns == NS::South {
        lat_f = -lat_f;
    }

    if lat_f > 90f64 || lat_f < -90f64 {
        return Err(ParseErrorInternal);
    }

    Ok(lat_f)
}

fn do_parse(s : &str) -> Result<Point<f64>, ParseErrorInternal> {
    if let Some(cap) = RE_DMS_NS_DMS_EW.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse()?,
            s: cap.get(3).unwrap().as_str().parse()?
        };

        let ns = NS::try_from(cap.get(4).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(5).unwrap().as_str().parse()?,
            m: cap.get(6).unwrap().as_str().parse()?,
            s: cap.get(7).unwrap().as_str().parse()?
        };

        let ew = EW::try_from(cap.get(8).unwrap().as_str())?;

        return build_point(lat, ns, lng, ew);
    }

    if let Some(cap) = RE_NS_DMS_EW_DMS.captures(s) {
        let ns = NS::try_from(cap.get(1).unwrap().as_str())?;

        let lat = DMS {
            d: cap.get(2).unwrap().as_str().parse()?,
            m: cap.get(3).unwrap().as_str().parse()?,
            s: cap.get(4).unwrap().as_str().parse()?
        };

        let ew = EW::try_from(cap.get(5).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(6).unwrap().as_str().parse()?,
            m: cap.get(7).unwrap().as_str().parse()?,
            s: cap.get(8).unwrap().as_str().parse()?
        };

        return build_point(lat, ns, lng, ew);
    }

    if let Some(cap) = RE_DMS_DMS.captures(s) {
        let ns = NS::North;
        let ew = EW::East;

        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse()?,
            s: cap.get(3).unwrap().as_str().parse()?
        };

        let lng = DMS {
            d: cap.get(4).unwrap().as_str().parse()?,
            m: cap.get(5).unwrap().as_str().parse()?,
            s: cap.get(6).unwrap().as_str().parse()?
        };

        return build_point(lat, ns, lng, ew);
    }

    if let Some(cap) = RE_DM_NS_DM_EW.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        let ns = NS::try_from(cap.get(3).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(4).unwrap().as_str().parse()?,
            m: cap.get(5).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        let ew = EW::try_from(cap.get(6).unwrap().as_str())?;

        return build_point(lat, ns, lng, ew);
    }

    if let Some(cap) = RE_NS_DM_EW_DM.captures(s) {
        let ns = NS::try_from(cap.get(1).unwrap().as_str())?;

        let lat = DMS {
            d: cap.get(2).unwrap().as_str().parse()?,
            m: cap.get(3).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        let ew = EW::try_from(cap.get(4).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(5).unwrap().as_str().parse()?,
            m: cap.get(6).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        return build_point(lat, ns, lng, ew);
    }

    if let Some(cap) = RE_DM_DM.captures(s) {
        let ns = NS::North;
        let ew = EW::East;

        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        let lng = DMS {
            d: cap.get(3).unwrap().as_str().parse()?,
            m: cap.get(4).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        return build_point(lat, ns, lng, ew);
    }

    if let Some(cap) = RE_NS_D_EW_D.captures(s) {
        let ns = NS::try_from(cap.get(1).unwrap().as_str())?;

        let lat = DMS {
            d: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ew = EW::try_from(cap.get(3).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(4).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        return build_point(lat, ns, lng, ew);
    }


    if let Some(cap) = RE_D_NS_D_EW.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ns = NS::try_from(cap.get(2).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(3).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ew = EW::try_from(cap.get(4).unwrap().as_str())?;

        return build_point(lat, ns, lng, ew);
    }


    if let Some(cap) = RE_D_D.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ns = NS::North;

        let lng = DMS {
            d: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ew = EW::East;

        return build_point(lat, ns, lng, ew);
    }

    Err(ParseErrorInternal)
}

fn do_parse_lat(s : &str) -> Result<f64, ParseErrorInternal> {
    if let Some(cap) = RE_DMS_NSEW.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse()?,
            s: cap.get(3).unwrap().as_str().parse()?
        };

        let ns = NS::try_from(cap.get(4).unwrap().as_str())?;

        return build_lat(lat, ns);
    }

    if let Some(cap) = RE_NSEW_DMS.captures(s) {
        let ns = NS::try_from(cap.get(1).unwrap().as_str())?;

        let lat = DMS {
            d: cap.get(2).unwrap().as_str().parse()?,
            m: cap.get(3).unwrap().as_str().parse()?,
            s: cap.get(4).unwrap().as_str().parse()?
        };

        return build_lat(lat, ns);
    }

    if let Some(cap) = RE_DMS.captures(s) {
        let ns = NS::North;

        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse()?,
            s: cap.get(3).unwrap().as_str().parse()?
        };

        return build_lat(lat, ns);
    }

    if let Some(cap) = RE_DM_NSEW.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        let ns = NS::try_from(cap.get(3).unwrap().as_str())?;

        return build_lat(lat, ns);
    }

    if let Some(cap) = RE_NSEW_DM.captures(s) {
        let ns = NS::try_from(cap.get(1).unwrap().as_str())?;

        let lat = DMS {
            d: cap.get(2).unwrap().as_str().parse()?,
            m: cap.get(3).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        return build_lat(lat, ns);
    }

    if let Some(cap) = RE_DM.captures(s) {
        let ns = NS::North;

        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        return build_lat(lat, ns);
    }

    if let Some(cap) = RE_NSEW_D.captures(s) {
        let ns = NS::try_from(cap.get(1).unwrap().as_str())?;

        let lat = DMS {
            d: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        return build_lat(lat, ns);
    }


    if let Some(cap) = RE_D_NSEW.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ns = NS::try_from(cap.get(2).unwrap().as_str())?;

        return build_lat(lat, ns);
    }


    if let Some(cap) = RE_D.captures(s) {
        let lat = DMS {
            d: cap.get(1).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ns = NS::North;

        return build_lat(lat, ns);
    }

    Err(ParseErrorInternal)
}

fn do_parse_lng(s : &str) -> Result<f64, ParseErrorInternal> {
    if let Some(cap) = RE_DMS_NSEW.captures(s) {
        let lng = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse()?,
            s: cap.get(3).unwrap().as_str().parse()?
        };

        let ew = EW::try_from(cap.get(4).unwrap().as_str())?;

        return build_lng(lng, ew);
    }

    if let Some(cap) = RE_NSEW_DMS.captures(s) {
        let ew = EW::try_from(cap.get(1).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(2).unwrap().as_str().parse()?,
            m: cap.get(3).unwrap().as_str().parse()?,
            s: cap.get(4).unwrap().as_str().parse()?
        };

        return build_lng(lng, ew);
    }

    if let Some(cap) = RE_DMS.captures(s) {
        let ew = EW::East;

        let lng = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse()?,
            s: cap.get(3).unwrap().as_str().parse()?
        };

        return build_lng(lng, ew);
    }

    if let Some(cap) = RE_DM_NSEW.captures(s) {
        let lng = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        let ew = EW::try_from(cap.get(3).unwrap().as_str())?;

        return build_lng(lng, ew);
    }

    if let Some(cap) = RE_NSEW_DM.captures(s) {
        let ew = EW::try_from(cap.get(1).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(2).unwrap().as_str().parse()?,
            m: cap.get(3).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        return build_lng(lng, ew);
    }

    if let Some(cap) = RE_DM.captures(s) {
        let ew = EW::East;

        let lng = DMS {
            d: cap.get(1).unwrap().as_str().parse()?,
            m: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            s: 0.
        };

        return build_lng(lng, ew);
    }

    if let Some(cap) = RE_NSEW_D.captures(s) {
        let ew = EW::try_from(cap.get(1).unwrap().as_str())?;

        let lng = DMS {
            d: cap.get(2).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        return build_lng(lng, ew);
    }


    if let Some(cap) = RE_D_NSEW.captures(s) {
        let lng = DMS {
            d: cap.get(1).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ew = EW::try_from(cap.get(2).unwrap().as_str())?;

        return build_lng(lng, ew);
    }


    if let Some(cap) = RE_D.captures(s) {
        let lng = DMS {
            d: cap.get(1).unwrap().as_str().parse_allow_comma()?,
            m: 0.,
            s: 0.
        };

        let ew = EW::East;

        return build_lng(lng, ew);
    }

    Err(ParseErrorInternal)
}


#[cfg(test)]
mod tests {
    use crate::{parse, parse_lat, parse_lng};
    use geo_types::Point;

    #[test]
    fn dms_ns_dms_ew() {
        let reference = Point::new(-79.98222222222222, 40.44611111111111);

        assert_eq!(reference, parse(r#"40° 26′ 46″ N 79° 58′ 56″ W"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"40° 26’ 46″ N 79° 58’ 56″ W"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse(r#"40° 26′ 46″ N, 79° 58′ 56″ W"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"40° 26′ 46″ N; 79° 58′ 56″ W"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"40° 26′ 46″ N,79° 58′ 56″ W"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40° 26′ 46″ N;79° 58′ 56″ W"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40° 26′ 46″ N ,79° 58′ 56″ W"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40° 26′ 46″ N ;79° 58′ 56″ W"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40°26′46″N79°58′56″W"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"40°26′46N79°58′56W"#).unwrap(), "compact, no sec mark");
        assert_eq!(reference, parse(r#"40 26 46 N 79 58 56 W"#).unwrap(), "no symbols");
        assert_eq!(reference, parse(r#"-40 26 46 S -79 58 56 E"#).unwrap(), "inverted");
    }

    #[test]
    fn ns_dms_ew_dms() {
        let reference = Point::new(-79.98222222222222, 40.44611111111111);

        assert_eq!(reference, parse(r#"N 40° 26′ 46″ W 79° 58′ 56″"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"N 40° 26’ 46″ W 79° 58’ 56″"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse(r#"N 40° 26′ 46″, W 79° 58′ 56″"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"N 40° 26′ 46″; W 79° 58′ 56″"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"N 40° 26′ 46″,W 79° 58′ 56″"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"N 40° 26′ 46″;W 79° 58′ 56″"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"N 40° 26′ 46″ , W 79° 58′ 56″"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"N 40° 26′ 46″ ; W 79° 58′ 56″"#).unwrap(), "semi3");
        assert_eq!(reference, parse(r#"N40°26′46″W79°58′56″"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"N40°26′46W79°58′56"#).unwrap(), "compact, no sec mark");
        assert_eq!(reference, parse(r#"N 40 26 46 W 79 58 56"#).unwrap(), "no symbols");
        assert_eq!(reference, parse(r#"S -40 26 46 E -79 58 56"#).unwrap(), "inverted");
    }

    #[test]
    fn dms_dms() {
        let reference = Point::new(79.98222222222222, 40.44611111111111);

        assert_eq!(reference, parse(r#"40° 26′ 46″ 79° 58′ 56″"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"40° 26’ 46″ 79° 58’ 56″"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse(r#"40° 26′ 46″, 79° 58′ 56″"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"40° 26′ 46″; 79° 58′ 56″"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"40° 26′ 46″,79° 58′ 56″"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40° 26′ 46″;79° 58′ 56″"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40° 26′ 46″ , 79° 58′ 56″"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"40° 26′ 46″ ; 79° 58′ 56″"#).unwrap(), "semi3");
        assert_eq!(reference, parse(r#"40°26′46″79°58′56″"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"40 26 46 79 58 56"#).unwrap(), "no symbols");
    }

    #[test]
    fn dm_ns_dm_ew() {
        let reference = Point::new(-79.98221666666667, 40.44055);

        assert_eq!(reference, parse(r#"40° 26.433′ N 79° 58.933′ W"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"40° 26,433′ N 79° 58,933′ W"#).unwrap(), "comma dec");
        assert_eq!(reference, parse(r#"40° 26.433’ N 79° 58.933’ W"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse(r#"40° 26.433′ N, 79° 58.933′ W"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"40° 26,433′ N, 79° 58,933′ W"#).unwrap(), "comma dec and comma sep");
        assert_eq!(reference, parse(r#"40° 26.433′ N; 79° 58.933′ W"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"40° 26.433′ N, 79° 58.933′ W"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40° 26.433′ N; 79° 58.933′ W"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40° 26.433 N,79° 58.933′ W"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"40° 26.433 N;79° 58.933′ W"#).unwrap(), "semi3");
        assert_eq!(reference, parse(r#"40° 26.433′N , 79° 58.933′ W"#).unwrap(), "comma4");
        assert_eq!(reference, parse(r#"40° 26.433′N ; 79° 58.933′ W"#).unwrap(), "semi4");
        assert_eq!(reference, parse(r#"40°26.433′N79°58.933′W"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"40°26.433N79°58.933W"#).unwrap(), "compact, no min mark");
        assert_eq!(reference, parse(r#"40 26.433 N 79 58.933 W"#).unwrap(), "no symbols");
        assert_eq!(reference, parse(r#"-40 26.433 S -79 58.933 E"#).unwrap(), "inverted");
    }

    #[test]
    fn ns_dm_ew_dm() {
        let reference = Point::new(-79.98221666666667, 40.44055);

        assert_eq!(reference, parse(r#"N 40° 26.433′ W 79° 58.933′"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"N 40° 26.433’ W 79° 58.933’"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse(r#"N 40° 26,433′ W 79° 58,933′"#).unwrap(), "comma dec");
        assert_eq!(reference, parse(r#"N 40° 26.433′, W 79° 58.933′"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"N 40° 26,433′, W 79° 58,933′"#).unwrap(), "comma dec and comma sep");
        assert_eq!(reference, parse(r#"N 40° 26.433′; W 79° 58.933′"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"N 40° 26.433′, W 79° 58.933′"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"N 40° 26.433′; W 79° 58.933′"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"N 40° 26.433 ,W 79° 58.933′"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"N 40° 26.433 ;W 79° 58.933′"#).unwrap(), "semi3");
        assert_eq!(reference, parse(r#"N 40° 26.433′ , W 79° 58.933′"#).unwrap(), "comma4");
        assert_eq!(reference, parse(r#"N 40° 26.433′ ; W 79° 58.933′"#).unwrap(), "semi4");
        assert_eq!(reference, parse(r#"N40°26.433′W79°58.933′"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"N40°26.433W79°58.933"#).unwrap(), "compact, no min mark");
        assert_eq!(reference, parse(r#"N40 26.433W79 58.933"#).unwrap(), "no symbols");
        assert_eq!(reference, parse(r#"S -40 26.433 E -79 58.933"#).unwrap(), "inverted");
    }

    #[test]
    fn dm_dm() {
        let reference = Point::new(79.98221666666667, 40.44055);

        assert_eq!(reference, parse(r#"40° 26.433′ 79° 58.933′"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"40° 26.433’ 79° 58.933’"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse(r#"40° 26,433′ 79° 58,933′"#).unwrap(), "comma dec");
        assert_eq!(reference, parse(r#"40° 26.433′, 79° 58.933′"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"40° 26,433′, 79° 58,933′"#).unwrap(), "comma dec and comma sep");
        assert_eq!(reference, parse(r#"40° 26.433′; 79° 58.933′"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"40° 26.433′, 79° 58.933′"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40° 26.433′; 79° 58.933′"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40° 26.433 ,79° 58.933′"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"40° 26.433 ;79° 58.933′"#).unwrap(), "semi3");
        assert_eq!(reference, parse(r#"40° 26.433′ , 79° 58.933′"#).unwrap(), "comma4");
        assert_eq!(reference, parse(r#"40° 26.433′ ; 79° 58.933′"#).unwrap(), "semi4");
        assert_eq!(reference, parse(r#"40°26.433′79°58.933′"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"40 26.433 79 58.933"#).unwrap(), "no symbols");
    }

    #[test]
    fn d_ns_d_ew() {
        let reference = Point::new(-79.9822, 40.44055);

        assert_eq!(reference, parse(r#"40.44055° N 79.9822° W"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"40,44055° N 79,9822° W"#).unwrap(), "comma dec");
        assert_eq!(reference, parse(r#"40.44055 N 79.9822 W"#).unwrap(), "no deg");
        assert_eq!(reference, parse(r#"40.44055° N, 79.9822° W"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"40,44055° N, 79,9822° W"#).unwrap(), "comma comma");
        assert_eq!(reference, parse(r#"40.44055° N; 79.9822° W"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"40.44055° N,79.9822° W"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40.44055° N;79.9822° W"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40.44055° N ,79.9822° W"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"40.44055° N ;79.9822° W"#).unwrap(), "semi3");
        assert_eq!(reference, parse(r#"40.44055N79.9822W"#).unwrap(), "compact");
        assert_eq!(reference, parse(r#"-40.44055° S -79.9822° E"#).unwrap(), "inverted");
    }

    #[test]
    fn d_d() {
        let reference = Point::new(79.9822, 40.44055);

        assert_eq!(reference, parse(r#"40.44055° 79.9822°"#).unwrap(), "normal");
        assert_eq!(reference, parse(r#"40,44055° 79,9822°"#).unwrap(), "comma dec");
        assert_eq!(reference, parse(r#"40.44055 79.9822"#).unwrap(), "no deg");
        assert_eq!(reference, parse(r#"40.44055°, 79.9822°"#).unwrap(), "comma");
        assert_eq!(reference, parse(r#"40,44055°, 79,9822°"#).unwrap(), "comma comma");
        assert_eq!(reference, parse(r#"40.44055°; 79.9822°"#).unwrap(), "semi");
        assert_eq!(reference, parse(r#"40.44055°,79.9822°"#).unwrap(), "comma2");
        assert_eq!(reference, parse(r#"40.44055°;79.9822°"#).unwrap(), "semi2");
        assert_eq!(reference, parse(r#"40.44055° ,79.9822°"#).unwrap(), "comma3");
        assert_eq!(reference, parse(r#"40.44055° ;79.9822°"#).unwrap(), "semi3");
    }


    // ------ lat / lng separate ------

    #[test]
    fn dms_nsew() {
        let reference = 40.44611111111111;

        assert_eq!(reference, parse_lat(r#"40° 26′ 46″ N"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"40° 26’ 46″ N"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lat(r#"40°26′46″N"#).unwrap(), "compact");
        assert_eq!(reference, parse_lat(r#"40°26′46N"#).unwrap(), "compact, no sec mark");
        assert_eq!(reference, parse_lat(r#"40 26 46 N"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lat(r#"-40 26 46 S"#).unwrap(), "inverted");

        assert_eq!(reference, parse_lng(r#"40° 26′ 46″ E"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"40° 26’ 46″ E"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lng(r#"40°26′46″E"#).unwrap(), "compact");
        assert_eq!(reference, parse_lng(r#"40°26′46E"#).unwrap(), "compact, no sec mark");
        assert_eq!(reference, parse_lng(r#"40 26 46 E"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lng(r#"-40 26 46 W"#).unwrap(), "inverted");
    }

    #[test]
    fn nsew_dms() {
        let reference = 40.44611111111111;

        assert_eq!(reference, parse_lat(r#"N 40° 26′ 46″"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"N 40° 26’ 46″"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lat(r#"N40°26′46"#).unwrap(), "compact, no sec mark");
        assert_eq!(reference, parse_lat(r#"N 40 26 46"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lat(r#"S -40 26 46"#).unwrap(), "inverted");

        assert_eq!(reference, parse_lng(r#"E 40° 26′ 46″"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"E 40° 26’ 46″"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lng(r#"E40°26′46"#).unwrap(), "compact, no sec mark");
        assert_eq!(reference, parse_lng(r#"E 40 26 46"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lng(r#"W -40 26 46"#).unwrap(), "inverted");
    }

    #[test]
    fn dms() {
        let reference = 40.44611111111111;
        let ref_neg = -reference;

        assert_eq!(reference, parse_lat(r#"40° 26′ 46″"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"40° 26’ 46″"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lat(r#"40°26′46″"#).unwrap(), "compact");
        assert_eq!(reference, parse_lat(r#"40 26 46"#).unwrap(), "no symbols");
        assert_eq!(ref_neg, parse_lat(r#"-40° 26′ 46″"#).unwrap(), "neg");

        assert_eq!(reference, parse_lng(r#"40° 26′ 46″"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"40° 26’ 46″"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lng(r#"40°26′46″"#).unwrap(), "compact");
        assert_eq!(reference, parse_lng(r#"40 26 46"#).unwrap(), "no symbols");
        assert_eq!(ref_neg, parse_lng(r#"-40° 26′ 46″"#).unwrap(), "neg");
    }

    #[test]
    fn dm_nsew() {
        let reference = 40.44055;

        assert_eq!(reference, parse_lat(r#"40° 26.433′ N"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"40° 26,433′ N"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lat(r#"40° 26.433’ N"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lat(r#"40°26.433′N"#).unwrap(), "compact");
        assert_eq!(reference, parse_lat(r#"40°26.433N"#).unwrap(), "compact, no min mark");
        assert_eq!(reference, parse_lat(r#"40 26.433 N"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lat(r#"-40 26.433 S"#).unwrap(), "inverted");

        assert_eq!(reference, parse_lng(r#"40° 26.433′ E"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"40° 26,433′ E"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lng(r#"40° 26.433’ E"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lng(r#"40°26.433′E"#).unwrap(), "compact");
        assert_eq!(reference, parse_lng(r#"40°26.433E"#).unwrap(), "compact, no min mark");
        assert_eq!(reference, parse_lng(r#"40 26.433 E"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lng(r#"-40 26.433 W"#).unwrap(), "inverted");
    }

    #[test]
    fn nsew_dm() {
        let reference = 40.44055;

        assert_eq!(reference, parse_lat(r#"N 40° 26.433′"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"N 40° 26.433’"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lat(r#"N 40° 26,433′"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lat(r#"N40°26.433′"#).unwrap(), "compact");
        assert_eq!(reference, parse_lat(r#"N40°26.433"#).unwrap(), "compact, no min mark");
        assert_eq!(reference, parse_lat(r#"N40 26.433"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lat(r#"S -40 26.433"#).unwrap(), "inverted");

        assert_eq!(reference, parse_lng(r#"E 40° 26.433′"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"E 40° 26.433’"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lng(r#"E 40° 26,433′"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lng(r#"E40°26.433′"#).unwrap(), "compact");
        assert_eq!(reference, parse_lng(r#"E40°26.433"#).unwrap(), "compact, no min mark");
        assert_eq!(reference, parse_lng(r#"E40 26.433"#).unwrap(), "no symbols");
        assert_eq!(reference, parse_lng(r#"W -40 26.433"#).unwrap(), "inverted");
    }

    #[test]
    fn dm() {
        let reference = 40.44055;
        let ref_neg = -reference;

        assert_eq!(reference, parse_lat(r#"40° 26.433′"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"40° 26.433’"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lat(r#"40° 26,433′"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lat(r#"40°26.433′"#).unwrap(), "compact");
        assert_eq!(reference, parse_lat(r#"40 26.433"#).unwrap(), "no symbols");
        assert_eq!(ref_neg, parse_lat(r#"-40° 26.433′"#).unwrap(), "neg");

        assert_eq!(reference, parse_lng(r#"40° 26.433′"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"40° 26.433’"#).unwrap(), "fancy apos");
        assert_eq!(reference, parse_lng(r#"40° 26,433′"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lng(r#"40°26.433′"#).unwrap(), "compact");
        assert_eq!(reference, parse_lng(r#"40 26.433"#).unwrap(), "no symbols");
        assert_eq!(ref_neg, parse_lng(r#"-40° 26.433′"#).unwrap(), "neg");
    }

    #[test]
    fn d_nsew() {
        let reference = 40.44055;
        assert_eq!(reference, parse_lat(r#"40.44055° N"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"40,44055° N"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lat(r#"40.44055 N"#).unwrap(), "no deg");
        assert_eq!(reference, parse_lat(r#"40.44055N"#).unwrap(), "compact");
        assert_eq!(reference, parse_lat(r#"-40.44055° S"#).unwrap(), "inverted");

        assert_eq!(reference, parse_lng(r#"40.44055° E"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"40,44055° E"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lng(r#"40.44055 E"#).unwrap(), "no deg");
        assert_eq!(reference, parse_lng(r#"40.44055E"#).unwrap(), "compact");
        assert_eq!(reference, parse_lng(r#"-40.44055° W"#).unwrap(), "inverted");
    }

    #[test]
    fn d() {
        let reference = 40.44055;
        let ref_neg = -reference;

        assert_eq!(reference, parse_lat(r#"40.44055°"#).unwrap(), "normal");
        assert_eq!(reference, parse_lat(r#"40,44055°"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lat(r#"40.44055"#).unwrap(), "no deg");
        assert_eq!(ref_neg, parse_lat(r#"-40.44055"#).unwrap(), "no deg");

        assert_eq!(reference, parse_lng(r#"40.44055°"#).unwrap(), "normal");
        assert_eq!(reference, parse_lng(r#"40,44055°"#).unwrap(), "comma dec");
        assert_eq!(reference, parse_lng(r#"40.44055"#).unwrap(), "no deg");
        assert_eq!(ref_neg, parse_lng(r#"-40.44055"#).unwrap(), "no deg");
    }
}
