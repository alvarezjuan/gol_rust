pub type UniverseCell = u8;
pub type UniversePlane = Vec<Vec<UniverseCell>>;
pub type UniversePlaneSet = Vec<UniversePlane>;

pub const CELL_DEATH: UniverseCell = 0;
pub const CELL_LIVE: UniverseCell = 1;

pub const MIN_X: isize = 0;
pub const MAX_X: isize = 1023;
pub const MIN_Y: isize = 0;
pub const MAX_Y: isize = 1023;
pub const HISTORY_SIZE: isize = 100;

pub const WORLD_SIZE_X: isize = MAX_X - MIN_X + 1;
pub const WORLD_SIZE_Y: isize = MAX_Y - MIN_Y + 1;

pub const CELL_SIZE: usize = 4;

pub const ENGINE_LOOP_DELAY_MILLIS: u64 = 100;
pub const ENTROPY_LOOP_DELAY_MILLIS: u64 = 10000;

pub const API_ADDRESS: &str = "127.0.0.1";
pub const API_PORT: u16 = 8080;
