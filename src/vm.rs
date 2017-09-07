type Cell = u8;

pub struct VM {
    cells: [Cell; 30_000],
    data_pointer: usize,
}


impl VM {
    pub fn new() -> VM {
        VM { cells: [0; 30_000], data_pointer: 0 }
    }
}
