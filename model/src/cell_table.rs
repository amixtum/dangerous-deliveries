use super::cell::Cell;

pub struct CellTable {
    width: usize,
    height: usize,
    table: Vec<Vec<Cell>>
}
