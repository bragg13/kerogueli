use crate::Rect;
use rltk::{Rltk, RGB};
use std::cmp::{max, min};

// so I can copy and not "move", clone programmatically, and check for type equality
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor
        }
    }
}

fn apply_htunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}
fn apply_vtunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rooms: Vec<Rect> = Vec::new();
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
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);
            rooms.push(new_room);
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(1.0, 0., 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 0., 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
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
