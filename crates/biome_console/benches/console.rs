use std::io;

use biome_console::fmt::{MarkupElements, Termcolor, Write};
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use termcolor::{Ansi, NoColor};

#[derive(Default)]
struct BenchWriter {
    bytes_written: usize,
}

impl io::Write for BenchWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let buffer = black_box(buffer);
        self.bytes_written = self.bytes_written.wrapping_add(buffer.len());
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn bench_console(criterion: &mut Criterion) {
    let ascii = "  123 | const answer = 42;\n".repeat(128);
    let unicode = "  123 │ const answer = 42;\n    ━━━━━━━━━━━━━━━━━━━━━━━\n".repeat(64);
    let sanitized = "source\0code\u{200B}diagnostic\n".repeat(128);
    let mut group = criterion.benchmark_group("console");

    group.throughput(Throughput::Bytes(ascii.len() as u64));
    group.bench_function("ascii", |bencher| {
        let mut writer = Termcolor(Ansi::new(BenchWriter::default()));
        bencher.iter(|| {
            Write::write_str(
                &mut writer,
                &MarkupElements::Root,
                black_box(ascii.as_str()),
            )
            .unwrap();
            black_box(&writer);
        });
    });

    group.throughput(Throughput::Bytes(unicode.len() as u64));
    group.bench_function("unicode_without_colors", |bencher| {
        let mut writer = Termcolor(NoColor::new(BenchWriter::default()));
        bencher.iter(|| {
            Write::write_str(
                &mut writer,
                &MarkupElements::Root,
                black_box(unicode.as_str()),
            )
            .unwrap();
            black_box(&writer);
        });
    });

    group.throughput(Throughput::Bytes(sanitized.len() as u64));
    group.bench_function("sanitized", |bencher| {
        let mut writer = Termcolor(Ansi::new(BenchWriter::default()));
        bencher.iter(|| {
            Write::write_str(
                &mut writer,
                &MarkupElements::Root,
                black_box(sanitized.as_str()),
            )
            .unwrap();
            black_box(&writer);
        });
    });

    group.finish();
}

criterion_group!(console, bench_console);
criterion_main!(console);
