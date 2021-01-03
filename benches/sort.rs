use algorithms_and_datastructures::sorting_algorithms as algos;
use criterion::{black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};

pub fn already_sorted(c: &mut Criterion) {
    let max_size = 2usize.pow(31);
    let mut sequence: Vec<i64> = (0..max_size as i64).collect();

    let mut group = c.benchmark_group("Already Sorted Sequence");
    group.sample_size(10);

    for size in (0..=max_size).step_by(100_000_000) {
        group.bench_with_input(BenchmarkId::new("QuickSort", size), &size, |b, &size| {
            b.iter(|| algos::quick_sort::quick_sort(&mut sequence[0..size]));
        });

        group.bench_with_input(BenchmarkId::new("MergeSort", size), &size, |b, &size| {
            b.iter(|| algos::merge_sort::merge_sort(&mut sequence[0..size]));
        });

        group.bench_with_input(BenchmarkId::new("HeapSort", size), &size, |b, &size| {
            b.iter(|| algos::heap_sort::heap_sort(&mut sequence[0..size]));
        });
    }
}

criterion_group!(benches, already_sorted);
criterion_main!(benches);
