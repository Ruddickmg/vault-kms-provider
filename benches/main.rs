use criterion::{criterion_group, criterion_main};
use decryption::decryption_benchmark;
use encryption::encryption_benchmark;
use health::health_check_benchmark;

mod decryption;
mod encryption;
mod health;

criterion_group!(
    benches,
    decryption_benchmark,
    encryption_benchmark,
    health_check_benchmark
);
criterion_main!(benches);
