pub fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/All/pass.rs");
    t.compile_fail("tests/All/fail/*.rs");
}
