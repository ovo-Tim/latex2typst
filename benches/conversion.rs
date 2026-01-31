use criterion::{black_box, criterion_group, criterion_main, Criterion};
use latex2typst::convert;

fn benchmark_simple_conversion(c: &mut Criterion) {
    let input = "# Hello World\n\nSome text with $x^2$ math.";

    c.bench_function("simple conversion", |b| {
        b.iter(|| {
            // This will fail for now since convert is not implemented
            // but the benchmark structure is in place
            let _ = convert(black_box(input));
        });
    });
}

criterion_group!(benches, benchmark_simple_conversion);
criterion_main!(benches);
