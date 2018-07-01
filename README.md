## Project Euler

These are my solutions to the [Project Euler](projecteuler.net) challenges. I am much more familiar with Python than I am with Rust, so the Rust solutions probably aren't the best.

### Benchmarks

The benchmarks are done using the different Problem Runners in the Problem\_Runners directory. Benchmarking consists of timing the problems for 5000 rounds, then determining the mean and standard devition. This is done in the program itself to try and minimize the overhead and give a good representation.

| Problem | Language | Mean ± σ [µs] |
|:---|:---|---:|
| Problem 1 | Python | 1.98 ± 0.60 |
| | Rust | 0.38 ± 0.46 |
| Problem 2 | Python | 5.76 ± 1.00 |
| | Rust | 0.28 ± 0.05 |
| Problem 3 | Python |  555.22 ± 8.52|
| | Rust | 32.46 ± 1.90 |
| Problem 4 | Python |  |
| | Rust |  |
