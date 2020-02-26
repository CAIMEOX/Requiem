extern crate image;
use super::table::block_ids::BlockStr;
use super::utils::str2i32;
use crate::mctype::config::Config;
use crate::mctype::geometry::{Block, Position};
use clap::ArgMatches;
use image::{GenericImageView, ImageError};
use nbt::{Blob, Error as NbtError, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs;
use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;

use std::option::NoneError;
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
#[derive(Debug)]
pub struct ParameterError {
    pub details: String,
}
impl ParameterError {
    fn new(reason: &str) -> Self {
        ParameterError {
            details: reason.to_owned(),
        }
    }
}

impl Error for ParameterError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&Error> {
        Some(self)
    }
}

impl Display for ParameterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.details)
    }
}

#[derive(Debug)]
pub enum BuilderError {
    Arg(ParameterError),
    ParseInt(ParseIntError),
    Nbt(NbtError),
    Img(ImageError),
    None(std::option::NoneError)
}

impl From<ParseIntError> for BuilderError {
    fn from(e: ParseIntError) -> Self {
        BuilderError::ParseInt(e)
    }
}

impl From<ImageError> for BuilderError {
    fn from(e: ImageError) -> Self {
        BuilderError::Img(e)
    }
}
impl From<ParameterError> for BuilderError {
    fn from(e: ParameterError) -> Self {
        BuilderError::Arg(e)
    }
}

impl From<NbtError> for BuilderError {
    fn from(e: NbtError) -> Self {
        BuilderError::Nbt(e)
    }
}

impl From<NoneError> for BuilderError {
    fn from(e: NoneError) -> Self {
        BuilderError::None(e)
    }
}
impl Display for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            BuilderError::None(ref e) => write!(f, "{:?}", e),
            BuilderError::Arg(ref e) => write!(f, "{}", e),
            BuilderError::ParseInt(ref e) => write!(f, "{}", e),
            BuilderError::Nbt(ref e) => write!(f, "{}", e),
            BuilderError::Img(ref e) => write!(f, "{}", e),
        }
    }
}

impl Error for BuilderError {
    fn description(&self) -> &str {
        match *self {
            BuilderError::None(ref e) => "None error",
            BuilderError::ParseInt(ref e) => e.description(),
            BuilderError::Arg(ref e) => e.description(),
            BuilderError::Nbt(ref e) => e.description(),
            BuilderError::Img(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BuilderError::None(ref e) => None,
            BuilderError::ParseInt(ref e) => Some(e),
            BuilderError::Arg(ref e) => Some(e),
            BuilderError::Nbt(ref e) => Some(e),
            BuilderError::Img(ref e) => Some(e),
        }
    }
}

type Builder<'a> =
    HashMap<&'static str, fn(ArgMatches, &Config<'a>, &ws::Sender) -> Vec<Block<'a>>>;
type BuilderFn<'a> = fn(ArgMatches, &'a Config, &ws::Sender) -> BuilderResult<'a>;
pub type BuilderResult<'a> = std::result::Result<Vec<Block<'a>>, BuilderError>;
pub fn generate<'a>(
    generator: String,
    args: ArgMatches,
    config: &'a Config,
    sender: &ws::Sender,
) -> BuilderResult<'a> {
    let GENERATOR = map! {
        "round" => round as BuilderFn,
        "circle" => circle as BuilderFn,
        "sphere" => sphere as BuilderFn,
        "ellipse" => ellipse as BuilderFn,
        "paint" => paint as BuilderFn,
        //"schematic" => schematic as BuilderFn,
        "drawmap" => draw_map as BuilderFn,
        "noise" => noise as BuilderFn,
        "pyramid" => pyramid as BuilderFn
    };
    round(args, config, sender)
}

//type BuilderResult<'a> = std::result::Result<Vec<Block<'a>>, BuilderError>;

//x ^ 2 + y ^ 2 < r ^ 2
fn round<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    let height = args.value_of("height")?;
    let height = height.parse::<i32>().unwrap_or(1);
    let radius = args.value_of("radius")?;
    let r = radius.parse::<i32>()?;
    match args.value_of("facing") {
        Some(facing) => {
            match facing {
                "x" => {
                    for h in 0..height {
                        for i in -r..=r {
                            for j in -r..=r {
                                if i * i + j * j < r * r {
                                    blocks.push(Block {
                                        position: Position { x: h, y: i, z: j },
                                        name: config.block.name,
                                        data: config.block.data,
                                    })
                                }
                            }
                        }
                    }
                }
                "y" => {
                    for h in 0..height {
                        for i in -r..=r {
                            for j in -r..=r {
                                if i * i + j * j < r * r {
                                    blocks.push(Block {
                                        position: Position { x: i, y: h, z: j },
                                        name: config.block.name,
                                        data: config.block.data,
                                    })
                                }
                            }
                        }
                    }
                }
                "z" => {
                    for h in 0..height {
                        for i in -r..=r {
                            for j in -r..=r {
                                if i * i + j * j < r * r {
                                    blocks.push(Block {
                                        position: Position { x: i, y: j, z: 0 },
                                        name: config.block.name,
                                        data: config.block.data,
                                    })
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(blocks)
        }
        None => Err(BuilderError::Arg(ParameterError::new("Invalid facing parameter"))),
    }
}

//x ^ 2 + y ^ 2 < r ^ 2 and x ^ 2 + y ^ 2 >= ir ^ 2
fn circle<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    let height = args.value_of("height")?;
    let height = height.parse::<i32>().unwrap_or(1);
    let radius = args.values_of("radius")?;
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
                                        position: Position { x: 0, y: i, z: j },
                                        name: config.block.name,
                                        data: config.block.data,
                                    })
                                }
                            }
                        }
                    }
                }
                "y" => {
                    for h in 0..height {
                        for i in -r..=r {
                            for j in -r..=r {
                                if i * i + j * j < r * r && i * i + j * j >= ir * ir {
                                    blocks.push(Block {
                                        position: Position { x: i, y: 0, z: j },
                                        name: config.block.name,
                                        data: config.block.data,
                                    })
                                }
                            }
                        }
                    }
                }
                "z" => {
                    for h in 0..height {
                        for i in -r..=r {
                            for j in -r..=r {
                                if i * i + j * j < r * r && i * i + j * j >= ir * ir {
                                    blocks.push(Block {
                                        position: Position { x: i, y: j, z: 0 },
                                        name: config.block.name,
                                        data: config.block.data,
                                    })
                                }
                            }
                        }
                    }
                }
                _ => {}
            };
            Ok(blocks)
        }
        None => Err(BuilderError::Arg(ParameterError::new("Invalid facing parameter"))),
    }
}

// x ^ 2 + y ^ 2 + z ^ 2 < r * r and x ^ 2 + y ^ 2 + z ^ 2 >= ir ^ 2
fn sphere<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    let radius = args.values_of("radius")?;
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
                    blocks.push(Block {
                        position: Position { x: i, y: j, z: k },
                        name: config.block.name,
                        data: config.block.data,
                    })
                }
            }
        }
    }
    Ok(blocks)
}

// (x ^ 2) / a ^ 2 + (y ^ 2) / b ^ 2 = 1
fn ellipse<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    Ok(blocks)
}

// (Sqrt( x ^ 2 + y ^ 2 ) - R) ^ 2 + z ^ 2 = r ^ 2
fn torus<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
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
                    if (r - ((I + J) as f32).sqrt()).powi(2) + (k * k) as f32 == (ir * ir) as f32 {}
                }
            }
        }
    }
    Ok(blocks)
}

//Bresenham's line algorithm
fn line(begin: Position, end: Position, name: &str, data: u8) -> Vec<Block> {
    let mut blocks: Vec<Block> = vec![];
    let Position { x: x1, y: _, z: z1 } = begin;
    let Position { x: x2, y: _, z: z2 } = end;
    let m = 2 * (z2 - z1);
    let mut sen = m - (x2 - x1);
    let mut z = z1;
    for x in x1..x2 + 1 {
        blocks.push(Block {
            position: Position { x, y: 0, z },
            name,
            data,
        });
        if sen >= 0 {
            z += 1;
            sen = sen - 2 * (x2 - x1);
        }
    }
    blocks
}

//Superpixel / CIELAB
fn paint<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    let path = args.value_of("path")?;
    let img = image::open(&Path::new(path))?;
    let height = img.height();
    let width = img.width();
    for h in 0..height {
        for w in 0..width {
            let c = img.get_pixel(w, h);
        }
    }
    Ok(blocks)
}

fn pyramid<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    Ok(blocks)
}

#[derive(Debug, PartialEq, Deserialize)]
struct Structure {
    #[serde(rename = "Blocks")]
    Blocks: Vec<String>,
    #[serde(rename = "Data")]
    Data: Vec<i32>,
    #[serde(rename = "Width")]
    Width: u32,
    #[serde(rename = "Height")]
    Height: u32,
    #[serde(rename = "Length")]
    Length: u32,
}

/*
fn schematic<'a>(
    args: ArgMatches,
    config: &'a Config,
    sender: &ws::Sender,
) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    match args.value_of("path") {
        Some(path) => {
            let file = fs::File::open(path);
            if let Ok(mut f) = file {
                let blob: Result<Structure> = Blob::from_gzip_reader(&mut f);
                match blob {
                    Ok(b) => {
                        let structure = &b["Blocks"];
                        let data = &b["Data"];
                        let (w, l, h) = (
                            b["Width"].to_string().parse::<u32>().unwrap(),
                            b["Length"].to_string().parse::<u32>().unwrap(),
                            b["Height"].to_string().parse::<u32>().unwrap(),
                        );
                        let mut index = 0;
                        for y in 0..h {
                            for z in 0..l {
                                for x in 0..w {
                                    blocks.push(Block {
                                        position: Position {
                                            x: x as i32,
                                            y: y as i32,
                                            z: z as i32,
                                        },

                                        name: BlockStr[structure.id() as u8],
                                        data: data[index],
                                    });
                                    index += 1;
                                }
                            }
                        }
                        Ok(blocks)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(file)
            }
            Ok(blocks)
        }
        None => Err("Args path not found.".to_string()),
    }
}*/

fn noise<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    Ok(blocks)
}
fn draw_map<'a>(args: ArgMatches, config: &'a Config, sender: &ws::Sender) -> BuilderResult<'a> {
    let mut blocks: Vec<Block> = vec![];
    Ok(blocks)
}
