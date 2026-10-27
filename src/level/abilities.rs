use crate::{constants::GRID_SIZE, level::coordinate::MapCoords};
use macroquad::prelude::*;

pub type Cost = u16;

pub const BOMB_COST: Cost = 10;

pub struct Bomb {
    radius: i8,
    timer: f32,
    explode_time: f32,
    coords: MapCoords,
    is_block: bool,
}

impl Bomb {
    pub const fn new(coords: MapCoords, is_block: bool) -> Self {
        Bomb {
            radius: 2,
            timer: 0.0,
            explode_time: 3.0,
            coords,
            is_block,
        }
    }

    pub fn bomb_update(&mut self) -> Option<Vec<MapCoords>> {
        self.timer += get_frame_time();

        let mut blocks_to_mine: Vec<MapCoords> = Vec::new();
        if self.timer >= self.explode_time {
            // manhattan distance to make a rhombus shape
            for dx in -self.radius..=self.radius {
                let max_dz = self.radius - dx.abs();
                for dz in -max_dz..=max_dz {
                    let x = self.coords.x.saturating_add_signed(dx);
                    let z = self.coords.z.saturating_add_signed(dz);

                    blocks_to_mine.push(MapCoords::new(x, z));
                }
            }
        }

        draw_bomb(self.coords, self.is_block);

        match blocks_to_mine.is_empty() {
            true => None,
            false => Some(blocks_to_mine),
        }
    }
}

fn draw_bomb(coords: MapCoords, is_block: bool) {
    const SCALE: f32 = 0.5;

    let y_coord = match is_block {
        true => GRID_SIZE - SCALE,
        false => 0.0,
    };

    draw_cube(
        vec3(
            GRID_SIZE * coords.x as f32,
            y_coord,
            GRID_SIZE * coords.z as f32,
        ),
        vec3(GRID_SIZE * SCALE, GRID_SIZE * SCALE, GRID_SIZE * SCALE),
        None,
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
    );
}
