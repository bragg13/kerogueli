// import as module, and then use its public content
mod components;
pub use components::*;
mod map;
pub use map::*;
mod rect;
pub use rect::Rect;

mod player;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
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
        let map = self.ecs.fetch::<Map>();
        draw_map(&map.tiles, ctx);

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
    gs.ecs.insert(map);

    // registro i componenti?
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

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
        .build();

    rltk::main_loop(context, gs)
}
