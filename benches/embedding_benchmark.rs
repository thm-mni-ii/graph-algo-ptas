use criterion::{criterion_group, criterion_main, Criterion};
use graph_algo_ptas::embedding::index::Embedding;
use graph_algo_ptas::embedding::maximal_planar::index::MaximalPlanar;
use graph_algo_ptas::generation::planar::generate;

fn embedding_benchmark(c: &mut Criterion) {
    let mut i = 0;
    c.bench_function("embed", |b| {
        b.iter(|| {
            MaximalPlanar::embed(
                generate(
                    10,
                    Some({
                        i += 1;
                        i
                    }),
                )
                .to_pet_graph(),
            )
        })
    });
}

criterion_group!(benches, embedding_benchmark);
criterion_main!(benches);
