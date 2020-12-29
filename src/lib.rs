#![allow(non_camel_case_types)]
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

pub mod bone;
pub mod hurtbox;
pub mod animation;

use bone::{Model, ParseSMDError};
use hurtbox::ParseHurtboxError;
use animation::{Animation, ParseFigaTreeError};

#[derive(Debug, Clone)]
pub struct LoadAnimationError;

impl From<std::io::Error> for LoadAnimationError {
    fn from(_: std::io::Error) -> Self {
        LoadAnimationError{}
    }
}

impl From<ParseSMDError> for LoadAnimationError {
    fn from(_: ParseSMDError) -> Self {
        LoadAnimationError{}
    }
}

impl From<ParseHurtboxError> for LoadAnimationError {
    fn from(_: ParseHurtboxError) -> Self {
        LoadAnimationError{}
    }
}

impl From<ParseFigaTreeError> for LoadAnimationError {
    fn from(_: ParseFigaTreeError) -> Self {
        LoadAnimationError{}
    }
}


pub trait Data {
    fn read(&self) -> Result<Vec<String>, std::io::Error>;
}

impl Data for &str {
    fn read(&self) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(self)?;
        let reader = BufReader::new(file);
        reader.lines().collect::<Result<Vec<_>, _>>()
    }
}

impl Data for &[u8] {
    fn read(&self) -> Result<Vec<String>, std::io::Error> {
        let reader = Cursor::new(self);
        reader.lines().collect::<Result<Vec<_>, _>>()
    }
}

pub fn get_animation_with_hurtboxes<T: Data>(model_data: T, hurtboxes_data: T, animation_data: T) -> Result<Animation, LoadAnimationError> {
    let model = Model::from_smd(&model_data.read()?)?;
    let hurtboxes = hurtbox::parse_hurtboxes(&hurtboxes_data.read()?)?;
    let mut anim = Animation::from_model(model.clone());
    anim.load_figatree(&animation_data.read()?)?;
    anim.attach_hurtboxes(hurtboxes);
    Ok(anim)
}