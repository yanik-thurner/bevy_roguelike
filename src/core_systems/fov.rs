use bevy::utils::HashSet;
use crate::prelude::*;

fn generate_circle_points(center: &GridPosition, radius: usize) -> HashSet<(i32, i32)> {
    let mut points = HashSet::new();
    let x0 = center.x;
    let y0 = center.y;
    let mut x = radius as i32;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        points.insert((x0 + x, y0 + y));
        points.insert((x0 + y, y0 + x));
        points.insert((x0 - y, y0 + x));
        points.insert((x0 - x, y0 + y));
        points.insert((x0 - x, y0 - y));
        points.insert((x0 - y, y0 - x));
        points.insert((x0 + y, y0 - x));
        points.insert((x0 + x, y0 - y));

        y += 1;
        if err <= 0 {
            err += 2 * y + 1;
        } else {
            x -= 1;
            err -= 2 * x + 1;
        }
    }

    points
}

fn dda_line(x0: i32, y0: i32, x1: i32, y1: i32, result_set: &mut HashSet<GridPosition>, map: &Map) {
    let dx = x1 - x0;
    let dy = y1 - y0;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() } as f32;

    let x_inc = dx as f32 / steps;
    let y_inc = dy as f32 / steps;

    let mut x = x0 as f32;
    let mut y = y0 as f32;

    for _ in 0..=steps as i32 {
        let next_position = GridPosition::new(x.round() as i32, y.round() as i32);
        result_set.insert(next_position);
        if map.try_get_tile_type(&next_position) == Some(TileType::Wall) {
            break;
        }
        x += x_inc;
        y += y_inc;
    }
}

fn bresenham_line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, result_set: &mut HashSet<GridPosition>, map: &Map) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    loop {
        let new_position = GridPosition::new(x0, y0);
        result_set.insert(new_position);
        if (x0 == x1 && y0 == y1) || map.try_get_tile_type(&new_position) == Some(TileType::Wall) { break; }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
        // Add neighboring points for increased visibility coverage
        if dx > dy {
            result_set.insert(GridPosition::new(x0, y0 - sy)); // Include point vertically adjacent
        } else {
            result_set.insert(GridPosition::new(x0 - sx, y0)); // Include point horizontally adjacent
        }
    }
}

fn field_of_view_set(center: &GridPosition, radius: usize, map: &Map) -> HashSet<GridPosition> {
    let circle_points = generate_circle_points(center, radius);
    let mut visible_points = HashSet::new();
    for circle in circle_points {
        bresenham_line(center.x, center.y, circle.0, circle.1, &mut visible_points, &map);
    }

    visible_points
}

pub fn fov_system(mut views: Query<(&Position, &mut FieldOfView)>, mut map: ResMut<MapResource>) {
    views.iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(&pos.0, fov.radius, &map.0);
            fov.visible_tiles.iter().for_each(|pos| { let _ = map.0.reveal(&pos); });

            fov.is_dirty = false;
        }
        );
}