use std::{hint::black_box, time::Instant};

use chessframe::{bitboard::EMPTY, board::Board, chess_move::ChessMove};

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    const ITERS: usize = 1_000_000;

    let mut moves = [ChessMove::NULL_MOVE; 48];

    let start = Instant::now();

    let mut total_nodes = 0usize;
    let mut total_moves = 0usize;

    for _ in 0..ITERS {
        let n = board.generate_moves(!EMPTY, &mut moves);

        total_nodes += 1;
        total_moves += n;

        black_box(&moves[..n]);
    }

    let elapsed = start.elapsed();

    black_box(total_nodes);
    black_box(total_moves);

    let secs = elapsed.as_secs_f64();

    let nodes_per_sec = total_nodes as f64 / secs;
    let moves_per_sec = total_moves as f64 / secs;

    let ns_per_node = elapsed.as_nanos() as f64 / total_nodes as f64;
    let _avg_moves = total_moves as f64 / total_nodes as f64;

    println!("╔════════════════════════════╗");
    println!("║        MOVEGEN BENCH       ║");
    println!("╠════════════════════════════╣");
    println!("║ time        : {:>9.3} ms ║", elapsed.as_micros() as f64 / 1000.0);
    println!("║ nodes       : {:>12} ║", total_nodes);
    println!("║ moves       : {:>12} ║", total_moves);
    println!("╠════════════════════════════╣");
    println!("║ nodes/sec   : {:>7.2} MN/s ║", nodes_per_sec / 1e6);
    println!("║ moves/sec   : {:>7.2} MN/s ║", moves_per_sec / 1e6);
    println!("║ ns/node     : {:>9.2} ns ║", ns_per_node);
    println!("╚════════════════════════════╝");
}
