pub fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/Not/pass.rs");
    t.compile_fail("tests/Not/fail/*.rs");
}
