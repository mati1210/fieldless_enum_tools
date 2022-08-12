use fieldless_enum_tools::All;

#[derive(Debug, All, PartialEq, Eq)]
enum MyCoolEnum {
    A,
    B,
    C,
    D,
}
fn main() {
    assert_eq!(
        MyCoolEnum::all(),
        [MyCoolEnum::A, MyCoolEnum::B, MyCoolEnum::C, MyCoolEnum::D]
    )
}
