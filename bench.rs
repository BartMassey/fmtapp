use criterion::*;
use rand::prelude::*;
use fmtapp::*;

fn rand_string(alphabet: &[char]) -> String {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..=20);
    (0..n)
        .map(|_| {
            alphabet.choose(&mut rng).unwrap()
        })
        .collect()
}

fn gen_rand_strings(n: usize) -> Vec<(String, String)> {
    let alphabet: Vec<char> = ('a'..='z').collect();
    (0..n)
        .map(|_| {
            (rand_string(&alphabet), rand_string(&alphabet))
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let strings = gen_rand_strings(10);
    c.bench_function("fmt", |b| {
        for (l, r) in &strings {
            b.iter(|| app_fmt(l, r));
        }
    });
    c.bench_function("clone", |b| {
        for (l, r) in &strings {
            b.iter(|| app_clone(l, r));
        }
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
