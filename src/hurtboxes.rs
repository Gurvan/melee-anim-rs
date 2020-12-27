use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra::{Unit, Vector3};
use nalgebra::geometry::{UnitQuaternion, Isometry3, Translation3};

#[derive(Debug)]
pub enum HurtboxType {
    Low,
    Mid,
    High,
}

#[derive(Debug)]
pub struct Hurtbox {
    pub bone_index: i32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub x2: f32,
    pub y2: f32,
    pub z2: f32,
    pub size: f32,
    pub r#type: HurtboxType,
    pub grabable: bool,
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
    pub fn norm(&self) -> f32 {
        ((self.x1 - self.x2).powi(2) + (self.y1 - self.y2).powi(2) + (self.z1 - self.z2).powi(2)).sqrt()
    }

    pub fn p1(&self) -> Translation3<f32> {
        Translation3::new(self.x1, self.y1, self.z1)
    }

    pub fn p2(&self) -> Translation3<f32> {
        Translation3::new(self.x2, self.y2, self.z2)
    }

    pub fn center(&self) -> Translation3<f32> {
        Translation3::new(0.5 * (self.x1 + self.x2), 0.5 * (self.y1 + self.y2), 0.5 * (self.z1 + self.z2))
    }

    pub fn rotation(&self) -> UnitQuaternion<f32> {
        if self.norm() > 0. {
            let dir = Vector3::new(self.x2 - self.x1, self.y2 - self.y1, self.z2 - self.z1);
            if dir.x == 0. && dir.z == 0. {
                return UnitQuaternion::identity();
            } else {
                let axis: Unit<Vector3<f32>> = Unit::new_normalize(Vector3::y_axis().cross(&dir));
                let angle = Vector3::y_axis().dot(&dir).acos();
                return UnitQuaternion::from_axis_angle(&axis, angle);
            }
        }
        UnitQuaternion::identity()
    }

    pub fn transform(&self) -> Isometry3<f32> {
        Isometry3::from_parts(self.center(), self.rotation())
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
