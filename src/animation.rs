use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

use crate::bone::{Model, ParseSMDError};
use crate::hurtbox::Hurtbox;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InterpolationType {
    HSD_A_OP_NONE,
    HSD_A_OP_CON,
    HSD_A_OP_LIN,
    HSD_A_OP_SPL0,
    HSD_A_OP_SPL,
    HSD_A_OP_SLP,
    HSD_A_OP_KEY,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrackType {
    HSD_A_J_NONE,
    HSD_A_J_ROTX,
    HSD_A_J_ROTY,
    HSD_A_J_ROTZ,
    HSD_A_J_PATH,
    HSD_A_J_TRAX,
    HSD_A_J_TRAY,
    HSD_A_J_TRAZ,
    HSD_A_J_SCAX,
    HSD_A_J_SCAY,
    HSD_A_J_SCAZ,
    HSD_A_J_NODE,
}

pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Key {
    frame: f32,
    value: f32,
    tan: f32,
    interpolation_type: InterpolationType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnimState {
    p0: f32,
    p1: f32,
    d0: f32,
    d1: f32,
    t0: f32,
    t1: f32,
    op_intrp: InterpolationType,
    op: InterpolationType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Track {
    pub r#type: TrackType,
    pub keys: Vec<Key>,
}

pub struct Animation {
    pub frame_count: f32,
    pub model: Model,
    pub hurtboxes: Vec<Hurtbox>,
}

#[derive(Debug, Clone)]
pub struct ParseKeyError;

impl From<std::num::ParseIntError> for ParseKeyError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseKeyError{}
    }
}

impl From<std::num::ParseFloatError> for ParseKeyError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseKeyError{}
    }
}

impl From<std::io::Error> for ParseKeyError {
    fn from(_: std::io::Error) -> Self {
        ParseKeyError{}
    }
}

#[derive(Debug, Clone)]
pub struct ParseFigaTreeError;

impl From<std::num::ParseIntError> for ParseFigaTreeError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseFigaTreeError{}
    }
}

impl From<std::num::ParseFloatError> for ParseFigaTreeError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseFigaTreeError{}
    }
}

impl From<std::io::Error> for ParseFigaTreeError {
    fn from(_: std::io::Error) -> Self {
        ParseFigaTreeError{}
    }
}

impl From<ParseKeyError> for ParseFigaTreeError {
    fn from(_: ParseKeyError) -> Self {
        ParseFigaTreeError{}
    }
}

impl Key {
    pub fn parse_interpolation_type(s: &str) -> Result<InterpolationType, ParseKeyError> {
        match s {
            "HSD_A_OP_NONE" => Ok(InterpolationType::HSD_A_OP_NONE),
            "HSD_A_OP_CON" => Ok(InterpolationType::HSD_A_OP_CON),
            "HSD_A_OP_LIN" => Ok(InterpolationType::HSD_A_OP_LIN),
            "HSD_A_OP_SPL0" => Ok(InterpolationType::HSD_A_OP_SPL0),
            "HSD_A_OP_SPL" => Ok(InterpolationType::HSD_A_OP_SPL),
            "HSD_A_OP_SLP" => Ok(InterpolationType::HSD_A_OP_SLP),
            "HSD_A_OP_KEY" => Ok(InterpolationType::HSD_A_OP_KEY),
            _ => Err(ParseKeyError),
        }
    }
}

impl FromStr for Key {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_data = s.split(" ").collect::<Vec<_>>();
        Ok(Key {
            frame: f32::from_str(key_data[0])?,
            value: f32::from_str(key_data[1])?,
            tan: f32::from_str(key_data[2])?,
            interpolation_type: Self::parse_interpolation_type(key_data[3])?,
        })
    }
}

impl Track {
    pub fn parse_type(s: &str) -> Result<TrackType, ParseFigaTreeError> {
        match s {
            "HSD_A_J_NONE" => Ok(TrackType::HSD_A_J_NONE),
            "HSD_A_J_ROTX" => Ok(TrackType::HSD_A_J_ROTX),
            "HSD_A_J_ROTY" => Ok(TrackType::HSD_A_J_ROTY),
            "HSD_A_J_ROTZ" => Ok(TrackType::HSD_A_J_ROTZ),
            "HSD_A_J_PATH" => Ok(TrackType::HSD_A_J_PATH),
            "HSD_A_J_TRAX" => Ok(TrackType::HSD_A_J_TRAX),
            "HSD_A_J_TRAY" => Ok(TrackType::HSD_A_J_TRAY),
            "HSD_A_J_TRAZ" => Ok(TrackType::HSD_A_J_TRAZ),
            "HSD_A_J_SCAX" => Ok(TrackType::HSD_A_J_SCAX),
            "HSD_A_J_SCAY" => Ok(TrackType::HSD_A_J_SCAY),
            "HSD_A_J_SCAZ" => Ok(TrackType::HSD_A_J_SCAZ),
            "HSD_A_J_NODE" => Ok(TrackType::HSD_A_J_NODE),
            _ => Err(ParseFigaTreeError),
        }
    }

    pub fn last_frame(&self) -> (f32, usize){
        let mut index = 0;
        let mut m = 0.;
        for (i,k) in self.keys.iter().enumerate() {
            if k.frame > m {
                index = i;
                m = k.frame;
            }
        }
        (m, index)
    }

    pub fn get_anim_state(&self, frame: f32) -> AnimState {
        let (last_frame, last_frame_index) = self.last_frame();
        if self.keys.len() > 1 && frame >= last_frame {
            let key = &self.keys[last_frame_index];
            return AnimState {
                p0: key.frame, p1: key.frame,
                d0: key.value, d1: key.value,
                t0: key.tan, t1: key.tan,
                op_intrp: key.interpolation_type, op: key.interpolation_type,
            };
        } else {
            let (mut p0, mut p1, mut d0, mut d1, mut t0, mut t1) = (0., 0., 0., 0., 0., 0.);
            let (mut op_intrp, mut op) = (InterpolationType::HSD_A_OP_CON, InterpolationType::HSD_A_OP_CON);
            for key in &self.keys {
                op_intrp = op;
                op = key.interpolation_type;
                match op {
                    InterpolationType::HSD_A_OP_CON | InterpolationType::HSD_A_OP_LIN => {
                        p0 = p1;
                        p1 = key.value;
                        if op_intrp != InterpolationType::HSD_A_OP_SLP {
                            d0 = d1;
                            d1 = 0.;
                        }
                        t0 = t1;
                        t1 = key.frame;
                    },
                    InterpolationType::HSD_A_OP_SPL0 => {
                        p0 = p1;
                        p1 = key.value;
                        d0 = d1;
                        d1 = 0.;
                        t0 = t1;
                        t1 = key.frame;
                    },
                    InterpolationType::HSD_A_OP_SPL => {
                        p0 = p1;
                        p1 = key.value;
                        d0 = d1;
                        d1 = key.tan;
                        t0 = t1;
                        t1 = key.frame;
                    },
                    InterpolationType::HSD_A_OP_SLP => {
                        d0 = d1;
                        d1 = key.tan;
                    },
                    InterpolationType::HSD_A_OP_KEY => {
                        p0 = key.value;
                        p1 = key.value;
                    }
                    InterpolationType::HSD_A_OP_NONE => {},
                }
                if t1 > frame && key.interpolation_type != InterpolationType::HSD_A_OP_SLP {
                    break
                }
                op_intrp = key.interpolation_type;
            }
            return AnimState {p0, p1, d0, d1, t0, t1, op_intrp, op};
        }
    }

    pub fn get_value(&self, frame: f32) -> f32 {
        let anim_state = self.get_anim_state(frame);
        if frame == anim_state.t0 {
            anim_state.p0
        } else if  frame == anim_state.t1 {
            anim_state.p1
        } else if anim_state.t0 == anim_state.t1 || anim_state.op_intrp == InterpolationType::HSD_A_OP_CON || anim_state.op_intrp == InterpolationType::HSD_A_OP_KEY {
            anim_state.p0
        } else {
            let frame_diff = frame - anim_state.t0;
            let weight = frame_diff / (anim_state.t1 - anim_state.t0);

            match anim_state.op_intrp {
                InterpolationType::HSD_A_OP_LIN => lerp_interpolation(anim_state.p0, anim_state.p1, weight),
                _ => hermite_spline_interpolation(1. / (anim_state.t1 - anim_state.t0), frame_diff, anim_state.p0, anim_state.p1, anim_state.d0, anim_state.d1)
            }
        }
    }
}


impl Animation {
    pub fn from_model(model: Model) -> Self {
        Self {
            model: model,
            frame_count: 0.,
            hurtboxes: vec![],
        }
    }

    pub fn from_smd_path(path: &str) -> Result<Self, ParseSMDError> {
        Ok(Self::from_model(Model::from_smd_path(path)?))
    }

    pub fn from_smd_bytes(bytes: &[u8]) -> Result<Self, ParseSMDError> {
        Ok(Self::from_model(Model::from_smd_bytes(bytes)?))
    }

    pub fn load_figatree_from_path(&mut self, path: &str) -> Result<(), ParseFigaTreeError> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        self.load_figatree(&lines)
    }

    pub fn load_figatree_from_bytes(&mut self, bytes: &[u8]) -> Result<(), ParseFigaTreeError> {
        let reader = Cursor::new(bytes);
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        self.load_figatree(&lines)
    }

    pub fn load_figatree(&mut self, lines: &Vec<String>) -> Result<(), ParseFigaTreeError> {
        let mut current_bone = -1;
        let mut current_track_type = TrackType::HSD_A_J_NONE;
    
        let mut tracks: Vec<Track> = vec![];
        let mut keys: Vec<Key> = vec![];
    
        for line in lines {
            let line = &line.trim().to_string();
            if line.starts_with("FrameCount: ") {
                self.frame_count = f32::from_str(&line.replace("FrameCount: ", ""))?;
            }
            else if line.starts_with("Node ") {
                if current_bone >= 0 {
                    if let Some(bone_index) = self.model.indexes.get(&current_bone) {
                        let mut bone = &mut self.model.bones[*bone_index];
                        bone.tracks = tracks.clone();
                    }
                }
                current_bone = i32::from_str(&line.replace("Node ", "").replace(":", ""))?;
                tracks = vec![];
            } else if line.starts_with("HSD_A_J_") {
                current_track_type = Track::parse_type(&line)?;
                keys = vec![];
            } else if line.contains("HSD_A_OP") {
                let mut key = Key::from_str(&line)?;
                if key.frame > self.frame_count {
                    key.frame = self.frame_count;
                }
                keys.push(key);
            } else if line == "}" {
                tracks.push(Track{r#type: current_track_type, keys: keys.clone()})
            }
        }
        Ok(())
    }

    pub fn get_frame_model(&self, frame: f32) -> Model {
        let mut anim_model = self.model.clone();
        anim_model.compute_frame_transform(frame);
        anim_model
    }

    pub fn attach_hurtboxes(&mut self, hurtboxes: Vec<Hurtbox>) {
        self.hurtboxes = hurtboxes;
        // TODO: trim bones to fix only hurboxes bone_index
    }

    pub fn get_frame_hurtboxes_2d(&self, frame: f32) -> Vec<(Point2<f32>, Point2<f32>, f32)>{
        let model = self.get_frame_model(frame);
        let mut hurtboxes_2d: Vec<(Point2<f32>, Point2<f32>, f32)> = vec![];
        for hb in &self.hurtboxes {
            if let Some(index) = model.indexes.get(&hb.bone_index) {
                let bone = &model.bones[*index];
                let hbc1 = bone.transform() * hb.p1() * nalgebra::Point3::origin();
                let hbc2 = bone.transform() * hb.p2() * nalgebra::Point3::origin();
                hurtboxes_2d.push((Point2{x: hbc1.z, y: hbc1.y}, Point2{x: hbc2.z, y: hbc2.y}, hb.size))
            }
        }
        hurtboxes_2d
    }
}

fn lerp_interpolation(lhs: f32, rhs: f32, weight: f32) -> f32 {
    lhs * (1. - weight) + rhs * weight
}

fn hermite_spline_interpolation(fterm: f32, time: f32, p0: f32, p1: f32, d0: f32, d1: f32) -> f32 {
    let fvar1 = time * time;
    let mut fvar2 = fterm * fterm * time * fvar1;
    let fvar3 = 3. * fvar1 * fterm * fterm;
    let fvar4 = fvar2 - fvar1 * fterm;
    fvar2 = 2. * fvar2 * fterm;
    d1 * fvar4 + d0 * (time + (fvar4 - fvar1 * fterm)) + p0 * (1. + (fvar2 - fvar3)) + p1 * (-fvar2 + fvar3)
}
