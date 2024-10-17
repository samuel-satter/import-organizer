mod iterations;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::iterations::{first, second, third};

fn criterion_benchmark(c: &mut Criterion) {
    let input = r#"
use std::collections::HashMap;
use std::io::{self, Read, Write};
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use crate::constants::{self, BASE_BUCKET_URL};
use crate::err::Res;
use crate::features::web::common;
// Some non-import lines
fn some_function() {
    println!("Hello, world!");
}
    "#;

    c.bench_function("first", |b| b.iter(|| first::organize_rust_imports(black_box(input))));
    c.bench_function("second", |b| b.iter(|| second::organize_rust_imports(black_box(input))));
    c.bench_function("third", |b| b.iter(|| third::organize_rust_imports(input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);