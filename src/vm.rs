type Cell = u8;

pub struct VM {
    cells: [Cell; 30_000],
    data_pointer: usize,
}
