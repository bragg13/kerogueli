use rltk::{Rltk, RGB};

// so I can copy and not "move", clone programmatically, and check for type equality
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Simple map 80x50, walls on the edges, and 10% randomly placed walls around
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // walls around
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 50 - 1)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(80 - 1, y)] = TileType::Wall;
    }

    // lets go random!
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 80 - 1);
        let y = rng.roll_dice(1, 50 - 1);
        let idx = xy_idx(x, y);

        // we're gonna start here - better not have a wall
        if idx != xy_idx(80 / 2, 50 / 2) {
            map[idx] = TileType::Wall;
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
