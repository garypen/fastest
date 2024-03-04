use bytes::BytesMut;
use std::{cell::RefCell, fmt::Write};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn drop_bytes(c: &mut Criterion) {
    c.bench_function("drop bytes", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(4_096);
            let _ = buf.write_str("something that is very big but not really all that big but big enough to force some kind of allocation");
        });
    });
}

pub fn drop_string(c: &mut Criterion) {
    c.bench_function("drop string", |b| {
        b.iter(|| {
            let mut buf = String::with_capacity(4_096);
            let _ = buf.write_str("something that is very big but not really all that big but big enough to force some kind of allocation");
        });
    });
}

pub fn clear_string(c: &mut Criterion) {
    c.bench_function("clear string", |b| {
        b.iter(|| {
            thread_local! {
                static BUF: RefCell<String> = RefCell::new(String::with_capacity(4_096));
            }
            BUF.with(|buf| {
                let borrow = buf.try_borrow_mut();
                let mut a;
                let mut b;
                let buf = match borrow {
                    Ok(buf) => {
                        a = buf;
                        &mut *a
                    }
                    _ => {
                        b = String::new();
                        &mut b
                    }
                };
                let _ = buf.write_str("something that is very big but not really all that big but big enough to force some kind of allocation");
                buf.clear();
            });
        });
    });
}

criterion_group!(benches, clear_string, drop_string, drop_bytes);
criterion_main!(benches);
