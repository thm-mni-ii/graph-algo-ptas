use criterion::{criterion_group, criterion_main, Criterion, black_box, Bencher};
use graph_algo_ptas::generation::erdos_renyi::generate_petgraph;
use graph_algo_ptas::generation::planar::generate;

fn bench_planar_generation(n: usize) -> impl Fn(&mut Bencher) {
    move |b| {
        b.iter(|| {
            generate(
                black_box(n),
                black_box(Some(44))
            ).to_pet_graph();
        })
    }
}

fn bench_er_generation(n: usize) -> impl Fn(&mut Bencher) {
    move |b| {
        b.iter(|| {
            generate_petgraph(
                black_box(n),
                black_box(0.5),
                black_box(Some(44))
            );
        })
    }
}

fn generation_benchmark(c: &mut Criterion) {
    let mut pgg = c.benchmark_group("Planar Generation");
    pgg.bench_function("Planar Generation (|G|=10)", bench_planar_generation(10));
    pgg.bench_function("Planar Generation (|G|=100)", bench_planar_generation(100));
    pgg.bench_function("Planar Generation (|G|=1000)", bench_planar_generation(1000));
    drop(pgg);

    let mut erg = c.benchmark_group("Erdos Renyi Generation");
    erg.bench_function("Erdos Renyi Generation (|G|=10)", bench_er_generation(10));
    erg.bench_function("Erdos Renyi Generation (|G|=100)", bench_er_generation(100));
    erg.bench_function("Erdos Renyi Generation (|G|=1000)", bench_er_generation(1000));
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets = generation_benchmark
}
criterion_main!(benches);
