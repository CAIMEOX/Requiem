extern crate image;
use clap::ArgMatches;
use std::collections::HashMap;
use crate::mctype::geometry::{Block, Position};
use crate::mctype::config::Config;
use std::path::Path;
use image::GenericImageView;
use super::utils::{str2i32};

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
type Builder<'a> = HashMap<&'static str, fn(ArgMatches, &Config<'a>, &ws::Sender) -> Vec<Block<'a>>>;
type BuilderFn<'a> = fn(ArgMatches, &'a Config, &ws::Sender) -> Vec<Block<'a>>;

pub fn generate<'a>(generator: String, args: ArgMatches, config: &'a Config, sender:&ws::Sender) -> Vec<Block<'a>>{
    let GENERATOR = map!{
        "round" => round as BuilderFn,
        "circle" => circle as BuilderFn,
        "sphere" => sphere as BuilderFn,
        "ellipse" => ellipse as BuilderFn,
        "paint" => paint as BuilderFn,
        "schematic" | "sc" => schematic as BuilderFn,
        "drawmap" | "dm" => draw_map as BuilderFn,
        "noise" => noise as BuilderFn,
        "pyramid" => pyramid as BuilderFn,
    };
    round(args, config, sender)
}


//x ^ 2 + y ^ 2 < r ^ 2
fn round<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{
    let mut blocks: Vec<Block> = vec![];
    let height = if let Some(height) = args.value_of("height") {
        if let Ok(h) = height.parse::<i32>(){
            h
        } else { 0 }
    } else {
        0
    };
    if let Some(radius) = args.value_of("radius") {
        if let Ok(r) = radius.parse::<i32>() {
            match args.value_of("facing") {
                Some(facing) => {
                    match facing {
                        "x" => {
                            for h in 0..height {
                                for i in -r..=r {
                                    for j in -r ..=r {
                                        if i * i + j * j < r * r {
                                            blocks.push(Block{
                                                position: Position {
                                                    x: h,
                                                    y: i,
                                                    z: j
                                                },
                                                name: config.block.name,
                                                data: config.block.data
                                            })
                                        }
                                    }
                                }
                            }

                        },
                        "y" => {
                            for h in 0..height {
                                for i in -r..=r {
                                    for j in -r ..=r {
                                        if i * i + j * j < r * r {
                                            blocks.push(Block{
                                                position: Position {
                                                    x: i,
                                                    y: h,
                                                    z: j
                                                },
                                                name: config.block.name,
                                                data: config.block.data
                                            })
                                        }
                                    }
                                }
                            }

                        },
                        "z" => {
                            for h in 0..height {
                                for i in -r..=r {
                                    for j in -r ..=r {
                                        if i * i + j * j < r * r {
                                            blocks.push(Block{
                                                position: Position {
                                                    x: i,
                                                    y: j,
                                                    z: 0
                                                },
                                                name: config.block.name,
                                                data: config.block.data
                                            })
                                        }
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    blocks
}

//x ^ 2 + y ^ 2 < r ^ 2 and x ^ 2 + y ^ 2 >= ir ^ 2
fn circle<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>> {
    let mut blocks: Vec<Block> = vec![];
    let mut blocks: Vec<Block> = vec![];
    let height = if let Some(height) = args.value_of("height") {
        if let Ok(h) = height.parse::<i32>() {
            h
        } else { 0 }
    } else {
        0
    };
    if let Some(radius) = args.values_of("radius") {
        let radius = str2i32(radius.collect::<Vec<_>>());
        let mut r = 0;
        let mut ir = 0;
        if radius.len() == 1 {
            r = radius[0];
            ir = radius[0] - 1;
        } else {
            match radius[1] - radius[0] {
                a if a > 0 => {
                    r = radius[0];
                    ir = radius[1]
                }
                b if b < 0 => {
                    r = radius[1];
                    ir = radius[0]
                }
                _ => {
                    r = radius[0];
                    ir = radius[0] - 1;
                }
            }
        }
        match args.value_of("facing") {
            Some(facing) => {
                match facing {
                    "x" => {
                        for h in 0..height {
                            for i in -r..=r {
                                for j in -r..=r {
                                    if i * i + j * j < r * r && i * i + j * j >= ir * ir {
                                        blocks.push(Block {
                                            position: Position {
                                                x: 0,
                                                y: i,
                                                z: j
                                            },
                                            name: config.block.name,
                                            data: config.block.data
                                        })
                                    }
                                }
                            }
                        }
                    },
                    "y" => {
                        for h in 0..height {
                            for i in -r..=r {
                                for j in -r..=r {
                                    if i * i + j * j < r * r && i * i + j * j >= ir * ir {
                                        blocks.push(Block {
                                            position: Position {
                                                x: i,
                                                y: 0,
                                                z: j
                                            },
                                            name: config.block.name,
                                            data: config.block.data
                                        })
                                    }
                                }
                            }
                        }
                    },
                    "z" => {
                        for h in 0..height {
                            for i in -r..=r {
                                for j in -r..=r {
                                    if i * i + j * j < r * r && i * i + j * j >= ir * ir {
                                        blocks.push(Block {
                                            position: Position {
                                                x: i,
                                                y: j,
                                                z: 0
                                            },
                                            name: config.block.name,
                                            data: config.block.data
                                        })
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        };

    }
    blocks
}

// x ^ 2 + y ^ 2 + z ^ 2 < r * r and x ^ 2 + y ^ 2 + z ^ 2 >= ir ^ 2
fn sphere<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{
    let mut blocks: Vec<Block> = vec![];
    if let Some(radius) = args.values_of("radius") {
        let radius = str2i32(radius.collect::<Vec<_>>());
        let mut r = 0;
        let mut ir = 0;
        if radius.len() == 1 {
            r = radius[0];
            ir = radius[0] - 1;
        } else {
            match radius[1] - radius[0] {
                a if a > 0 => {
                    r = radius[0];
                    ir = radius[1]
                }
                b if b < 0 => {
                    r = radius[1];
                    ir = radius[0]
                }
                _ => {
                    r = radius[0];
                    ir = radius[0] - 1;
                }
            }
        }
        for i in -r..r {
            for j in -r..r {
                for k in -r..r {
                    if i * i + j * j + k * k < r * r && i * i + j * j + k * k >= ir * ir {
                        blocks.push(Block{
                            position: Position {
                                x: i,
                                y: j,
                                z: k
                            },
                            name: config.block.name,
                            data: config.block.data
                        })
                    }
                }
            }
        }
    }
    blocks
}

// (x ^ 2) / a ^ 2 + (y ^ 2) / b ^ 2 = 1
fn ellipse<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{
    let mut blocks: Vec<Block> = vec![];
    blocks
}

// (Sqrt( x ^ 2 + y ^ 2 ) - R) ^ 2 + z ^ 2 = r ^ 2
fn torus<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{
    let mut blocks: Vec<Block> = vec![];
    if let Some(radius) = args.values_of("radius") {
        let radius = str2i32(radius.collect::<Vec<_>>());
        let mut r = 0;
        let mut ir = 0;
        if radius.len() == 1 {
            r = radius[0];
            ir = radius[0] - 1;
        } else {
            match radius[1] - radius[0] {
                a if a > 0 => {
                    r = radius[0];
                    ir = radius[1]
                }
                b if b < 0 => {
                    r = radius[1];
                    ir = radius[0]
                }
                _ => {
                    r = radius[0];
                    ir = radius[0] - 1;
                }
            }
        }

        let R = r + ir;
        let (r, ir) = (r as f32, r as f64);
        for i in -R..R {
            let I = i * i;
            for j in -R..R {
                let J = j * j;
                for k in -R..R {
                    if (r - ((I + J) as f32).sqrt() ).powi(2) + (k * k) as f32 == (ir * ir) as f32 {

                    }
                }
            }
        }
    }
    blocks
}

//Bresenham's line algorithm
fn line(begin: Position, end: Position, name: &str, data: u8) -> Vec<Block> {
    let mut blocks: Vec<Block> = vec![];
    let Position{x:x1,y:_,z:z1}= begin;
    let Position{x:x2,y:_,z:z2} = end;
    let m = 2 * (z2 - z1);
    let mut sen = m - (x2 - x1);
    let mut z = z1;
    for x in x1..x2 + 1 {
        blocks.push(Block {
            position: Position {
                x,
                y: 0,
                z
            },
            name,
            data
        });
        if sen >= 0 {
            z += 1;
            sen = sen - 2 * (x2 - x1);
        }
    }
    blocks
}

//Superpixel / CIELAB
fn paint<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{
    let mut blocks: Vec<Block> = vec![];
    if let Some(path) = args.value_of("path") {
        let img = image::open(&Path::new(path));
        if let Ok(img) = img {
            let height = img.height();
            let width = img.width();
            for h in 0..height {
                for w in 0..width {
                    let c = img.get_pixel(w, h);

                }
            }
        }
    }
    blocks
}

fn schematic<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{}
fn noise<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{}
fn draw_map<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> Vec<Block<'a>>{}
