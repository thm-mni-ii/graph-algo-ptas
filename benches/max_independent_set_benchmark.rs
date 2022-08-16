use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use graph_algo_ptas::algorithm::dynamic_programming::solve::{dp_solve, DynamicProgrammingProblem};
use graph_algo_ptas::algorithm::ptas::ptas;
use graph_algo_ptas::generation::planar::generate;

fn max_independent_set_ptas_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("PTAS | eps=1/2 | Maximum Independent Set");
    group.sample_size(10);
    let mut i = 0;

    for n in [500, 1000, 2000, 4000, 8000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                i += 1;
                ptas(
                    &generate(n as usize, Some(i)).to_pet_graph(),
                    &DynamicProgrammingProblem::max_independent_set(),
                    1.0 / 2.0,
                )
            });
        });
    }
    group.finish();
}

fn max_independent_set_ptas_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("PTAS | eps=1/3 | Maximum Independent Set");
    group.sample_size(10);
    let mut i = 0;

    for n in [250, 500, 1000, 2000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                i += 1;
                ptas(
                    &generate(n as usize, Some(i)).to_pet_graph(),
                    &DynamicProgrammingProblem::max_independent_set(),
                    1.0 / 3.0,
                )
            });
        });
    }
    group.finish();
}

fn max_independent_set_dp(c: &mut Criterion) {
    let mut group = c.benchmark_group("DP | Maximum Independent Set");
    group.sample_size(10);
    let mut i = 0;

    for n in [40, 80, 120, 160] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                i += 1;
                dp_solve(
                    &generate(n as usize, Some(i)).to_pet_graph(),
                    None,
                    &DynamicProgrammingProblem::max_independent_set(),
                )
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    max_independent_set_ptas_1,
    max_independent_set_ptas_2,
    max_independent_set_dp
);
criterion_main!(benches);
