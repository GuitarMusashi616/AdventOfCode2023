#[derive(Debug)]
pub struct EngineSchematic {
    pub grid: Vec<Vec<char>>,
}

impl EngineSchematic {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        EngineSchematic {grid}
    }
}