use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("solutions");
    for (i, solution) in aoc2018::SOLUTIONS.iter().enumerate() {
        let day = i + 1;
        group.bench_function(format!("Day {day} Part 1"), |b| {
            b.iter(|| black_box((solution.part1)(black_box(solution.input))));
        });
        group.bench_function(format!("Day {day} Part 2"), |b| {
            b.iter(|| black_box((solution.part2)(black_box(solution.input))));
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
