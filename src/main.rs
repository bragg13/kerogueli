// import as module, and then use its public content
mod components;
pub use components::*;
mod map;
pub use map::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster;
use monster::RandomMovementSystem;

mod player;
use rltk::{GameState, RandomNumberGenerator, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

pub struct State {
    pub ecs: World,
}
impl GameState for State {
    // this gets called at each frame - it's kind of the renderer I guess
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        read_input(self, ctx);
        self.run_systems();
        // let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// qui per leggere la tastiera
pub fn read_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {} // nothing happened
        Some(key) => match key {
            // movement
            VirtualKeyCode::Up | VirtualKeyCode::W => player::try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => player::try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Left | VirtualKeyCode::A => player::try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => player::try_move_player(1, 0, &mut gs.ecs),

            // teleport the player to a random room
            VirtualKeyCode::Space => player::move_to_random_room(&mut gs.ecs),

            // matchall
            _ => {}
        },
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        let mut rand_mov = RandomMovementSystem {};
        rand_mov.run_now(&self.ecs);
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("My fancy RLTK game")
        .build()?;

    let mut gs = State { ecs: World::new() };
    let map = map::Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // registro i componenti?
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<RandomMovement>();

    // creo un'entita player
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 12,
            dirty: true,
        })
        .build();

    // creo un'entita monster per ogni stanza
    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let monster_pos = room.center();
        let create_entity = gs.ecs.create_entity();
        let move_prob = rng.range(0, 101);
        create_entity
            .with(Position {
                x: monster_pos.0,
                y: monster_pos.1,
            })
            .with(RandomMovement {
                probability: move_prob,
            })
            .with(Renderable {
                glyph: rltk::to_cp437('!'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::from_u8(234, 182, 118),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 5,
                dirty: true,
            })
            .build();
    }
    // for room in &map.rooms {
    // }

    gs.ecs.insert(map);
    rltk::main_loop(context, gs)
}
