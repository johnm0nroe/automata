use crate::rule_logic::Automata;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub type SvgModule = String;
type SvgComponent = String;

const XMLSN: &str = "http://www.w3.org/2000/svg";
const XLINK: &str = "http://www.w3.org/1999/xlink";
const BLACK: &str = "#000";
const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn svg(inner_content: SvgComponent) -> SvgModule {
   let mut svg_header: String = format!(
    "<svg xmlns=\"{}\" xmlns:xlink=\"{}\" viewBox=\"0 0 {} {}\">", 
    XMLSN,
    XLINK,
    WIDTH.to_string(),
    HEIGHT.to_string()
    ).to_owned();

   svg_header.push_str(&inner_content[..]);
   svg_header.push_str(&"</svg>".to_string());
   svg_header
}

fn rect(
    marked:bool, 
    width:u32, 
    height:u32, 
    x: u32, 
    y:u32
) -> SvgComponent {
    let fill: &str = 
        if marked {
            BLACK
        } else {
            "none"
        };

    let rect: String = format!(
        "<rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" stroke=\"{}\" fill=\"{}\"/>",
        width.to_string(),
        height.to_string(),
        x.to_string(),
        y.to_string(),
        BLACK,
        fill
    );

    rect

}

pub fn render_automata_to_svg(auto: Automata) -> SvgModule {
    let mut inner_content: SvgComponent = String::new();
    let width: usize = WIDTH / auto[0].len();
    let height: usize = HEIGHT / auto.len();
    println!("width:{}, height:{}", width, height);
    for (r, v) in auto.into_iter().enumerate() {
        for (e, n)  in v.into_iter().enumerate() {
            let marked: bool = if n==1 {true} else {false};
            let x: usize = width * e;
            let y: usize = height * r;
            let new_rect = rect(
                marked,
                width as u32,
                height as u32,
                x as u32,
                y as u32
            );
            inner_content.push_str(&new_rect[..]);
        }
    }
    svg(inner_content)
}

pub fn create_svg_file(svg:SvgModule) {
    let path = Path::new("automata.svg");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(svg.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}