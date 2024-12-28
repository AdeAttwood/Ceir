use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use common::Board;
use engine::search::Search;
use engine::transposition_table::TranspositionTable;
use engine::uci;

struct NullWriter {}

impl uci::UciWriter for NullWriter {
    fn writeln(&mut self, _output: &str) {}
}

fn bench_position(fen: &str) {
    let board = black_box(Board::from_fen_str(fen).unwrap());
    let mut transposition_table = black_box(TranspositionTable::new());
    let mut writer = black_box(NullWriter {});

    let mut search = Search::new(&mut writer, &mut transposition_table, board, 4);

    search.search();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("random fen 1", |b| {
        b.iter(|| {
            bench_position(black_box(
                "r1b1qrk1/pp2bpp1/3pnn1p/2p1p3/2P5/1QNPP1P1/PP1N1PBP/R1B2RK1 b - - 6 12",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
