use criterion::*;
use rand::prelude::*;
use fmtapp::*;

fn rand_string(alphabet: &[char], n: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            alphabet.choose(&mut rng).unwrap()
        })
        .collect()
}

fn gen_rand_strings(n: usize) -> Vec<(String, String)> {
    let alphabet: Vec<char> = ('a'..='z').collect();
    (0..n)
        .map(|i| {
            (rand_string(&alphabet, 5 * i), rand_string(&alphabet, 5 * i))
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let strings = gen_rand_strings(10);
    let mut g = c.benchmark_group("app");
    for (i, pair) in strings.as_slice().iter().enumerate() {
        g.bench_with_input(
            BenchmarkId::new("fmt", i),
            pair,
            |b, (l, r)| b.iter(|| app_fmt(l, r))
        );
        g.bench_with_input(
            BenchmarkId::new("clone", i),
            pair,
            |b, (l, r)| b.iter(|| app_clone(l, r))
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
