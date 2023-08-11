use glob::glob;
use regex::{RegexBuilder, RegexSetBuilder};
use std::fs::read_to_string;

use crate::game_constants::{CELL_DEATH, CELL_LIVE, UniversePlane, UniversePlaneSet};
use crate::game_universe::Universe;

fn process_lif(text: &str) -> String {
    const PATTERN_COMMENT: &'static str = r"^#.*$";
    const PATTERN_SIZE: &'static str = r"^\s*x\s*=\s*(?P<X>\d*)\s*,\s*y\s*=\s*(?P<Y>\d*).*$";
    const PATTERN_RLE: &'static str = r"^(\d*[bo\$!])*$";
    const PATTERN_RLE2: &'static str = r"((?P<rle1>\d*)(?P<rle2>[bo\$!]))";

    let mut buffer = String::new();

    match RegexSetBuilder::new(&[PATTERN_COMMENT, PATTERN_SIZE, PATTERN_RLE])
        .multi_line(true)
        .build()
    {
        Err(_) => unreachable!(),
        Ok(set) => {
            text.lines()
                .filter(|line| set.is_match(line))
                .for_each(|line| {
                    for i in set.matches(line) {
                        match i {
                            0 => {
                                // println!("Comentario : {}", line)
                            }
                            1 => {
                                // println!("Tamanio : {}", line);

                                // match RegexBuilder::new(PATTERN_SIZE).build() {
                                //     Err(_) => unreachable!(),
                                //     Ok(regex_rle) => {
                                //         for c in regex_rle.captures_iter(line) {
                                //             let xxx = match (&c["X"]).parse::<u16>() {
                                //                 Err(_) => unreachable!(),
                                //                 Ok(vx) => vx,
                                //             };
                                //             let yyy = match (&c["Y"]).parse::<u16>() {
                                //                 Err(_) => unreachable!(),
                                //                 Ok(vy) => vy,
                                //             };
                                //             // println!("X {} Y {}", xxx, yyy);
                                //         }
                                //     }
                                // }
                            }
                            2 => {
                                // println!("Contenido : {}", line);

                                match RegexBuilder::new(PATTERN_RLE2).build() {
                                    Err(_) => unreachable!(),
                                    Ok(regex_rle) => {
                                        for c in regex_rle.captures_iter(line) {
                                            let nseg = match (&c["rle1"]).parse::<u16>() {
                                                Err(_) => 1,
                                                Ok(vseg) => vseg,
                                            };
                                            let cseg = match &c["rle2"] {
                                                "b" => " ",
                                                "o" => "X",
                                                "$" => "\n",
                                                "!" => return,
                                                _ => unreachable!(),
                                            };
                                            buffer.push_str(
                                                ((0..nseg).map(|_| cseg).collect::<String>())
                                                    .as_str(),
                                            );
                                        }
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                });
        }
    }

    buffer
}

pub fn load_plaintext_species(universe: &mut Universe) -> () {
    match glob("**/*.cells") {
        Err(e) => {
            eprintln!("{:?}", e);
        },
        Ok(paths) => {
            for p in paths {
                match p {
                    Err(e) => {
                        eprintln!("{:?}", e);
                    },
                    Ok(path) => {
                        println!("Loading species: {}", path.display());

                        match read_to_string(path) {
                            Err(e) => {
                                eprintln!("{:?}", e);
                            },
                            Ok(content) => {
                                let entity_base = species_plaintext_to_vec(content.as_str());

                                let entities = species_from_base(&entity_base);

                                for entity in entities {
                                    universe.push_entity(entity);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn load_rle_species(universe: &mut Universe) -> () {
    match glob("**/*.lif") {
        Err(e) => {
            eprintln!("{:?}", e);
        },
        Ok(paths) => {
            for p in paths {
                match p {
                    Err(e) => {
                        eprintln!("{:?}", e);
                    },
                    Ok(path) => {
                        println!("Loading species: {}", path.display());

                        match read_to_string(path) {
                            Err(e) => {
                                eprintln!("{:?}", e);
                            },
                            Ok(content) => {
                                let raw_content = process_lif(content.as_str());

                                let entity_base = species_plaintext_to_vec(raw_content.as_str());

                                let entities = species_from_base(&entity_base);

                                for entity in entities {
                                    universe.push_entity(entity);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn species_from_base(entity: &UniversePlane) -> UniversePlaneSet {
    let mut entities = Vec::new();

    entities.push(species_nop(&entity));

    entities.push(species_rotate_90(&entity));

    let entity1 = species_rotate_90(&entity);
    entities.push(species_rotate_90(&entity1));

    let entity1 = species_rotate_90(&entity);
    let entity1 = species_rotate_90(&entity1);
    entities.push(species_rotate_90(&entity1));

    entities.push(species_flip_h(&entity));

    entities.push(species_flip_v(&entity));

    entities
}

fn species_plaintext_to_vec(text: &str) -> UniversePlane {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for line in text.lines() {
        if line.trim().starts_with('!') {
            continue;
        }
        let line_len = line.trim().len();
        max_x = if line_len > max_x { line_len } else { max_x };
        max_y += 1;
    }

    let mut entity: UniversePlane = vec![vec![CELL_DEATH; max_y as usize]; max_x as usize];

    let mut char_y: usize = 0;

    for line in text.lines() {
        if line.trim().starts_with('!') {
            continue;
        }
        let mut char_x: usize = 0;
        for c in line.trim().chars() {
            entity[char_x][char_y] = match c {
                '.' => CELL_DEATH,
                _ => CELL_LIVE,
            };
            char_x += 1;
        }
        char_y += 1;
    }

    entity
}

fn species_rotate_90(entity_source: &UniversePlane) -> UniversePlane {
    let source_max_x: usize = entity_source.len();
    let source_max_y: usize = entity_source[0].len();

    let mut entity: UniversePlane =
        vec![vec![CELL_DEATH; source_max_x as usize]; source_max_y as usize];

    for a in 0..source_max_x {
        for b in 0..source_max_y {
            entity[b][source_max_x - a - 1] = entity_source[a][b];
        }
    }

    entity
}

fn species_flip_h(entity_source: &UniversePlane) -> UniversePlane {
    let source_max_x: usize = entity_source.len();
    let source_max_y: usize = entity_source[0].len();

    let mut entity: UniversePlane =
        vec![vec![CELL_DEATH; source_max_y as usize]; source_max_x as usize];

    for a in 0..source_max_x {
        for b in 0..source_max_y {
            entity[source_max_x - a - 1][b] = entity_source[a][b];
        }
    }

    entity
}

fn species_flip_v(entity_source: &UniversePlane) -> UniversePlane {
    let source_max_x: usize = entity_source.len();
    let source_max_y: usize = entity_source[0].len();

    let mut entity: UniversePlane =
        vec![vec![CELL_DEATH; source_max_y as usize]; source_max_x as usize];

    for a in 0..source_max_x {
        for b in 0..source_max_y {
            entity[a][source_max_y - b - 1] = entity_source[a][b];
        }
    }

    entity
}

pub fn species_nop(entity_source: &UniversePlane) -> UniversePlane {
    let source_max_x: usize = entity_source.len();
    let source_max_y: usize = entity_source[0].len();

    let mut entity: UniversePlane =
        vec![vec![CELL_DEATH; source_max_y as usize]; source_max_x as usize];

    for a in 0..source_max_x {
        for b in 0..source_max_y {
            entity[a][b] = entity_source[a][b];
        }
    }

    entity
}
