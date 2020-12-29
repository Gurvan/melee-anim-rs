use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::collections::BTreeMap;
use nalgebra::geometry::{UnitQuaternion, Translation3, Isometry3};

use crate::animation::{Track, TrackType};

#[derive(Debug, Clone, Copy)]
pub struct Joint {
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
}

#[derive(Debug, Clone)]
pub struct Bone {
    pub index: i32,
    pub parent: i32,
    pub childs: Vec<i32>,
    pub name: String,
    pub joint: Joint,
    pub transform: Option<Isometry3<f32>>,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub bones: Vec<Bone>,
    pub indexes: BTreeMap<i32,usize>,
    pub root_bone_index: i32,
}

#[derive(Debug, Clone)]
pub struct ParseBoneError;

impl From<std::num::ParseIntError> for ParseBoneError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseBoneError{}
    }
}

impl From<std::num::ParseFloatError> for ParseBoneError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseBoneError{}
    }
}

#[derive(Debug, Clone)]
pub struct ParseJointError;

impl From<std::num::ParseIntError> for ParseJointError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseJointError{}
    }
}

impl From<std::num::ParseFloatError> for ParseJointError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseJointError{}
    }
}

#[derive(Debug, Clone)]
pub struct ParseSMDError;

impl From<ParseJointError> for ParseSMDError {
    fn from(_: ParseJointError) -> Self {
        ParseSMDError{}
    }
}

impl From<ParseBoneError> for ParseSMDError {
    fn from(_: ParseBoneError) -> Self {
        ParseSMDError{}
    }
}

impl From<std::io::Error> for ParseSMDError {
    fn from(_: std::io::Error) -> Self {
        ParseSMDError{}
    }
}

impl Joint {
    pub fn new() -> Joint {
        Joint {
            tx: 0.,
            ty: 0.,
            tz: 0.,
            rx: 0.,
            ry: 0.,
            rz: 0.,
        }
    }
    
    pub fn parse(s: &str) -> Result<(i32, Joint), ParseJointError> {
        let joint = s.split(" ").collect::<Vec<_>>();
        if joint.len() == 7 {
            let index = i32::from_str(joint[0])?;
            let tx = f32::from_str(joint[1])?;
            let ty = f32::from_str(joint[2])?;
            let tz = f32::from_str(joint[3])?;
            let rx = f32::from_str(joint[4])?;
            let ry = f32::from_str(joint[5])?;
            let rz = f32::from_str(joint[6])?;
            return Ok((index, Joint {tx, ty, tz, rx, ry, rz}));
        }
        return Err(ParseJointError);
    }
}

impl Bone {
    pub fn new(index: i32, parent: i32, name: String) -> Bone {
        Bone {
            index: index,
            parent: parent,
            name: name,
            childs: vec![],
            joint: Joint::new(),
            transform: None,
            tracks: vec![],
        }
    }

    pub fn local_transform(&self) -> Isometry3<f32> {
        Isometry3::from_parts(self.local_translation(), self.local_rotation())
    }

    pub fn local_translation(&self) -> Translation3<f32> {
        Translation3::new(self.joint.tx, self.joint.ty, self.joint.tz)
    }

    pub fn local_rotation(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_euler_angles(self.joint.rx, self.joint.ry, self.joint.rz)
    }

    pub fn transform(&self) -> Isometry3<f32> {
        match self.transform {
            Some(transform) => transform,
            None => Isometry3::identity(),
        }
    }

    pub fn translation(&self) -> Translation3<f32> {
        match self.transform {
            Some(transform) => transform.translation,
            None => Translation3::identity(),
        }
    }

    pub fn rotation(&self) -> UnitQuaternion<f32> {
        match self.transform {
            Some(transform) => transform.rotation,
            None => UnitQuaternion::identity(),
        }
    }
}

impl FromStr for Bone {
    type Err = ParseBoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index_name_parent = s.split(" ").collect::<Vec<_>>();
        if index_name_parent.len() == 3 {
            let index = i32::from_str(index_name_parent[0])?;
            let parent = i32::from_str(index_name_parent[2])?;
            let name = index_name_parent[1].replace("\"", "").trim().to_string();
            return Ok(Bone::new(index, parent, name));
        }
        return Err(ParseBoneError);
    }
}


impl Model {
    pub fn from_smd_path(path: &str) -> Result<Self, ParseSMDError> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Self::from_smd(&lines)
    }
    
    pub fn from_smd_bytes(bytes: &[u8]) -> Result<Self, ParseSMDError> {
        let reader = Cursor::new(bytes);
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Self::from_smd(&lines)
    }
    
    pub fn from_smd(lines: &Vec<String>) -> Result<Self, ParseSMDError> {
        let mut nodes_flag = false;
        let mut skeleton_flag = false;
    
        let mut bones: Vec<Bone> = vec![];
    
        for line in lines {
            let line = &line.trim().to_string();
    
            if line.contains("nodes") {
                nodes_flag = true;
            }
            if line.contains("skeleton") {
                skeleton_flag = true;
            }
            if nodes_flag && line.split(" ").count() == 3 {
                &bones.push(Bone::from_str(&line)?);
            }
            if skeleton_flag && line.split(" ").count() == 7 {
                let (index, joint) = Joint::parse(&line)?;
                for mut bone in &mut bones {
                    if bone.index == index {
                        bone.joint = joint;
                    }
                }
            }
        }
        let indexes = Self::make_index(&bones);
        let mut root_bone_index = 0;
        for bone in &bones {
            if bone.parent == -1 {
                root_bone_index = bone.index;
            }
        }
        let mut model = Model {bones, indexes, root_bone_index};
        model.compute_childs();
        Ok(model)
    }

    pub fn make_index(bones: &Vec<Bone>) -> BTreeMap<i32, usize> {
        let mut indexes: BTreeMap<i32,usize> = BTreeMap::new();
        for (index, bone) in bones.iter().enumerate() {
            indexes.insert(bone.index, index);
        }
        indexes
    }

    pub fn compute_childs(&mut self) {
        assert!(self.bones.len() == self.indexes.len());
        for bone in self.bones.clone() {
            if let Some(parent_index) = self.indexes.get(&bone.parent) {
                self.bones[*parent_index].childs.push(bone.index);
            }
        }
    }

    pub fn update_transforms(&mut self, bone_index: i32, parent_index: Option<i32>) {
        let bone = &self.bones[self.indexes[&bone_index]];
        let mut local_transform = bone.local_transform();
        if let Some(index) = parent_index {
            let parent = &self.bones[self.indexes[&index]];
            if let Some(parent_transform) = parent.transform {
                local_transform = parent_transform * local_transform;
            } 
        }
        let mut bone = &mut self.bones[self.indexes[&bone_index]];
        bone.transform = Some(local_transform.clone());
        // println!("{:?}, {:?}", bone_index, parent_index);
        // println!("{:?}", bone.transform);
        for child_index in &bone.childs.clone() {
            self.update_transforms(*child_index, Some(bone_index));
        }
    }

    pub fn compute_t_pose_transform(&mut self) {
        self.update_transforms(self.root_bone_index, None);
    }

    pub fn update_joints(&mut self, frame: f32) {
        for bone in &mut self.bones {
            for track in &bone.tracks {
                match track.r#type {
                    TrackType::HSD_A_J_ROTX => bone.joint.rx = track.get_value(frame),
                    TrackType::HSD_A_J_ROTY => bone.joint.ry = track.get_value(frame),
                    TrackType::HSD_A_J_ROTZ => bone.joint.rz = track.get_value(frame),
                    TrackType::HSD_A_J_TRAX => bone.joint.tx = track.get_value(frame),
                    TrackType::HSD_A_J_TRAY => bone.joint.ty = track.get_value(frame),
                    TrackType::HSD_A_J_TRAZ => bone.joint.tz = track.get_value(frame),
                    _ => (),
                }
            }
        }
    }

    pub fn compute_frame_transform(&mut self, frame:f32) {
        self.update_joints(frame);
        self.update_transforms(self.root_bone_index, None);
    }
}
