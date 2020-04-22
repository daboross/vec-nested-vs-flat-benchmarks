use criterion::{black_box, criterion_group, criterion_main, Criterion};

const OUTER_VEC_SIZE: usize = 10000000;

fn alloc_flat() -> Vec<u16> {
    vec![0; 60 * OUTER_VEC_SIZE]
}

fn loop_flat(v: &Vec<u16>) {
    let mut _counter = 0;
    for _i in 0..OUTER_VEC_SIZE {
        for _j in 0..60 {
            _counter += v[_j * OUTER_VEC_SIZE + _i];
        }
    }
    black_box(_counter);
}

fn loop_flat_explicit_size_assert(v: &Vec<u16>) {
    // This helps the compiler elide bounds checks when doing non-Iterator-based iteration:
    // if the bounds are broken, it explicitly panics early, so it shouldn't need to test
    // those bounds later.
    assert!(v.len() >= OUTER_VEC_SIZE);

    let mut _counter = 0;
    for _i in 0..OUTER_VEC_SIZE {
        for _j in 0..60 {
            _counter += v[_j * OUTER_VEC_SIZE + _i];
        }
    }
    black_box(_counter);
}

fn alloc_nested() -> Vec<Vec<u16>> {
    vec![vec![0; 60]; 10000000]
}

fn loop_nested(v: &Vec<Vec<u16>>) {
    let mut _counter = 0;
    for _sequence in v.iter() {
        for _turn in _sequence.iter() {
            _counter += *_turn;
        }
    }
    black_box(_counter);
}

fn benchmark_nested(c: &mut Criterion) {
    c.bench_function(&format!("nested: alloc {}", OUTER_VEC_SIZE), |b| {
        b.iter(|| alloc_nested())
    });
    let nested = alloc_nested();
    c.bench_function(&format!("nested: loop over {}", OUTER_VEC_SIZE), |b| {
    // use black_box to ensure the compiler can't tell what's inside the vec.
        b.iter(|| loop_nested(black_box(&nested)))
    });
}

fn benchmark_flat(c: &mut Criterion) {
    c.bench_function(&format!("flat: alloc {}", OUTER_VEC_SIZE), |b| {
        b.iter(|| alloc_flat())
    });
    let flat = alloc_flat();
    c.bench_function(&format!("flat: loop over {}", OUTER_VEC_SIZE), |b| {
        b.iter(|| loop_flat(black_box(&flat)))
    });
    c.bench_function(
        &format!("flat: loop over {} with explicit assert", OUTER_VEC_SIZE),
        |b| b.iter(|| loop_flat_explicit_size_assert(black_box(&flat))),
    );
}

criterion_group!(vec_benchmarks, benchmark_flat, benchmark_nested);
criterion_main!(vec_benchmarks);