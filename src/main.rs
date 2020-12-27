#![allow(non_camel_case_types, dead_code)]

mod bones;
mod hurtboxes;
mod animation;

// extern crate lyon;
// use lyon::math::{point, Point};
// use lyon::path::Path;
// use lyon::path::builder::*;
// use lyon::tessellation::*;

// use bevy::prelude::*;
// use bevy_prototype_lyon::prelude::*;

use bones::Model;

fn main() {
    let smd_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/Fox.smd";
    let hurtbox_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/hurtboxes.csv";

    let mut model = Model::from_smd(&smd_path).unwrap();
    let _hurtboxes = hurtboxes::parse_hurtboxes(&hurtbox_path).unwrap();
    // println!("{:?}", model);
    // println!("{:?}", hurtboxes);
    // println!("Bones: {:?}, Hurtboxes: {:?}", model.bones.len(), hurtboxes.len());
    model.compute_t_pose_transform();
    for bone in &model.bones {
        println!("{:?}: {:?}", bone.index, bone.translation().vector.data);
    }

    // Rendering code
    // let mut builder = Path::builder();
    // builder.move_to(point(0.0, 0.0));
    // builder.line_to(point(1.0, 0.0));
    // builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
    // builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
    // builder.close();
    // let path = builder.build();
    // // Let's use our own custom vertex type instead of the default one.
    // #[derive(Copy, Clone, Debug)]
    // struct MyVertex { position: [f32; 2] };
    // // Will contain the result of the tessellation.
    // let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    // let mut tessellator = FillTessellator::new();
    // {
    //     // Compute the tessellation.
    //     tessellator.tessellate_path(
    //         &path,
    //         &FillOptions::default(),
    //         &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
    //             MyVertex {
    //                 position: pos.to_array(),
    //             }
    //         }),
    //     ).unwrap();
    // }
    // // The tessellated geometry is ready to be uploaded to the GPU.
    // println!(" -- {} vertices {} indices",
    //     geometry.vertices.len(),
    //     geometry.indices.len()
    // );

}


// fn main() {
//     App::build()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup.system())
//         .run();
// }


// fn setup(
//     commands: &mut Commands,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     let smd_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/Fox.smd";
//     let hurtbox_path = "/Users/gurvan/Perso/mini_melee_dir/melee-anim-rs/assets/Fox/hurtboxes.csv";

//     let mut model = Model::from_smd(&smd_path).unwrap();
//     let hurtboxes = hurtboxes::parse_hurtboxes(&hurtbox_path).unwrap();
//     // println!("{:?}", model);
//     // println!("{:?}", hurtboxes);
//     // println!("Bones: {:?}, Hurtboxes: {:?}", model.bones.len(), hurtboxes.len());
//     model.compute_t_pose_transform();
//     for bone in &model.bones {
//         println!("{:?}: {:?}", bone.index, bone.translation().vector.data);
//     }



//     let material = materials.add(Color::rgba(1., 1., 0., 0.5).into());

//     commands
//         .spawn(Camera2dBundle::default())
//         .spawn(primitive(
//             // materials.add(Color::rgb(0., 0., 0.,).into()),
//             materials.add(Color::BLACK.into()),
//             &mut meshes,
//             ShapeType::Rectangle{ width: 1000., height: 1000.},
//             TessellationMode::Fill(&FillOptions::default()),
//             Vec3::new(-500., -500., 0.).into(),
//         ))
//         .spawn(primitive(
//             material.clone(),
//             &mut meshes,
//             ShapeType::Circle(60.0),
//             TessellationMode::Stroke(&StrokeOptions::default()),
//             Vec3::new(0.0, 0.0, 0.0).into(),
//         ));
// }