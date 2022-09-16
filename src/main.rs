#![warn(clippy::all, clippy::pedantic)]
mod rule_logic;
mod svg;

use crate::rule_logic::Automata;
use crate::svg::SvgModule;

use clap::Parser;
use svg::{render_automata_to_svg, create_svg_file};

#[derive(Parser)]
struct Cli {
    iter: String,
    breadth: String,
    rule_number: String,
    initial_position: String,
}

fn main() {
    let args = Cli::parse();
    let iter = args.iter.parse::<usize>().unwrap();
    let breadth = args.breadth.parse::<usize>().unwrap();
    let rule_number = args.rule_number.parse::<u8>().unwrap();
    let initial_position: &str = &args.initial_position[..]; 

    let auto: Automata = rule_logic::generate_ecm(
        iter, breadth, rule_number, initial_position);
    /*
        for r in auto.iter() {
        println!("{:?}", r);    
    }
    */

    let svg_module: SvgModule = render_automata_to_svg(auto);
    create_svg_file(svg_module)
}
