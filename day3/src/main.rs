mod engine;
mod coord;
mod parser;
mod part_number;
use crate::{engine::{EngineSchematicFactoryFromFile, IEngineSchematicFactory}, parser::{IFindNumberCoords, FindNumberCoords, IFindSymbolCoords, FindSymbolCoords, IFindPartNumberCoords, FindPartNumberCoords}, part_number::{IPartNumberFilter, PartNumberFilter, ICoordsToNumber, CoordsToNumber}, coord::{ICoordAdj, CoordAdj}};

fn part1(filename: &str) {
    // let schem = EngineSchematic::new();
    let schem_factory: Box<dyn IEngineSchematicFactory> = Box::new(EngineSchematicFactoryFromFile::new());
    let schem = schem_factory.create(filename);

    // let find_numbers: Box<dyn IFindNumberCoords> = Box::new(FindNumberCoords::new());
    let find_symbols: Box<dyn IFindSymbolCoords> = Box::new(FindSymbolCoords::new());
    let find_parts: Box<dyn IFindPartNumberCoords> = Box::new(FindPartNumberCoords::new());

    // let number_coords = find_numbers.find(&schem);
    let symbol_coords = find_symbols.find(&schem);
    let part_number_coords = find_parts.find(&schem);

    let coord_adj: Box<dyn ICoordAdj> = Box::new(CoordAdj::new());
    let part_number_filter: Box<dyn IPartNumberFilter> = Box::new(PartNumberFilter::new(coord_adj));
    let coords_to_number: Box<dyn ICoordsToNumber> = Box::new(CoordsToNumber::new());


    let result = part_number_filter.filter(&part_number_coords, &symbol_coords);

    let mut numbers = Vec::new();
    for number_coords in result.iter() {
        let number = coords_to_number.process(&schem, number_coords);
        numbers.push(number);
    }


    // println!("{:?}\n", schem);
    // println!("{:?}\n", number_coords);
    // println!("{:?}\n", symbol_coords);
    // println!("{:?}\n", part_number_coords);
    // println!("{:?}\n", result);
    println!("{:?}\n", numbers);
    println!("{:?}\n", numbers.iter().fold(0, |x, y| x + y));


}

fn main() {
    part1("input.txt");
}