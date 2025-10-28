use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use opendb::{Memory, OpenDB, OpenDBOptions};
use std::collections::HashMap;
use std::hint::black_box;
use tempfile::TempDir;

fn kv_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("kv_operations");

    group.bench_function("put", |b| {
        let temp_dir = TempDir::new().unwrap();
        let db = OpenDB::open(temp_dir.path()).unwrap();

        b.iter(|| {
            db.put(b"test_key", b"test_value").unwrap();
        });
    });

    group.bench_function("get", |b| {
        let temp_dir = TempDir::new().unwrap();
        let db = OpenDB::open(temp_dir.path()).unwrap();
        db.put(b"test_key", b"test_value").unwrap();

        b.iter(|| {
            black_box(db.get(b"test_key").unwrap());
        });
    });

    group.finish();
}

fn memory_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");

    group.bench_function("insert_memory", |b| {
        let temp_dir = TempDir::new().unwrap();
        let options = OpenDBOptions::with_dimension(384);
        let db = OpenDB::open_with_options(temp_dir.path(), options).unwrap();
        let mut counter = 0;

        b.iter(|| {
            let memory = Memory {
                id: format!("mem_{}", counter),
                content: "Test memory content for benchmarking".to_string(),
                embedding: vec![0.1; 384],
                importance: 0.5,
                timestamp: chrono::Utc::now().timestamp(),
                metadata: HashMap::new(),
            };
            counter += 1;
            db.insert_memory(&memory).unwrap();
        });
    });

    group.bench_function("get_memory", |b| {
        let temp_dir = TempDir::new().unwrap();
        let options = OpenDBOptions::with_dimension(384);
        let db = OpenDB::open_with_options(temp_dir.path(), options).unwrap();

        // Insert test memories
        let mut ids = Vec::new();
        for i in 0..100 {
            let memory = Memory {
                id: format!("mem_{}", i),
                content: format!("Memory {}", i),
                embedding: vec![0.1; 384],
                importance: 0.5,
                timestamp: chrono::Utc::now().timestamp(),
                metadata: HashMap::new(),
            };
            ids.push(memory.id.clone());
            db.insert_memory(&memory).unwrap();
        }

        let mut idx = 0;
        b.iter(|| {
            let id = &ids[idx % ids.len()];
            idx += 1;
            black_box(db.get_memory(id).unwrap());
        });
    });

    group.finish();
}

fn vector_search_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_search");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let temp_dir = TempDir::new().unwrap();
            let options = OpenDBOptions::with_dimension(384);
            let db = OpenDB::open_with_options(temp_dir.path(), options).unwrap();

            // Insert test memories
            for i in 0..size {
                let embedding: Vec<f32> =
                    (0..384).map(|j| (i as f32 + j as f32) / 1000.0).collect();
                let memory = Memory {
                    id: format!("mem_{}", i),
                    content: format!("Test memory {}", i),
                    embedding,
                    importance: ((i % 100) as f32) / 100.0,
                    timestamp: chrono::Utc::now().timestamp(),
                    metadata: HashMap::new(),
                };
                db.insert_memory(&memory).unwrap();
            }

            let query: Vec<f32> = (0..384).map(|j| j as f32 / 1000.0).collect();

            b.iter(|| {
                black_box(db.search_similar(&query, 10).unwrap());
            });
        });
    }

    group.finish();
}

fn graph_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_operations");

    group.bench_function("link", |b| {
        let temp_dir = TempDir::new().unwrap();
        let options = OpenDBOptions::with_dimension(384);
        let db = OpenDB::open_with_options(temp_dir.path(), options).unwrap();

        // Insert test memories
        for i in 0..100 {
            let memory = Memory {
                id: format!("mem_{}", i),
                content: format!("Test memory {}", i),
                embedding: vec![0.1; 384],
                importance: 0.5,
                timestamp: chrono::Utc::now().timestamp(),
                metadata: HashMap::new(),
            };
            db.insert_memory(&memory).unwrap();
        }

        let mut idx = 0;
        b.iter(|| {
            let from = format!("mem_{}", idx);
            let to = format!("mem_{}", (idx + 1) % 100);
            idx = (idx + 1) % 100;
            db.link(&from, "related", &to).unwrap();
        });
    });

    group.bench_function("get_related", |b| {
        let temp_dir = TempDir::new().unwrap();
        let options = OpenDBOptions::with_dimension(384);
        let db = OpenDB::open_with_options(temp_dir.path(), options).unwrap();

        // Setup
        for i in 0..100 {
            let memory = Memory {
                id: format!("mem_{}", i),
                content: format!("Test memory {}", i),
                embedding: vec![0.1; 384],
                importance: 0.5,
                timestamp: chrono::Utc::now().timestamp(),
                metadata: HashMap::new(),
            };
            db.insert_memory(&memory).unwrap();

            if i > 0 {
                db.link(&format!("mem_0"), "related", &format!("mem_{}", i))
                    .unwrap();
            }
        }

        b.iter(|| {
            black_box(db.get_related("mem_0", "related").unwrap());
        });
    });

    group.finish();
}

fn transaction_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("transactions");

    group.bench_function("commit", |b| {
        let temp_dir = TempDir::new().unwrap();
        let options = OpenDBOptions::with_dimension(384);
        let db = OpenDB::open_with_options(temp_dir.path(), options).unwrap();

        b.iter(|| {
            let mut txn = db.begin_transaction().unwrap();
            txn.put("default", b"key1", b"value1").unwrap();
            txn.put("default", b"key2", b"value2").unwrap();
            txn.commit().unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    kv_benchmarks,
    memory_benchmarks,
    vector_search_benchmarks,
    graph_benchmarks,
    transaction_benchmarks
);
criterion_main!(benches);
