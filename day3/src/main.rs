use parser::FindGearSymbolCoords;
use part_number::{IGearCounter, GearCounter};

use crate::{engine::{IEngineSchematicFactory, EngineSchematicFactoryFromFile}, coord::{ICoordAdj, CoordAdj}, part_number::{IPartNumberFilter, ICoordsToNumber, PartNumberFilter, CoordsToNumber}, parser::{IFindSymbolCoords, IFindPartNumberCoords, FindPartNumberCoords, FindSymbolCoords}};

mod engine;
mod coord;
mod parser;
mod part_number;

fn part1(filename: &str) {
    let schem_factory: Box<dyn IEngineSchematicFactory> = Box::new(EngineSchematicFactoryFromFile::new());
    let schem = schem_factory.create(filename);

    let find_symbols: Box<dyn IFindSymbolCoords> = Box::new(FindSymbolCoords::new());
    let find_parts: Box<dyn IFindPartNumberCoords> = Box::new(FindPartNumberCoords::new());

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

fn part2(filename: &str) {
    let schem_factory: Box<dyn IEngineSchematicFactory> = Box::new(EngineSchematicFactoryFromFile::new());
    let schem = schem_factory.create(filename);

    let coord_adj: Box<dyn ICoordAdj> = Box::new(CoordAdj::new());
    let coords_to_number: Box<dyn ICoordsToNumber> = Box::new(CoordsToNumber::new());

    let find_parts: Box<dyn IFindPartNumberCoords> = Box::new(FindPartNumberCoords::new());
    let part_number_coords = find_parts.find(&schem);

    let find_gears: Box<dyn IFindSymbolCoords> = Box::new(FindGearSymbolCoords::new());
    let gear_counter: Box<dyn IGearCounter> = Box::new(GearCounter::new(coord_adj, coords_to_number));

    let gear_coords = find_gears.find(&schem);
    let count = gear_counter.count(&schem, &part_number_coords, &gear_coords);

    println!("{:?}\n", gear_coords);
    println!("{:?}\n", count);
}

fn main() {
    let filename = "input.txt";
    part1(filename);
    part2(filename);
}