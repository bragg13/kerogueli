use crate::{Player, Rect, Viewshed};
use rltk::{Algorithm2D, BaseMap, Point, Rltk, RGB};
use specs::{Join, World, WorldExt};
use std::cmp::{max, min};

// so I can copy and not "move", clone programmatically, and check for type equality
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Water,
    Ground,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    /// Returns the index of a tile given its X and Y position
    pub const fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Build a room in a map
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Ground;
            }
        }
    }

    /// Build a horizontal tunnel between to rooms
    fn apply_htunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            let height_times_width = self.width as usize * self.height as usize;
            if idx > 0 && idx < height_times_width {
                self.tiles[idx as usize] = TileType::Ground;
            }
        }
    }
    fn apply_vtunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            let height_times_width = self.width as usize * self.height as usize;
            if idx > 0 && idx < height_times_width {
                self.tiles[idx as usize] = TileType::Ground;
            }
        }
    }

    /// Generate a new map with random rooms connected by corridors
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Water; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50],
        };

        const MAX_ROOMS: i32 = 16;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 15;

        let mut rng = rltk::RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w: i32 = rng.range(MIN_SIZE, MAX_SIZE);
            let h: i32 = rng.range(MIN_SIZE, MAX_SIZE);

            let x = rng.roll_dice(1, 80 - w - 1);
            let y = rng.roll_dice(1, 50 - h - 1);
            let new_room = Rect::new(x, y, w, h);

            // check if new room overlaps with others
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                // add corridors if not the first room
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_htunnel(prev_x, new_x, prev_y);
                        map.apply_vtunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vtunnel(prev_y, new_y, prev_x);
                        map.apply_htunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        }

        map
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, _viewshed) in (&mut players, &mut viewsheds).join() {
        let mut x = 0;
        let mut y = 0;

        for (idx, tile) in map.tiles.iter().enumerate() {
            // render a tile based on its type
            if map.revealed_tiles[idx] {
                match tile {
                    TileType::Water => {
                        ctx.set(
                            x,
                            y,
                            // RGB::from_f32(1.0, 1.0, 1.0),
                            RGB::from_u8(37, 150, 200),
                            RGB::from_u8(37, 150, 190),
                            rltk::to_cp437('.'),
                        );
                    }
                    TileType::Ground => {
                        ctx.set(
                            x,
                            y,
                            RGB::from_f32(0., 0., 0.),
                            RGB::from_u8(234, 182, 118),
                            rltk::to_cp437('.'),
                        );
                    }
                }
            }

            // move the coords
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Water
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
