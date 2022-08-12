use fieldless_enum_tools::{All, FromToStr};

#[derive(Debug, FromToStr, All, PartialEq, Eq)]
#[fromtostr(format(style = "delimitedlower", separator = "❤️.❤️"))]
enum CoolEnum {
    #[fromtostr(aliases("variant_number_one", "VariantNumberOne"))]
    VariantNumberOne,
    VariantNumberTwo,
}

fn main() {
    assert_eq!("variant_number_one".parse(), Ok(CoolEnum::VariantNumberOne));
    assert_eq!("VariantNumberOne".parse(), Ok(CoolEnum::VariantNumberOne));
    assert_eq!(
        "variant❤️.❤️number❤️.❤️one".parse(),
        Ok(CoolEnum::VariantNumberOne)
    );
}
