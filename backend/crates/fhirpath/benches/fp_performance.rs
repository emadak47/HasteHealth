use criterion::{Criterion, criterion_group, criterion_main};
use haste_fhir_model::r4::generated::{
    resources::Patient,
    types::{FHIRString, HumanName},
};
use haste_fhirpath::FPEngine;
use tokio::runtime::Runtime;

fn fp_performance_simple(c: &mut Criterion) {
    let root = Patient {
        name: Some(vec![Box::new(HumanName {
            given: Some(vec![Box::new(FHIRString {
                value: Some("John".to_string()),
                ..Default::default()
            })]),
            ..Default::default()
        })]),
        ..Default::default()
    };
    let engine = FPEngine::new();
    c.bench_function("fp_performance_simple", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| engine.evaluate("Patient.name.given", vec![&root]))
    });
}

fn parser_test_performance(c: &mut Criterion) {
    let engine = FPEngine::new();
    c.bench_function("parser_test_performance", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| engine.evaluate("1 + 2 * (3 - 4) / 5", vec![]))
    });
}

fn parser_test_complex(c: &mut Criterion) {
    let engine = FPEngine::new();
    c.bench_function("parser_test_complex",
    |b|  b.to_async(Runtime::new().unwrap()).iter(|| engine.evaluate("$this.field + %test._asdf.test(45, $this.field) * 64 * $this.where($this.field = '23'.length())", vec![])));
}

fn parser_test_simple(c: &mut Criterion) {
    let engine = FPEngine::new();
    c.bench_function("parser_test_simple", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| engine.evaluate("$this.field", vec![]))
    });
}

criterion_group!(
    benches,
    fp_performance_simple,
    parser_test_performance,
    parser_test_complex,
    parser_test_simple
);
criterion_main!(benches);
