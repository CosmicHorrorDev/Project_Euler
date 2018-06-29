## Project Euler

These are my solutions to the [Project Euler](projecteuler.net) challenges. I am much more familiar with Python than I am with Rust, so the Rust solutions probably aren't the best.

### Benchmarks

All benchmarks are done using hyperfine, the Rust solutions so far are too fast for the benchmark to be accurate, but it gives a general idea. These benchmarks are done on my personal computer so results will vary.

| Problem | Language | Mean [ms] | Min...Max [ms] |
|:---|:---|---:|---:|
| Problem 1 | Python | 18.8 ± 1.1 | 17.6…23.1 |
| Problem 1 | Rust | 1.7 ± 1.0 | 0.1…4.8 |
| Problem 2 | Python | 18.9 ± 1.3 | 17.5…23.8 |
| Problem 2 | Rust | 1.6 ± 1.0 | 0.1…4.8 |
| Problem 3 | Python | 19.3 ± 1.4 | 18.1…27.4 |
| Problem 3 | Rust | 1.7 ± 1.0 | 0.2…4.6 |
