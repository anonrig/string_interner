use codspeed_criterion_compat::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

const SEED: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

fn random_string(n: usize) -> String {
    let mut rng = XorShiftRng::from_seed(SEED); // You can change the seed value

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect::<Vec<char>>();

    (0..n)
        .map(|_| {
            let random_index = rng.next_u32() as usize % charset.len();
            charset[random_index]
        })
        .collect()
}

fn vec_of_random_strings(string_length: usize, capacity: usize) -> Vec<String> {
    let mut out = Vec::with_capacity(capacity);
    for _x in 0..capacity {
        out.push(random_string(string_length));
    }
    out
}

pub fn intern_and_lookup(c: &mut Criterion) {
    let vec_lengths: [u64; 3] = [1000, 10000, 100000];
    let mut group = c.benchmark_group("intern_and_lookup");
    for vec_length in &vec_lengths {
        let data = vec_of_random_strings(100, *vec_length as usize);
        group.bench_with_input(
            BenchmarkId::from_parameter(vec_length),
            &data,
            |b, dataset| {
                let mut intern = intern_string::Intern::new();
                b.iter(|| {
                    for data in dataset {
                        let id = intern.intern(black_box(data));
                        black_box(intern.lookup(id));
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, intern_and_lookup);
criterion_main!(benches);
