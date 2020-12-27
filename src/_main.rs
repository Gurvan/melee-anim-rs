mod bones;
mod hurtboxes;

extern crate kiss3d;
extern crate nalgebra as na;

use na::{Point3, Vector3, UnitQuaternion, Isometry3};
use na::geometry::Translation3;
use kiss3d::camera::ArcBall;
use kiss3d::window::Window;
use kiss3d::light::Light;

use bones::Model;

use na::Matrix4;

fn main() {
    let smd_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/Fox.smd";
    let hurtbox_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/hurtboxes.csv";

    let mut model = Model::from_smd(&smd_path).unwrap();
    let hurtboxes = hurtboxes::parse_hurtboxes(&hurtbox_path).unwrap();
    // println!("{:?}", model);
    // println!("{:?}", hurtboxes);
    // println!("Bones: {:?}, Hurtboxes: {:?}", model.bones.len(), hurtboxes.len());
    model.compute_t_pose_transform();
    // for bone in &model.bones {
    //     println!("{:?}: {:?}", bone.index, bone.xyz());
    // }

    // println!("{:?}", Matrix4::from_euler_angles(0., 1., 2.));


    // Rendering code
    let eye = Point3::new(0.0f32, 10., 30.);
    let at = Point3::new(0.0f32, 10., 0.);
    let mut arc_ball = ArcBall::new(eye, at);

    let mut window = Window::new("Kiss3d: T-Fox");
    // for hb in &hurtboxes {
    //     let index = hb.bone_index;
    //     let bone = &model.bones[model.indexes[&index]];
    //     let mut c = window.add_capsule(hb.size, hb.norm());
    //     c.prepend_to_local_transformation(&(bone.transform() * hb.transform()));
    //     c.set_color(1.0, 1.0, 0.0);
    // }


    window.set_light(Light::StickToCamera);
    while window.render_with_camera(&mut arc_ball) {
        for bone in &model.bones.clone() {
            let bone_point = bone.transform().translation * Point3::origin();
            if let Some(parent_index) = model.indexes.get(&bone.parent) {
                let parent_point = &model.bones[*parent_index].transform().translation * Point3::origin();
                // println!("{:?} -> {:?}", bone_point, parent_point);
                window.draw_line(&bone_point, &parent_point, &Point3::new(0., 1., 0.));
            }
            window.draw_point(&bone_point, &Point3::new(1., 0., 0.));
        }
        window.set_point_size(10.);
    }
}
