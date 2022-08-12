use fieldless_enum_tools::Not;

#[derive(Not)]
enum EmptyNot {
    #[not]
    A,
    #[not(A)]
    B,
}

fn main() {}
