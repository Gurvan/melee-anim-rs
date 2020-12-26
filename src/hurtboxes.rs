use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum HurtboxType {
    Low,
    Mid,
    High,
}

#[derive(Debug)]
pub struct Hurtbox {
    bone_index: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    size: f32,
    r#type: HurtboxType,
    grabable: bool,
}

#[derive(Debug, Clone)]
pub struct ParseHurtboxError;

impl From<std::num::ParseIntError> for ParseHurtboxError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseHurtboxError{}
    }
}

impl From<std::num::ParseFloatError> for ParseHurtboxError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseHurtboxError{}
    }
}

impl From<std::str::ParseBoolError> for ParseHurtboxError {
    fn from(_: std::str::ParseBoolError) -> Self {
        ParseHurtboxError{}
    }
}

impl From<std::io::Error> for ParseHurtboxError {
    fn from(_: std::io::Error) -> Self {
        ParseHurtboxError{}
    }
}

impl Hurtbox {
    pub fn parse_type(s: &str) -> Result<HurtboxType, ParseHurtboxError> {
        match s {
            "Low" => Ok(HurtboxType::Low),
            "Mid" => Ok(HurtboxType::Mid),
            "High" => Ok(HurtboxType::High),
            _ => Err(ParseHurtboxError),
        }
    }
}

impl FromStr for Hurtbox {
    type Err = ParseHurtboxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hb_data = s.split(",").collect::<Vec<_>>();
        Ok(Hurtbox {
            bone_index: i32::from_str(hb_data[0])?,
            x1: f32::from_str(hb_data[1])?,
            y1: f32::from_str(hb_data[2])?,
            z1: f32::from_str(hb_data[3])?,
            x2: f32::from_str(hb_data[4])?,
            y2: f32::from_str(hb_data[5])?,
            z2: f32::from_str(hb_data[6])?,
            size: f32::from_str(hb_data[7])?,
            r#type: Hurtbox::parse_type(hb_data[8])?,
            grabable: i32::from_str(hb_data[9])? != 0,
        })
    }
}

pub fn parse_hurtboxes(path: &str) -> Result<Vec<Hurtbox>, ParseHurtboxError> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut hurtboxes: Vec<Hurtbox> = vec![];

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        match Hurtbox::from_str(&line) {
            Ok(hurtbox) => &hurtboxes.push(hurtbox),
            Err(e) => {
                println!("{:?}", e);
                &()
            }
        };
    }
    Ok(hurtboxes)
}
