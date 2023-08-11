use serde::{Deserialize, Serialize};

use crate::game_constants::{
    CELL_DEATH, HISTORY_SIZE, MAX_X, MAX_Y, MIN_X, MIN_Y, WORLD_SIZE_X, WORLD_SIZE_Y, UniversePlane, UniversePlaneSet, UniverseCell
};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldBounds {
    pub x: isize,
    pub y: isize,
    pub w: isize,
    pub h: isize,
}

pub struct Universe {
    current_time: isize,
    space_time: Vec<UniverseCell>,
    species: UniversePlaneSet,
}

impl Universe {
    pub fn new() -> Universe {
        let mut universe = Universe {
            current_time: 0,
            space_time: vec![CELL_DEATH; (WORLD_SIZE_Y * WORLD_SIZE_X * HISTORY_SIZE) as usize],
            species: Vec::new(),
        };
        universe.init_time();
        universe
    }

    #[inline]
    pub fn get_dimensions() -> (isize, isize, isize) {
        (HISTORY_SIZE, WORLD_SIZE_X, WORLD_SIZE_Y)
    }

    fn init_time(&mut self) -> () {
        for x_pos in MIN_X..=MAX_X {
            for y_pos in MIN_Y..=MAX_Y {
                let (x_index, y_index) = self.position_to_index(x_pos, y_pos);

                self.space_time[Universe::map_3d_to_1d_index(self.current_time, x_index, y_index)] =
                    // TODO: Enable this ...
                    CELL_DEATH;
                // rand::thread_rng().gen_range(CELL_DEATH..=CELL_LIVE);
                //
            }
        }
    }

    #[inline]
    pub fn map_3d_to_1d_index(t: isize, x: isize, y: isize) -> usize {
        (t * WORLD_SIZE_Y * WORLD_SIZE_X + y * WORLD_SIZE_X + x) as usize
    }

    pub fn set_cell(
        &mut self,
        next_time: isize,
        x_pos: isize,
        y_pos: isize,
        x_offset: isize,
        y_offset: isize,
        cell_state: UniverseCell,
    ) -> () {
        let (x_index, y_index) = self.position_to_index(x_pos + x_offset, y_pos + y_offset);
        let (x_index, y_index) = self.fix_index(x_index, y_index);
        self.space_time[Universe::map_3d_to_1d_index(next_time, x_index, y_index)] = cell_state;
    }

    #[inline]
    pub fn set_cell_low_level(
        &mut self,
        next_time: isize,
        x_index: isize,
        y_index: isize,
        cell_state: UniverseCell,
    ) -> () {
        self.space_time[Universe::map_3d_to_1d_index(next_time, x_index, y_index)] = cell_state;
    }

    pub fn get_current_world(&self, bounds: WorldBounds) -> UniversePlane {
        let mut world_surface = vec![vec![CELL_DEATH; bounds.h as usize]; bounds.w as usize];

        for a in bounds.x..bounds.x + bounds.w {
            for b in bounds.y..bounds.y + bounds.h {
                let (al, bl) = self.position_to_index(a, b);
                world_surface[(a - bounds.x) as usize][(b - bounds.y) as usize] =
                    self.get_current_time_cell(al, bl);
            }
        }

        world_surface
    }

    pub fn get_current_time_cell(&self, x_index: isize, y_index: isize) -> UniverseCell {
        let mut x_f: isize = x_index;
        let mut y_f: isize = y_index;

        while x_f < 0 {
            x_f += WORLD_SIZE_X;
        }
        while x_f >= WORLD_SIZE_X {
            x_f -= WORLD_SIZE_X;
        }
        while y_f < 0 {
            y_f += WORLD_SIZE_Y;
        }
        while y_f >= WORLD_SIZE_Y {
            y_f -= WORLD_SIZE_Y;
        }

        self.space_time[Universe::map_3d_to_1d_index(self.current_time, x_f, y_f)]
    }

    fn fix_index(&self, x_index: isize, y_index: isize) -> (isize, isize) {
        let mut x_f: isize = x_index;
        let mut y_f: isize = y_index;

        while x_f < 0 {
            x_f += WORLD_SIZE_X;
        }
        while x_f >= WORLD_SIZE_X {
            x_f -= WORLD_SIZE_X;
        }
        while y_f < 0 {
            y_f += WORLD_SIZE_Y;
        }
        while y_f >= WORLD_SIZE_Y {
            y_f -= WORLD_SIZE_Y;
        }

        (x_f, y_f)
    }

    #[inline]
    pub fn position_to_index(&self, x_pos: isize, y_pos: isize) -> (isize, isize) {
        (x_pos - MIN_X, y_pos - MIN_Y)
    }

    #[inline]
    pub fn get_current_time(&self) -> isize {
        self.current_time
    }

    #[inline]
    pub fn set_current_time(&mut self, time: isize) -> () {
        self.current_time = time;
    }

    #[inline]
    pub fn push_entity(&mut self, entity: UniversePlane) -> () {
        self.species.push(entity);
    }

    #[inline]
    pub fn get_entities(&self) -> &UniversePlaneSet {
        &self.species
    }
}
