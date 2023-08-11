use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use image::ImageBuffer;
use imageproc::{drawing, rect::Rect};
use std::io::Write;
use std::sync::{Arc, RwLock};
use stopwatch::Stopwatch;

use crate::game_constants::{CELL_LIVE, CELL_SIZE, UniversePlane};
use crate::game_universe::{Universe, WorldBounds};

#[get("/gettext")]
pub async fn gettext(
    bounds: web::Query<WorldBounds>,
    rwlock_app: web::Data<Arc<RwLock<Universe>>>,
) -> impl Responder {
    let mut sw: Stopwatch = Stopwatch::start_new();

    let _current_time: isize;

    let world: UniversePlane;

    {
        let unlocked_data = match rwlock_app.read() {
            Err(error) => {
                return HttpResponse::InternalServerError().body(format!("{:?}", error));
            },
            Ok(data) => data
        };

        let universe = &*unlocked_data;

        _current_time = universe.get_current_time();

        world = universe.get_current_world(bounds.0);
    }

    let mut text_canvas = String::new();
    for world_line in world {
        for world_cell in world_line {
            match world_cell {
                CELL_LIVE => text_canvas.push('X'),
                _ => text_canvas.push(' '),
            }
        }
        text_canvas.push('\n');
    }

    sw.stop();

    println!("gettext() elapsed [{} ms]", sw.elapsed_ms());

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(text_canvas)
}

#[get("/getsvg")]
pub async fn getsvg(
    bounds: web::Query<WorldBounds>,
    rwlock_app: web::Data<Arc<RwLock<Universe>>>,
) -> impl Responder {
    let mut sw: Stopwatch = Stopwatch::start_new();

    let _current_time: isize;

    let world: UniversePlane;

    {
        let unlocked_data = match rwlock_app.read() {
            Err(error) => {
                return HttpResponse::InternalServerError().body(format!("{:?}", error));
            },
            Ok(data) => data
        };

        let universe = &*unlocked_data;

        _current_time = universe.get_current_time();

        world = universe.get_current_world(bounds.0);
    }

    let xsize = world.len();
    let ysize = world[0].len();

    let mut svgcontent = String::new();
    svgcontent.push_str("<?xml version='1.0' encoding='UTF-8'?>");
    svgcontent.push_str(
        "<!DOCTYPE svg PUBLIC '-//W3C//DTD SVG 1.1//EN' 'http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd'>"
    );
    svgcontent.push_str(
        format!(
            "<svg xmlns='http://www.w3.org/2000/svg' version='1.1' width='{}' height='{}'>",
            (xsize * CELL_SIZE) as u32,
            (ysize * CELL_SIZE) as u32
        )
        .as_str(),
    );
    svgcontent.push_str(
        format!(
            "  <rect x='0' y='0' width='{}' height='{}' fill='white' />",
            (xsize * CELL_SIZE) as u32,
            (ysize * CELL_SIZE) as u32
        )
        .as_str(),
    );

    for a in 0..xsize {
        for b in 0..ysize {
            match world[a][b] {
                CELL_LIVE => {
                    svgcontent.push_str(
                        format!(
                            "  <rect x='{}' y='{}' width='{}' height='{}' fill='black' />",
                            (a * CELL_SIZE) as i32,
                            (b * CELL_SIZE) as i32,
                            CELL_SIZE as u32,
                            CELL_SIZE as u32
                        )
                        .as_str(),
                    );
                }
                _ => {}
            }
        }
    }

    svgcontent.push_str("</svg>");

    sw.stop();

    println!("getsvg() elapsed [{} ms]", sw.elapsed_ms());

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svgcontent)
}

#[get("/getimage")]
pub async fn getimage(
    bounds: web::Query<WorldBounds>,
    rwlock_app: web::Data<Arc<RwLock<Universe>>>,
) -> impl Responder {
    let mut sw: Stopwatch = Stopwatch::start_new();

    let _current_time: isize;

    let world: UniversePlane;

    {
        let unlocked_data = match rwlock_app.read() {
            Err(error) => {
                return HttpResponse::InternalServerError().body(format!("{:?}", error));
            },
            Ok(data) => data
        };

        let universe = &*unlocked_data;

        _current_time = universe.get_current_time();

        world = universe.get_current_world(bounds.0);
    }

    let xsize = world.len();
    let ysize = world[0].len();

    let mut img = ImageBuffer::new((xsize * CELL_SIZE) as u32, (ysize * CELL_SIZE) as u32);

    drawing::draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size((xsize * CELL_SIZE) as u32, (ysize * CELL_SIZE) as u32),
        image::Rgb([255u8, 255u8, 255u8]),
    );

    for a in 0..xsize {
        for b in 0..ysize {
            match world[a][b] {
                CELL_LIVE => {
                    drawing::draw_filled_rect_mut(
                        &mut img,
                        Rect::at((a * CELL_SIZE) as i32, (b * CELL_SIZE) as i32)
                            .of_size(CELL_SIZE as u32, CELL_SIZE as u32),
                        image::Rgb([0u8, 0u8, 0u8]),
                    );
                }
                _ => {}
            }
        }
    }

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut writer = std::io::Cursor::new(&mut buffer);
        match img.write_to(&mut writer, image::ImageOutputFormat::Png) {
            Err(e) => {
                return HttpResponse::InternalServerError().body(format!("{:?}", e));
            },
            Ok(_) => {}
        }
        match writer.flush() {
            Err(e) => {
                return HttpResponse::InternalServerError().body(format!("{:?}", e));
            },
            Ok(_) => {}
        }
    }

    sw.stop();

    println!("getimage() elapsed [{} ms]", sw.elapsed_ms());

    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(buffer)
}
