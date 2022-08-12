use fieldless_enum_tools::FromToStr;

#[derive(FromToStr)]
enum Enum {
    VariantOne,
    #[fromtostr(rename("VariantOne"))]
    VariantTwo,
}

fn main() {}
