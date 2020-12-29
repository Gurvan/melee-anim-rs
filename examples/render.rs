use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::mint::Point2;
use ggez::event::{self, EventHandler};


use melee_anim_rs::bone::Model;
use melee_anim_rs::animation::Animation;


#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {() => (
    r"\"
)}
#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {() => (
    r"/"
)}


const SCREEN_SIZE: (f32, f32) = (640.0, 528.0);
const RATIO: f32 = 10.0;

pub fn make_context() -> ContextBuilder {
    let cb = ggez::ContextBuilder::new("drawing", "ggez")
    .window_mode(ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
    .window_setup(ggez::conf::WindowSetup::default()
                .title("Melee Anim")
                .vsync(true));
    return cb;
}

pub fn scale_one(p: (f32, f32)) -> Point2<f32> {
    let (w, h) = (SCREEN_SIZE.0, SCREEN_SIZE.1);
    return Point2 { x: RATIO * p.0 + w / 2.0, y: -RATIO * p.1 + h / 2.0 };
}

pub fn draw_hurtboxes(hurtboxes_2d: Vec<(melee_anim_rs::animation::Point2<f32>, melee_anim_rs::animation::Point2<f32>, f32)>, ctx: &mut Context) -> GameResult<()> {

    for (c1, c2, size) in &hurtboxes_2d {
        let line_options = graphics::StrokeOptions::default()
        .with_line_width(2. * size * RATIO)
        .with_start_cap(graphics::LineCap::Round)
        .with_end_cap(graphics::LineCap::Round);

        let hbc = graphics::Mesh::new_polyline(
        ctx,
        graphics::DrawMode::Stroke(line_options),
        &[scale_one((c2.x, c2.y)), scale_one((c1.x, c1.y))],
        [1., 1., 0., 0.2].into(),
        )?;
        graphics::draw(ctx, &hbc, graphics::DrawParam::default())?;
    }
    Ok(())
}


fn main() -> GameResult {
    let cb = make_context();
    let (ctx, event_loop) = &mut cb.build()?;
    let mut my_game = MyGame::new(ctx);
    event::run(ctx, event_loop, &mut my_game)
}

struct MyGame {
    anim: Animation,
    frame: i32,
    model: Model,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let smd_data: &[u8] = include_bytes!(concat!("assets", PATH_SEPARATOR!(), "model.smd"));
        let hurtbox_data: &[u8] = include_bytes!(concat!("assets", PATH_SEPARATOR!(), "hurtboxes.csv"));
        let figatree_data: &[u8] = include_bytes!(concat!("assets", PATH_SEPARATOR!(), "animation.figatree"));

        let anim = melee_anim_rs::get_animation_with_hurtboxes(smd_data, hurtbox_data, figatree_data).unwrap();
        let model = anim.model.clone();
        MyGame {
            anim,
            frame: 0,
            model,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let slow = 5.;
        if self.frame >= (slow * self.anim.frame_count) as i32 {
            self.frame -= (slow * self.anim.frame_count) as i32;
        }
        self.frame += 1;
        graphics::clear(ctx, graphics::BLACK);
        let hurtboxes = self.anim.get_frame_hurtboxes_2d((self.frame as f32) / slow);
        draw_hurtboxes(hurtboxes, ctx)?;
        graphics::present(ctx)
    }
}
