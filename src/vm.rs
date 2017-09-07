type Cell = u8;

pub struct VM {
    cells: [Cell; 30_000],
    data_pointer: usize,
}


impl VM {
    pub fn new() -> VM {
        VM { cells: [0; 30_000], data_pointer: 0 }
    }


    pub fn output(&self) -> Cell {
        self.cells[self.data_pointer]
    }


    pub fn increment(&mut self) {
        let value = self.cells[self.data_pointer];
        self.cells[self.data_pointer] = value + 1;
    }
}