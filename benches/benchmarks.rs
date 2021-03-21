use criterion::{black_box, criterion_group, criterion_main, Criterion};
use todo_swamp::file_run;

// fn benchmark(file_name: &str) -> io::Result<f64> {
//     for suffix in BENCHMARK_SUFFIX.iter() {
//         let mut file_name = file_name.to_owned();
//         file_name.push_str(suffix);
//         file_run(&file_name)?;
//     }
//     Ok(4.0)
// }
//const BENCHMARK_SUFFIX: [&str; 5] = ["","_2","_3","_4","_5"];

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("10k");
    group.sample_size(10); //TODO: find out whether I need to tweak some other settings to offset loss of accuracy from reducing sample size
    group.bench_function("file_run 10k", |b| b.iter(|| file_run(black_box("tests/10k/benchmark_10k"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);