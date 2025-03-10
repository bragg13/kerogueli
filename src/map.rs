use crate::Rect;
use rltk::{Algorithm2D, BaseMap, Point, Rltk, RGB};
use specs::World;
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
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
}

impl Map {
    /// Returns the index of a tile given its X and Y position
    pub const fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    // == pathfinding ==
    /// Given some coordinates, returns True if the tile is walkable
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        // borders of the screen
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx as usize]
    }

    // == rooms and corridors ==
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
            let idx_expanded = self.xy_idx(x, y + 1);
            let height_times_width = self.width as usize * self.height as usize;
            if idx > 0 && idx < height_times_width {
                self.tiles[idx as usize] = TileType::Ground;
                self.tiles[idx_expanded as usize] = TileType::Ground;
            }
        }
    }
    fn apply_vtunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            let idx_expanded = self.xy_idx(x + 1, y);
            let height_times_width = self.width as usize * self.height as usize;
            if idx > 0 && idx < height_times_width {
                self.tiles[idx as usize] = TileType::Ground;
                self.tiles[idx_expanded as usize] = TileType::Ground;
            }
        }
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = *tile == TileType::Water;
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
            visible_tiles: vec![false; 80 * 50],
            blocked: vec![false; 80 * 50],
        };

        const MAX_ROOMS: i32 = 26;
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
    let map = ecs.fetch::<Map>();
    let mut x = 0;
    let mut y = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        // render a tile based on its type
        if map.revealed_tiles[idx] {
            let glyph;
            let bg;
            let fg;
            match tile {
                TileType::Water => {
                    fg = RGB::from_u8(37, 150, 200);
                    bg = RGB::from_u8(37, 150, 190);
                    glyph = rltk::to_cp437('.');
                }
                TileType::Ground => {
                    fg = RGB::from_f32(0., 0., 0.);
                    bg = RGB::from_u8(234, 182, 118);
                    glyph = rltk::to_cp437('.');
                }
            }
            // this makes the revelaed tiles greyscale - not my fav effect
            // if !map.visible_tiles[idx] {
            //     bg = bg.to_greyscale();
            // }

            ctx.set(x, y, fg, bg, glyph);
        }

        // move the coords
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Water
    }

    /// Get heuristic distance between two points using Pythagoras theorem
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    /// Returns available exits, aka movements from the tile the entity is in
    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        exits
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
