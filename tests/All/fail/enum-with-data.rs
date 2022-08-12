use fieldless_enum_tools::All;

#[derive(All)]
enum EnumWithData {
    A(u8),
    B { field1: u8, field2: u8 },
}

fn main() {}
