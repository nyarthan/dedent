use dedent::dedent;

fn main() {
    let str = dedent!(
        r"
        it should
        compile
    "
    );

    assert_eq!(str, "it should\ncompile")
}
