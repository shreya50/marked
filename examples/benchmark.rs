//! A lightweight throughput benchmark for the parser.
//!
//! Run with: `cargo run --release --example benchmark -- 20`

use std::{env, hint::black_box, time::Instant};

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
    let iterations = env::args()
        .nth(1)
        .map(|value| value.parse().expect("iterations must be a positive integer"))
        .unwrap_or(20);
    assert!(iterations > 0, "iterations must be greater than zero");

    // Roughly 2.1 MiB: large enough for parsing work to dominate timing noise.
    let source = SAMPLE.repeat(5_000);
    let started = Instant::now();
    for _ in 0..iterations {
        black_box(marked_rs::parse(black_box(&source)));
    }
    let elapsed = started.elapsed();
    let total_mib = source.len() as f64 * iterations as f64 / (1024.0 * 1024.0);

    println!("parser: marked-rs");
    println!("input: {} bytes", source.len());
    println!("iterations: {iterations}");
    println!("mean: {:.2} ms", elapsed.as_secs_f64() * 1_000.0 / iterations as f64);
    println!("throughput: {:.2} MiB/s", total_mib / elapsed.as_secs_f64());
}
