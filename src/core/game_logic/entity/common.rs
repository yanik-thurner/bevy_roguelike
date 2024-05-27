use std::collections::HashSet;
use bevy::prelude::Component;

pub use super::super::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(hp: u32) -> Self {
        Health {
            current: hp,
            max: hp,
        }
    }
}

#[derive(Component)]
pub struct FieldOfView
{
    pub visible_tiles: HashSet<Position>,
    pub radius: usize,
    pub is_dirty: bool,
}

impl FieldOfView {
    fn generate_circle_points(center: &Position, radius: usize) -> HashSet<(i32, i32)> {
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

    #[allow(dead_code)]
    fn dda_line(x0: i32, y0: i32, x1: i32, y1: i32, result_set: &mut HashSet<Position>, map: &Map) {
        let dx = x1 - x0;
        let dy = y1 - y0;

        let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() } as f32;

        let x_inc = dx as f32 / steps;
        let y_inc = dy as f32 / steps;

        let mut x = x0 as f32;
        let mut y = y0 as f32;

        for _ in 0..=steps as i32 {
            let next_position = Position::new(x.round() as i32, y.round() as i32);
            result_set.insert(next_position);
            if map.tiles.try_get(&next_position) == Some(&TileType::Wall) {
                break;
            }
            x += x_inc;
            y += y_inc;
        }
    }

    fn bresenham_line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, result_set: &mut HashSet<Position>, map: &Map) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            let new_position = Position::new(x0, y0);
            result_set.insert(new_position);
            if (x0 == x1 && y0 == y1) || map.tiles.try_get(&new_position) == Some(&TileType::Wall) { break; }
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
                result_set.insert(Position::new(x0, y0 - sy)); // Include point vertically adjacent
            } else {
                result_set.insert(Position::new(x0 - sx, y0)); // Include point horizontally adjacent
            }
        }
    }

    fn field_of_view_set(center: &Position, radius: usize, map: &Map) -> HashSet<Position> {
        let circle_points = Self::generate_circle_points(center, radius);
        let mut visible_points = HashSet::new();
        for circle in circle_points {
            Self::bresenham_line(center.x, center.y, circle.0, circle.1, &mut visible_points, &map);
        }

        visible_points
    }
    pub fn update_visible_tiles(&mut self, anchor_position: &Position, map: &Map) {
        self.visible_tiles = Self::field_of_view_set(anchor_position, self.radius, map);
    }

    pub fn new(radius: usize) -> Self{
        FieldOfView{
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self{
        FieldOfView{
            visible_tiles: self.visible_tiles.clone(),
            radius: self.radius,
            is_dirty: true
        }
    }
}