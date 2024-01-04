use codspeed_criterion_compat::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

pub fn intern_and_lookup(c: &mut Criterion) {
    let sizes = [10, 100, 1000, 10000];

    for size in &sizes {
        let input = random_string(*size);

        c.bench_with_input(BenchmarkId::new("intern", size), size, |b, &_size| {
            b.iter(|| {
                let mut intern = string_interner::Intern::new();
                let id = intern.intern(black_box(&input));
                black_box(intern.lookup(id));
            })
        });
    }
}

criterion_group!(benches, intern_and_lookup);
criterion_main!(benches);
