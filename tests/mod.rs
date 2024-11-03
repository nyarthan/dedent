use dedent::dedent;

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/valid.rs");
    t.compile_fail("tests/trybuild/non-string-literal.rs")
}

#[test]
fn preserves_relative_indents() {
    let str = dedent!(
        "
        least indented line
            this is indented 1 tab
    "
    );

    assert_eq!(str, "least indented line\n    this is indented 1 tab")
}

#[test]
fn trims_leading_trailing_whitespace() {
    let str = dedent!(
        "


        lots of whitespace


    "
    );

    assert_eq!(str, "lots of whitespace")
}

#[test]
fn nothing_to_do() {
    let str = dedent!("doesn't really do anything");

    assert_eq!(str, "doesn't really do anything")
}

#[test]
fn empty_string() {
    let str = dedent!("");

    assert_eq!(str, "");
}

#[test]
fn produces_str_reference() {
    let str: &str = dedent!("");

    assert_eq!(str, "");
}
