use criterion::{criterion_group, criterion_main, Criterion};
use prost::Message;
use rs_protobuf_demo::student::Student;

fn decode_slice(buf: &[u8]) -> anyhow::Result<()> {
    Student::decode(&buf[..])?;
    Ok(())
}

fn decode_buf(buf: &[u8]) -> anyhow::Result<()> {
    Student::decode(buf)?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let buf = [0u8; 1024];
    // let buf = 0x0801120568656c6c6f;
    c.bench_function("decode_slice", |b| b.iter(|| decode_slice(&buf)));
    c.bench_function("decode_buf", |b| b.iter(|| decode_buf(&buf)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);