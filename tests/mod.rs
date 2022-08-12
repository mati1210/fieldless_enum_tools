#![allow(non_snake_case)]

mod All;
mod FromToStr;
mod Not;

#[test]
fn tests() {
    All::tests();
    Not::tests();
    FromToStr::tests();
}
