use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unicode_branchless::branchless_utf8;

pub fn criterion_benchmark(c: &mut Criterion) {
    let benchpoints = [
        0x7f,
        0xef,
        0x1400,
        '🦀' as u32,
    ];
    for i in benchpoints {
        let name = format!("branchless_utf8 0x{i:04x}");
        c.bench_function(
            &name,
            |b| b.iter(|| {
                branchless_utf8(
                    black_box(unsafe { char::from_u32_unchecked(i) }),
                )
            }),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
