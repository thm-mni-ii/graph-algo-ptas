use criterion::{criterion_group, criterion_main, Criterion, black_box, Bencher};
use graph_algo_ptas::embedding::index::Embedding;
use graph_algo_ptas::embedding::maximal_planar::index::MaximalPlanar;
use graph_algo_ptas::generation::planar::generate;
use std::time::Duration;

fn bench_embedding(n: usize) -> impl Fn(&mut Bencher) {
    move |b| {
        let graph = generate(
            n,
            Some(44)
        ).to_pet_graph();
        b.iter(|| {
            MaximalPlanar::embed(
                black_box(graph.clone())
            )
        })
    }
}

fn embedding_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("MaximalPlanar embedding");
    g.bench_function("MaximalPlanar embedding (|G|=10)", bench_embedding(10));
    g.bench_function("MaximalPlanar embedding (|G|=100)", bench_embedding(100));
    g.bench_function("MaximalPlanar embedding (|G|=1000)", bench_embedding(1000));
}

criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = embedding_benchmark
}
criterion_main!(benches);
