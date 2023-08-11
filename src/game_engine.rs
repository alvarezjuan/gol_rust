use rand::prelude::*;
use std::{
    sync::{mpsc::Receiver, Arc, RwLock},
    thread, time,
};
use stopwatch::Stopwatch;

use crate::game_constants::{
    UniversePlane, CELL_DEATH, CELL_LIVE, ENGINE_LOOP_DELAY_MILLIS, HISTORY_SIZE, MAX_X, MAX_Y,
    MIN_X, MIN_Y,
};
use crate::game_universe::Universe;

pub fn engine_loop(rwlock: Arc<RwLock<Universe>>, receiver: &Receiver<UniversePlane>) -> () {
    // Init
    let dimensions = Universe::get_dimensions();

    println!(
        "scheduler_loop() {} {} {}",
        dimensions.0, dimensions.1, dimensions.2
    );

    let ten_millis = time::Duration::from_millis(ENGINE_LOOP_DELAY_MILLIS);

    loop {
        let current_time: isize;

        let mut sw: Stopwatch = Stopwatch::start_new();

        {
            let mut unlocked_data = match rwlock.write() {
                Err(_) => {
                    continue;
                },
                Ok(data) => data
            };

            let universe = &mut *unlocked_data;

            current_time = universe.get_current_time();

            generate_next_time(universe, receiver);
        }

        sw.stop();

        println!(
            "scheduler_loop() elapsed {} [{} ms]",
            current_time,
            sw.elapsed_ms()
        );

        thread::sleep(ten_millis);
    }
}

fn generate_next_time(universe: &mut Universe, receiver: &Receiver<UniversePlane>) -> () {
    let next_time: isize = (universe.get_current_time() + 1) % HISTORY_SIZE;

    for x_pos in MIN_X..=MAX_X {
        for y_pos in MIN_Y..=MAX_Y {
            let (x_index, y_index) = universe.position_to_index(x_pos, y_pos);

            let old_state = universe.get_current_time_cell(x_index, y_index);

            let neighbors_count = universe.get_current_time_cell(x_index - 1, y_index - 1)
                + universe.get_current_time_cell(x_index - 1, y_index)
                + universe.get_current_time_cell(x_index - 1, y_index + 1)
                + universe.get_current_time_cell(x_index, y_index - 1)
                + universe.get_current_time_cell(x_index, y_index + 1)
                + universe.get_current_time_cell(x_index + 1, y_index - 1)
                + universe.get_current_time_cell(x_index + 1, y_index)
                + universe.get_current_time_cell(x_index + 1, y_index + 1);

            universe.set_cell_low_level(
                next_time,
                x_index,
                y_index,
                match old_state {
                    CELL_LIVE => match neighbors_count {
                        2 | 3 => CELL_LIVE,
                        _ => CELL_DEATH,
                    },
                    _ => match neighbors_count {
                        3 => CELL_LIVE,
                        _ => CELL_DEATH,
                    },
                },
            );
        }
    }

    match receiver.try_recv() {
        Err(_) => {},
        Ok(entity) => {
            inject_entropy(universe, next_time, entity);
        }
    }

    universe.set_current_time(next_time);
}

fn inject_entropy(universe: &mut Universe, next_time: isize, entity: UniversePlane) -> () {
    let (x_pos, y_pos) = (
        rand::thread_rng().gen_range(MIN_X..=MAX_X),
        rand::thread_rng().gen_range(MIN_Y..=MAX_Y),
    );

    let entity_xsize = entity.len() as isize;
    let entity_ysize = entity[0].len() as isize;

    universe.set_cell(next_time, x_pos, y_pos, -1, -1, CELL_DEATH);
    for i in 0..entity_xsize {
        universe.set_cell(next_time, x_pos, y_pos, i, -1, CELL_DEATH);
    }
    universe.set_cell(next_time, x_pos, y_pos, entity_xsize, -1, CELL_DEATH);

    for j in 0..entity_ysize {
        universe.set_cell(next_time, x_pos, y_pos, -1, j, CELL_DEATH);
        for i in 0..entity_xsize {
            universe.set_cell(
                next_time,
                x_pos,
                y_pos,
                i,
                j,
                entity[i as usize][j as usize],
            );
        }
        universe.set_cell(next_time, x_pos, y_pos, entity_xsize, j, CELL_DEATH);
    }

    universe.set_cell(next_time, x_pos, y_pos, -1, entity_ysize, CELL_DEATH);
    for i in 0..entity_xsize {
        universe.set_cell(next_time, x_pos, y_pos, i, entity_ysize, CELL_DEATH);
    }
    universe.set_cell(
        next_time,
        x_pos,
        y_pos,
        entity_xsize,
        entity_ysize,
        CELL_DEATH,
    );
}
