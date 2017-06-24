extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::rc::Rc;
use std::vec::Vec;
use piston_window::*;
use sprite::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct AnimationState {
    vignete: (u32, u32),
    current: usize,
    accum_time: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// use to divide the asset textures into regions
/// each region is a full sprite keyframe
struct Grid{
    tile_size: (u32, u32),
    tile_count: (u32, u32),
}

#[derive(Debug)]
struct GridError;

impl Grid{
    fn new(tile_size: (u32, u32), tile_count: (u32, u32)) -> Grid{
        Grid{
            tile_size: tile_size,
            tile_count: tile_count,
        }
    }

    /// returns the texture rectagle that contains the keyframe we requested
    fn get_keyframe_rect(&self, state: &AnimationState ) -> Result<[f64; 4], GridError>{

        let (i, j) = state.vignete;
        let (max_i, max_j) = self.tile_count;

        if max_i <= i { return Err(GridError);}
        if max_j <= j { return Err(GridError);}
        Ok([self.tile_size.0 as f64 * i as f64, self.tile_size.1 as f64 * j as f64, self.tile_size.0 as f64, self.tile_size.1 as f64])
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct Animation{
    steps: Vec<(u32, u32)>,
    frame_time: f64,
}

impl Animation{
    fn new(frame_time: f64, steps: Vec<(u32, u32)>) -> Animation{
        Animation{
            steps: steps,
            frame_time: frame_time
        }
    }

    fn get_start(&self) -> AnimationState{
        AnimationState{ 
            vignete:(0,0), 
            current: 0,
            accum_time: 0.0
        }
    }

    fn update_state(&self, state: &mut AnimationState, dt: f64) -> bool{
        let total = state.accum_time + dt;

        if total > self.frame_time{
            state.accum_time = 0.0;
            state.current = (state.current + 1) % self.steps.len();
            state.vignete = self.steps[state.current];

            println!(" {} -> {:?}", state.current, state.vignete);
            return true;
        }

        state.accum_time = total;
        return false;
    }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {
    let width = 800;
    let height = 800;

    // ~~~~~~~~~~~~~~ init window ~~~~~~~~~~~~~~~~~~~
    let mut window: PistonWindow = WindowSettings::new("hey!", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // ~~~~~~~~~~~~~~~ Load Assets ~~~~~~~~~~~~~~~~~~

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let mut scene = Scene::new();
    let tex = Texture::from_path(&mut window.factory, assets.join("anim.png"), Flip::None, &TextureSettings::new());
    let tex = Rc::new(tex.expect("could not find the damm texture asset"));

    let grid = Grid::new((75,120), (8, 2));
    let anim = Animation::new(1.0, vec![(0,0),(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(7,0),
                                        (0,1),(1,1),(2,1),(3,1),(4,1),(5,1),(6,1),(7,1)]);
    let mut anim_state = anim.get_start();

    let mut sprite = Sprite::from_texture_rect(tex.clone(), grid.get_keyframe_rect(&anim_state).expect("animation index is wrong"));
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);
    let id = scene.add_child(sprite);

    // ~~~~~~~~~~~~~~~ Event Loop ~~~~~~~~~~~~~~~~~~~
    while let Some(e) =  window.next() {

        scene.event(&e);

        // ~~~~~~~~~~~~~~~ Process Input  ~~~~~~~~~~~~~~~~~~~
        match e {
            Input::Update(args) => {
                if  anim.update_state(&mut anim_state, args.dt){

                    let rec = grid.get_keyframe_rect(&anim_state).expect("update out of bounds");
                    scene.child_mut(id).expect("it should be still there").set_src_rect(rec);
                }

            },
            _ => {}
        }

        // ~~~~~~~~~~~~~~~ Draw ~~~~~~~~~~~~~~~~~~~
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            scene.draw(c.transform, g);
        });
    }
}
