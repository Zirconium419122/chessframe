use crate::square::Square;

pub fn diagonal(start: Square, end: Square) -> bool {
    let start_rank = start.get_rank().to_index() as i8;
    let start_file = start.get_file().to_index() as i8;
    let end_rank = end.get_rank().to_index() as i8;
    let end_file = end.get_file().to_index() as i8;

    (start_rank - end_rank).abs() == (start_file - end_file).abs()
}

pub fn orthagonal(start: Square, end: Square) -> bool {
    let start_rank = start.get_rank().to_index() as i8;
    let start_file = start.get_file().to_index() as i8;
    let end_rank = end.get_rank().to_index() as i8;
    let end_file = end.get_file().to_index() as i8;

    start_rank == end_rank || start_file == end_file
}

pub fn between(start: Square, end: Square, test: Square) -> bool {
    let start = start.to_int();
    let end = end.to_int();
    let test = test.to_int();

    (start < end && start < test && test < end) || (start >= end && end < test && test < start)
}
