mod bones;
mod hurtboxes;
use bones::Model;
use nalgebra::Matrix2;

fn main() {
    let smd_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/Fox.smd";
    let hurtbox_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/hurtboxes.csv";

    let mut model = Model::from_smd(&smd_path).unwrap();
    let hurtboxes = hurtboxes::parse_hurtboxes(&hurtbox_path).unwrap();
    // println!("{:?}", model);
    println!("{:?}", hurtboxes);
    println!("Bones: {:?}, Hurtboxes: {:?}", model.bones.len(), hurtboxes.len());
    model.compute_t_pose_transform();
    for bone in &model.bones {
        println!("{:?}: {:?}", bone.index, bone.xyz());
    }
}
