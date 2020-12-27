#[derive(Debug, PartialEq, Clone, Copy)]
enum InterpolationType {
    HSD_A_OP_NONE,
    HSD_A_OP_CON,
    HSD_A_OP_LIN,
    HSD_A_OP_SPL0,
    HSD_A_OP_SPL,
    HSD_A_OP_SLP,
    HSD_A_OP_KEY,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TrackType {
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

#[derive(Debug, PartialEq, Clone)]
struct Key {
    frame: f32,
    value: f32,
    tan: f32,
    interpolation_type: InterpolationType,
}

#[derive(Debug, PartialEq, Clone)]
struct AnimState {
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
struct Track {
    r#type: TrackType,
    keys: Vec<Key>,
}


impl Track {
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
