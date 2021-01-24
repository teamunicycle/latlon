use crate::{parse, parse_lat, parse_lng, GeoParseError};
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

    assert_eq!(reference, parse(r#"40° 26’ 46″ N 79° 58’ 56″ W"#).unwrap(), "q1");
    assert_eq!(reference, parse(r#"40° 26' 46” N 79° 58' 56” W"#).unwrap(), "q2");
    assert_eq!(reference, parse(r#"40° 26′ 46" N 79° 58′ 56" W"#).unwrap(), "q3");
    assert_eq!(reference, parse(r#"40° 26‘ 46“ N 79° 58‘ 56“ W"#).unwrap(), "q4");
    assert_eq!(reference, parse(r#"40° 26‛ 46″ N 79° 58‛ 56″ W"#).unwrap(), "q5");

    parse(r#"90° 0′ 0″ N 180° 0′ 0″ E"#).unwrap();
    parse(r#"90° 0′ 0″ S 180° 0′ 0″ W"#).unwrap();
    parse(r#"-90° 0′ 0″ N -180° 0′ 0″ E"#).unwrap();
    parse(r#"-90° 0′ 0″ S -180° 0′ 0″ W"#).unwrap();
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

    assert_eq!(reference, parse(r#"N 40° 26’ 46″ W 79° 58’ 56″"#).unwrap(), "q1");
    assert_eq!(reference, parse(r#"N 40° 26' 46” W 79° 58' 56”"#).unwrap(), "q2");
    assert_eq!(reference, parse(r#"N 40° 26′ 46" W 79° 58′ 56""#).unwrap(), "q3");
    assert_eq!(reference, parse(r#"N 40° 26‘ 46“ W 79° 58‘ 56“"#).unwrap(), "q4");
    assert_eq!(reference, parse(r#"N 40° 26‛ 46″ W 79° 58‛ 56″"#).unwrap(), "q5");

    parse(r#"N 90° 0′ 0″ E 180° 0′ 0″"#).unwrap();
    parse(r#"S 90° 0′ 0″ W 180° 0′ 0″"#).unwrap();
    parse(r#"N -90° 0′ 0″ E -180° 0′ 0″"#).unwrap();
    parse(r#"S -90° 0′ 0″ W -180° 0′ 0″"#).unwrap();
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

    assert_eq!(reference, parse(r#"40° 26’ 46″ 79° 58’ 56″"#).unwrap(), "q1");
    assert_eq!(reference, parse(r#"40° 26' 46” 79° 58' 56”"#).unwrap(), "q2");
    assert_eq!(reference, parse(r#"40° 26′ 46" 79° 58′ 56""#).unwrap(), "q3");
    assert_eq!(reference, parse(r#"40° 26‘ 46“ 79° 58‘ 56“"#).unwrap(), "q4");
    assert_eq!(reference, parse(r#"40° 26‛ 46″ 79° 58‛ 56″"#).unwrap(), "q5");

    parse(r#"90° 0′ 0″ 180° 0′ 0″"#).unwrap();
    parse(r#"-90° 0′ 0″ -180° 0′ 0″"#).unwrap();
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

    assert_eq!(reference, parse(r#"40° 26.433’ N 79° 58.933’ W"#).unwrap(), "q1");
    assert_eq!(reference, parse(r#"40° 26.433' N 79° 58.933' W"#).unwrap(), "q2");
    assert_eq!(reference, parse(r#"40° 26.433′ N 79° 58.933′ W"#).unwrap(), "q3");
    assert_eq!(reference, parse(r#"40° 26.433‘ N 79° 58.933‘ W"#).unwrap(), "q4");
    assert_eq!(reference, parse(r#"40° 26.433‛ N 79° 58.933‛ W"#).unwrap(), "q5");

    parse(r#"90° 0′ N 180° 0′ E"#).unwrap();
    parse(r#"90° 0′ S 180° 0′ W"#).unwrap();
    parse(r#"-90° 0′ N -180° 0′ E"#).unwrap();
    parse(r#"-90° 0′ S -180° 0′ W"#).unwrap();
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

    assert_eq!(reference, parse(r#"N 40° 26.433’ W 79° 58.933’"#).unwrap(), "q1");
    assert_eq!(reference, parse(r#"N 40° 26.433' W 79° 58.933'"#).unwrap(), "q2");
    assert_eq!(reference, parse(r#"N 40° 26.433′ W 79° 58.933′"#).unwrap(), "q3");
    assert_eq!(reference, parse(r#"N 40° 26.433‘ W 79° 58.933‘"#).unwrap(), "q4");
    assert_eq!(reference, parse(r#"N 40° 26.433‛ W 79° 58.933‛"#).unwrap(), "q5");

    parse(r#"N 90° 0′ E 180° 0′"#).unwrap();
    parse(r#"S 90° 0′ W 180° 0′"#).unwrap();
    parse(r#"N -90° 0′ E -180° 0′"#).unwrap();
    parse(r#"S -90° 0′ W -180° 0′"#).unwrap();
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

    assert_eq!(reference, parse(r#"40° 26.433’ 79° 58.933’"#).unwrap(), "q1");
    assert_eq!(reference, parse(r#"40° 26.433' 79° 58.933'"#).unwrap(), "q2");
    assert_eq!(reference, parse(r#"40° 26.433′ 79° 58.933′"#).unwrap(), "q3");
    assert_eq!(reference, parse(r#"40° 26.433‘ 79° 58.933‘"#).unwrap(), "q4");
    assert_eq!(reference, parse(r#"40° 26.433‛ 79° 58.933‛"#).unwrap(), "q5");

    parse(r#"90° 0′ 180° 0′"#).unwrap();
    parse(r#"-90° 0′ -180° 0′"#).unwrap();
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

    parse(r#"90° N 180° E"#).unwrap();
    parse(r#"90° S 180° W"#).unwrap();
    parse(r#"-90° N -180° E"#).unwrap();
    parse(r#"-90° S -180° W"#).unwrap();
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

    parse(r#"90 180"#).unwrap();
    parse(r#"-90 -180"#).unwrap();
    parse(r#"90° 180°"#).unwrap();
    parse(r#"-90° -180°"#).unwrap();
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

    assert_eq!(reference, parse_lng(r#"40° 26’ 46″ E"#).unwrap(), "q1");
    assert_eq!(reference, parse_lng(r#"40° 26' 46” E"#).unwrap(), "q2");
    assert_eq!(reference, parse_lng(r#"40° 26′ 46" E"#).unwrap(), "q3");
    assert_eq!(reference, parse_lng(r#"40° 26‘ 46“ E"#).unwrap(), "q4");
    assert_eq!(reference, parse_lng(r#"40° 26‛ 46″ E"#).unwrap(), "q5");

    parse_lat(r#"90° 0′ 0″ N"#).unwrap();
    parse_lat(r#"90° 0′ 0″ S"#).unwrap();
    parse_lng(r#"180° 0′ 0″ E"#).unwrap();
    parse_lng(r#"180° 0′ 0″ W"#).unwrap();
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

    assert_eq!(reference, parse_lng(r#"E 40° 26’ 46″"#).unwrap(), "q1");
    assert_eq!(reference, parse_lng(r#"E 40° 26' 46”"#).unwrap(), "q2");
    assert_eq!(reference, parse_lng(r#"E 40° 26′ 46""#).unwrap(), "q3");
    assert_eq!(reference, parse_lng(r#"E 40° 26‘ 46“"#).unwrap(), "q4");
    assert_eq!(reference, parse_lng(r#"E 40° 26‛ 46″"#).unwrap(), "q5");

    parse_lat(r#"N 90° 0′ 0″"#).unwrap();
    parse_lat(r#"S 90° 0′ 0″"#).unwrap();
    parse_lng(r#"E 180° 0′ 0″"#).unwrap();
    parse_lng(r#"W 180° 0′ 0″"#).unwrap();
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

    // Test all the weird quotation marks

    // ’'′‘‛
    // ″”"“
    assert_eq!(reference, parse_lng(r#"40° 26’ 46″"#).unwrap(), "q1");
    assert_eq!(reference, parse_lng(r#"40° 26' 46”"#).unwrap(), "q2");
    assert_eq!(reference, parse_lng(r#"40° 26′ 46""#).unwrap(), "q3");
    assert_eq!(reference, parse_lng(r#"40° 26‘ 46“"#).unwrap(), "q4");
    assert_eq!(reference, parse_lng(r#"40° 26‛ 46″"#).unwrap(), "q5");

    parse_lat(r#"90° 0’ 0″"#).unwrap();
    parse_lat(r#"-90° 0’ 0″"#).unwrap();
    parse_lng(r#"180° 0’ 0″"#).unwrap();
    parse_lng(r#"-180° 0’ 0″"#).unwrap();
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

    assert_eq!(reference, parse_lng(r#"40° 26.433’ E"#).unwrap(), "q1");
    assert_eq!(reference, parse_lng(r#"40° 26.433' E"#).unwrap(), "q2");
    assert_eq!(reference, parse_lng(r#"40° 26.433′ E"#).unwrap(), "q3");
    assert_eq!(reference, parse_lng(r#"40° 26.433‘ E"#).unwrap(), "q4");
    assert_eq!(reference, parse_lng(r#"40° 26.433‛ E"#).unwrap(), "q5");

    parse_lat(r#"90° 0′ N"#).unwrap();
    parse_lat(r#"90° 0′ S"#).unwrap();
    parse_lng(r#"180° 0′ E"#).unwrap();
    parse_lng(r#"180° 0′ W"#).unwrap();
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

    // quotes
    assert_eq!(reference, parse_lng(r#"E 40° 26.433’"#).unwrap(), "q1");
    assert_eq!(reference, parse_lng(r#"E 40° 26.433'"#).unwrap(), "q2");
    assert_eq!(reference, parse_lng(r#"E 40° 26.433′"#).unwrap(), "q3");
    assert_eq!(reference, parse_lng(r#"E 40° 26.433‘"#).unwrap(), "q4");
    assert_eq!(reference, parse_lng(r#"E 40° 26.433‛"#).unwrap(), "q5");


    // verify corner cases don't error
    parse_lat(r#"N 90° 0.0′"#).unwrap();
    parse_lat(r#"N -90° 0′"#).unwrap();
    parse_lat(r#"S 90° 0′"#).unwrap();
    parse_lat(r#"S -90° 0′"#).unwrap();

    parse_lng(r#"E -180° 0′"#).unwrap();
    parse_lng(r#"W 180° 0′"#).unwrap();
    parse_lng(r#"W -180° 0′"#).unwrap();
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

    // verify corner cases don't error
    parse_lat(r#"0° 0′"#).unwrap();
    parse_lat(r#"0° 0"#).unwrap();
    parse_lat(r#"90° 0"#).unwrap();
    parse_lat(r#"-90° 0"#).unwrap();

    parse_lng(r#"0° 0′"#).unwrap();
    parse_lng(r#"180° 0′"#).unwrap();
    parse_lng(r#"-180° 0′"#).unwrap();
    parse_lng(r#"-180° 0.000′"#).unwrap();
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

    // verify corner cases don't error
    parse_lat(r#"1.123456789° N"#).unwrap();
    parse_lat(r#"90° S"#).unwrap();
    parse_lat(r#"-90°S"#).unwrap();
    parse_lat(r#"1°S"#).unwrap();

    parse_lng(r#"1.123456789°E"#).unwrap();
    parse_lng(r#"180.0000°W"#).unwrap();
    parse_lng(r#"-180 E"#).unwrap();
    parse_lng(r#"1W"#).unwrap();
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

    // verify corner cases don't error
    parse_lat(r#"1.123456789°"#).unwrap();
    parse_lat(r#"90°"#).unwrap();
    parse_lat(r#"-90°"#).unwrap();
    parse_lat(r#"1°"#).unwrap();

    parse_lng(r#"1.123456789°"#).unwrap();
    parse_lng(r#"180.0000°"#).unwrap();
    parse_lng(r#"-180°"#).unwrap();
    parse_lng(r#"1°"#).unwrap();
}

#[test]
fn error() {
    let e = GeoParseError("Hello");
    assert_eq!("GeoParseError(\"Hello\")", format!("{:?}", e));
    assert_eq!("GeoParseError(\n    \"Hello\",\n)", format!("{:#?}", e));

    let e = GeoParseError("Hello2".to_string());
    assert_eq!("GeoParseError(\"Hello2\")", format!("{:?}", e));
    assert_eq!("GeoParseError(\n    \"Hello2\",\n)", format!("{:#?}", e));

    let e = GeoParseError(std::borrow::Cow::Owned("Foo".to_string()));
    assert_eq!("GeoParseError(\"Foo\")", format!("{:?}", e));
    assert_eq!("GeoParseError(\n    \"Foo\",\n)", format!("{:#?}", e));
}
