use rand::prelude::*;
use std::{
    sync::{mpsc::Sender, Arc, RwLock},
    thread, time,
};
use stopwatch::Stopwatch;

use crate::game_constants::{ ENTROPY_LOOP_DELAY_MILLIS, UniversePlane };
use crate::game_species::species_nop;
use crate::game_universe::Universe;

pub fn entropy_loop(rwlock: Arc<RwLock<Universe>>, sender: &Sender<UniversePlane>) -> () {
    // Init
    let dimensions = Universe::get_dimensions();

    println!(
        "entropy_loop() {} {} {}",
        dimensions.0, dimensions.1, dimensions.2
    );

    let ten_millis = time::Duration::from_millis(ENTROPY_LOOP_DELAY_MILLIS);

    loop {
        let mut sw: Stopwatch = Stopwatch::start_new();

        {
            let unlocked_data = match rwlock.read() {
                Err(_) => {
                    continue;
                },
                Ok(data) => data
            };

            let universe = &*unlocked_data;

            let entity = get_random_entity(universe);

            match sender.send(entity) {
                Err(e) => {
                    eprintln!("{:?}", e);
                },
                Ok(_) => {}
            }
        }

        sw.stop();

        println!("entropy_loop() elapsed [{} ms]", sw.elapsed_ms());

        thread::sleep(ten_millis);
    }
}

pub fn get_random_entity(universe: &Universe) -> UniversePlane {
    let species = universe.get_entities();

    let i = rand::thread_rng().gen_range(0..species.len());

    let entity = species_nop(&species[i]);

    entity
}
