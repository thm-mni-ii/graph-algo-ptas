use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph_algo_ptas::embeding::index::Embeding;
use graph_algo_ptas::embeding::maximal_planar::index::MaximalPlanar;
use graph_algo_ptas::generation::planar::generate;

fn embeding_benchmark(c: &mut Criterion) {
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

criterion_group!(benches, embeding_benchmark);
criterion_main!(benches);
