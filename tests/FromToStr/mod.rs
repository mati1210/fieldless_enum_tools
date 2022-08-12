pub fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/FromToStr/pass.rs");
    t.compile_fail("tests/FromToStr/fail/*.rs");
}
