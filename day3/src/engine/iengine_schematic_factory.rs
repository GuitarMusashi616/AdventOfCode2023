use super::EngineSchematic;

pub trait IEngineSchematicFactory {
    fn create(&self, filename: &str) -> EngineSchematic;
}