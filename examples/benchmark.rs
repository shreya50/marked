//! A lightweight throughput benchmark for the parser.
//!
//! Run with: `cargo run --release --example benchmark -- 30 1 benchmarks/corpus/*.md`

use std::{env, fs, hint::black_box, time::Instant};

use marked_rs::{parse_with_options, Options};

const SAMPLE: &str = r#"# Benchmark document

This is a **small** paragraph with *emphasis*, [a link](https://example.com),
an ![image](image.png), and `inline code`.

> A quoted line for block parsing.

- first item
- second item
- third item

1. one
2. two

```rust
fn main() {
    println!("<escaped>");
}
```

---
"#;

fn main() {
    let mut arguments = env::args().skip(1);
    let iterations = arguments.next().map(|value| value.parse().expect("iterations must be a positive integer")).unwrap_or(20usize);
    let repetitions = arguments.next().map(|value| value.parse().expect("repetitions must be a positive integer")).unwrap_or(5_000usize);
    assert!(iterations > 0 && repetitions > 0, "iterations and repetitions must be greater than zero");
    let files: Vec<_> = arguments.collect();

    let corpus = if files.is_empty() {
        SAMPLE.to_owned()
    } else {
        files.iter().map(|path| fs::read_to_string(path).unwrap_or_else(|error| panic!("cannot read {path}: {error}"))).collect::<Vec<_>>().join("\n\n")
    };
    // Repetition makes parsing work dominate process startup and timing noise.
    let source = corpus.repeat(repetitions);
    for _ in 0..3 { black_box(parse_with_options(black_box(&source), Options { gfm: true })); }
    let started = Instant::now();
    for _ in 0..iterations {
        black_box(parse_with_options(black_box(&source), Options { gfm: true }));
    }
    let elapsed = started.elapsed();
    let total_mib = source.len() as f64 * iterations as f64 / (1024.0 * 1024.0);

    println!("parser: marked-rs (GFM enabled)");
    println!("input: {} bytes", source.len());
    println!("iterations: {iterations}");
    println!("mean: {:.2} ms", elapsed.as_secs_f64() * 1_000.0 / iterations as f64);
    println!("throughput: {:.2} MiB/s", total_mib / elapsed.as_secs_f64());
}
