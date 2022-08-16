use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use fieldless_enum_tools_internals::{All, FromToStr, Not};

fn fromtostr(c: &mut Criterion) {
    let input: syn::DeriveInput = syn::parse_quote! {
        #[fromtostr(format(style = "delimited", separator = "😎"))]
        enum CoolEnum {
            #[fromtostr(aliases("cool_variant_one"))]
            CoolVariantOne,
            #[fromtostr(rename("Very😎Cool😎Variant😎Two"))]
            CoolVariantTwo
        }
    };

    c.bench_function("fromtostr", move |b| {
        b.iter_batched(|| input.clone(), FromToStr::main, BatchSize::SmallInput)
    });
}

fn all(c: &mut Criterion) {
    let input: syn::DeriveInput = syn::parse_quote! {
        enum LotsVariantsEnum {
            A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
        }
    };

    c.bench_function("all", move |b| {
        b.iter_batched(|| input.clone(), All::main, BatchSize::SmallInput)
    });
}

fn not(c: &mut Criterion) {
    let input: syn::DeriveInput = syn::parse_quote! {
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
    };

    c.bench_function("not", move |b| {
        b.iter_batched(|| input.clone(), Not::main, BatchSize::SmallInput)
    });
}

criterion_group![benches, fromtostr, all, not];
criterion_main!(benches);
