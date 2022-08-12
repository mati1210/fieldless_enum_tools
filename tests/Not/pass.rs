use fieldless_enum_tools::Not;

#[derive(Not, Debug, PartialEq, Eq, Clone, Copy)]
enum OneVariant {
    A,
}

#[derive(Not, Debug, PartialEq, Eq, Clone, Copy)]
enum TwoVariants {
    A,
    B,
}

#[derive(Not, Debug, PartialEq, Eq, Clone, Copy)]
enum TwoVariantsWithAttrb {
    #[not(A)]
    A,
    #[not(A)]
    B,
}

#[derive(Not, Debug, PartialEq, Eq, Clone, Copy)]
enum MultipleVariants {
    #[not(OppositeOfA)]
    A,
    #[not(OppositeOfB)]
    B,
    #[not(OppositeOfC)]
    C,
    #[not(A)]
    OppositeOfA,
    #[not(B)]
    OppositeOfB,
    #[not(C)]
    OppositeOfC,
}

fn main() {
    assert_eq!(!OneVariant::A, OneVariant::A);
    assert_eq!(!TwoVariants::A, TwoVariants::B);
    assert_eq!(!TwoVariantsWithAttrb::A, TwoVariantsWithAttrb::A);
    assert_eq!(!TwoVariantsWithAttrb::B, TwoVariantsWithAttrb::A);
    assert_eq!(!MultipleVariants::A, MultipleVariants::OppositeOfA);
    assert_eq!(!MultipleVariants::OppositeOfB, MultipleVariants::B);
    assert_eq!(!MultipleVariants::C, MultipleVariants::OppositeOfC);
}
