use {
    lazy_regex::*,
};

fn example_builds() {

    // build a simple regex
    let r = regex!("sa+$");
    assert_eq!(r.is_match("Saa"), false);

    // build a regex with flag(s)
    let r = regex!("sa+$"i);
    assert_eq!(r.is_match("Saa"), true);

    // you can use a raw literal
    let r = regex!(r#"^"+$"#);
    assert_eq!(r.is_match("\"\""), true);

    // and a raw literal with flag(s)
    let r = regex!(r#"^\s*("[a-t]*"\s*)+$"#i);
    assert_eq!(r.is_match(r#" "Aristote" "Platon" "#), true);

    // this line wouldn't compile:
    // let r = regex!("(unclosed");

}

fn example_is_match() {
    let b = regex_is_match!("[ab]+", "car");
    assert_eq!(b, true);
}

fn example_captures() {
    let (whole, name, version) = regex_captures!(
        r#"(\w+)-([0-9.]+)"#, // a literal regex
        "This is lazy_regex-2.0!", // any expression
    ).unwrap();
    assert_eq!(whole, "lazy_regex-2.0");
    assert_eq!(name, "lazy_regex");
    assert_eq!(version, "2.0");
}

fn main() {

    // the regular expressions will be built only once
    for _ in 0..10 {
        example_builds();
    }

    example_is_match();

    for _ in 0..10 {
        example_captures();
    }
}
