use std::fs;

use super::{IEngineSchematicFactory, EngineSchematic};

pub struct EngineSchematicFactoryFromFile; 

impl EngineSchematicFactoryFromFile {
    pub fn new() -> Self {
        EngineSchematicFactoryFromFile {}
    }
}

impl IEngineSchematicFactory for EngineSchematicFactoryFromFile {
    fn create(&self, filename: &str) -> EngineSchematic {
        let content = fs::read_to_string(filename).expect("file not found");
        
        let val: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        return EngineSchematic::new(val);
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::{IEngineSchematicFactory, EngineSchematicFactoryFromFile};

    #[test]
    fn correct_schem_from_file() {
        // Arrange
        let factory: Box<dyn IEngineSchematicFactory> = Box::new(EngineSchematicFactoryFromFile::new());
        let filename = r"C:\Users\awilliams\Code\rust\AdventOfCode2023\day3\src\input.txt";

        // // Act
        let schem = factory.create(filename);

        // // Assert
        println!("{:?}", schem);
    }
}