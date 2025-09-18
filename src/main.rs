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
use monster::MonsterSystem;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;

mod player;
use rltk::{to_cp437, GameState, Point, RandomNumberGenerator, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}
impl GameState for State {
    // this gets called at each frame - it's kind of the renderer I guess
    fn tick(&mut self, ctx: &mut Rltk) {
        read_input(self, ctx);

        if self.runstate == RunState::Running {
            self.run_systems();

            ctx.cls();
            draw_map(&self.ecs, ctx);

            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            let names = self.ecs.read_storage::<Name>();
            let map = self.ecs.fetch::<Map>();

            // draw entities with a renderable compoennt attached
            for (pos, render, _name) in (&positions, &renderables, &names).join() {
                let idx = map.xy_idx(pos.x, pos.y);
                if map.visible_tiles[idx] {
                    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                }
            }
            self.runstate = RunState::Paused;
        } else {
            self.runstate = read_input(self, ctx);
        }
    }
}

// qui per leggere la tastiera
pub fn read_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused, // nothing happened
        Some(key) => match key {
            // movement
            VirtualKeyCode::Up | VirtualKeyCode::K => player::try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::J => player::try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Left | VirtualKeyCode::H => player::try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::L => player::try_move_player(1, 0, &mut gs.ecs),

            // teleport the player to a random room
            VirtualKeyCode::Space => player::move_to_random_room(&mut gs.ecs),

            // matchall
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        let mut rand_mov = MonsterSystem {};
        let mut map_indexing = MapIndexingSystem {};
        rand_mov.run_now(&self.ecs);
        map_indexing.run_now(&self.ecs);
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("My fancy RLTK game")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    let map = map::Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // registro i componenti?
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<Name>();

    // because many systems will require this
    gs.ecs.insert(Point::new(player_x, player_y));

    // creo un'entita player
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(BlocksTile {})
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 10,
            dirty: true,
        })
        .build();

    // creo un'entita monster per ogni stanza
    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let monster_pos = room.center();
        let create_entity = gs.ecs.create_entity();
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        let name: String;

        match roll {
            1 => {
                glyph = to_cp437('$');
                name = "Vosklamati".to_string();
            }
            _ => {
                glyph = to_cp437('£');
                name = "Vokastati".to_string();
            }
        }
        create_entity
            .with(Position {
                x: monster_pos.0,
                y: monster_pos.1,
            })
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(Monster {})
            .with(BlocksTile {})
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
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
