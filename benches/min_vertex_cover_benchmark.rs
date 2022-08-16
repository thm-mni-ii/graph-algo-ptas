use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use graph_algo_ptas::algorithm::dynamic_programming::solve::{dp_solve, DpProblem};
use graph_algo_ptas::algorithm::ptas::ptas;
use graph_algo_ptas::generation::planar::generate;

fn min_vertex_cover_ptas_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("PTAS | eps=1/2 | Minimum Vertex Cover");
    group.sample_size(10);
    let mut i = 0;

    for n in [500, 1000, 2000, 4000, 8000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                i += 1;
                ptas(
                    &generate(n as usize, Some(i)).to_pet_graph(),
                    &DpProblem::min_vertex_cover(),
                    1.0 / 2.0,
                )
            });
        });
    }
    group.finish();
}

fn min_vertex_cover_ptas_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("PTAS | eps=1/3 | Minimum Vertex Cover");
    group.sample_size(10);
    let mut i = 0;

    for n in [250, 500, 1000, 2000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                i += 1;
                ptas(
                    &generate(n as usize, Some(i)).to_pet_graph(),
                    &DpProblem::min_vertex_cover(),
                    1.0 / 3.0,
                )
            });
        });
    }
    group.finish();
}

fn min_vertex_cover_dp(c: &mut Criterion) {
    let mut group = c.benchmark_group("DP | Minimum Vertex Cover");
    group.sample_size(10);
    let mut i = 0;

    for n in [40, 80, 120, 160] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                i += 1;
                dp_solve(
                    &generate(n as usize, Some(i)).to_pet_graph(),
                    None,
                    &DpProblem::min_vertex_cover(),
                )
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    min_vertex_cover_ptas_1,
    min_vertex_cover_ptas_2,
    min_vertex_cover_dp
);
criterion_main!(benches);
