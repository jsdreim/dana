#[test]
fn qty_invalid() {
    let test = trybuild::TestCases::new();

    test.compile_fail("tests/err_qty/*.rs");
}
