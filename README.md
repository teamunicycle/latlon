# latlon

Parse geographic coordinates from string.

A wide range of commonly used formats is supported.
See the unit tests for a complete reference.

If a format you need is missing, please submit a merge request (including unit tests).

## Usage

```rust
// parse a coord
let coord : geo::Point = latlon::parse("N 50°5.30385', E 14°26.94732'").unwrap();

// individual lat/lng parsing
let lat : f64 = latlon::parse_lat("N 50°5.30385'").unwrap();
let lng : f64 = latlon::parse_lng("E 14°26.94732'").unwrap();
```

## Supported formats

Example of supported formats:

- `40° 26' 46" N 79° 58' 56" W`
- `N 40° 26' 46" W 79° 58' 56"`
- `40° 26.767' N 79° 58.933' W`
- `40° 26' 46" 79° 58' 56"`, `40° 26' 46", 79° 58' 56"`, ...
- `N 40° 26.767' W 79° 58.933'`
- `40° 26.767' 79° 58.933'`, `40° 26.767', 79° 58.933'`, ...
- `N 40.446° W 79.982°`
- `40.446° N 79.982° W`
- `40.446° 79.982°`, `40.446,79.982`, etc.

## Parser rules
- All formats support negative degrees (preceded by a minus sign). Positive latitude is North, positive longitude is East.
- Whitespace is optional and ignored, except for formats that would become unparsable.
- Degree, minute and second symbols can be omitted.
- Comma (`,`) may be used as an alternate decimal separator.
- Unicode quotes (e.g. `’`, `”`) are supported
  for minutes and seconds.
- The two coordinates can be separated by comma (`,`), semicolon (`;`), whitespace, or nothing
  at all, if not ambiguous.
