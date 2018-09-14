extern crate euclid;
use self::euclid::{Point2D};

mod map;

/// Moonphases can be found on https://terraria.gamepedia.com/Moon_phase
#[repr(u8)]
enum MoonPhase{
    FullMoon = 1,
    WaningGibbous,
    ThirdQuarter,
    WaningCrescent,
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous
}

/// Holds the basic Information about the currently loaded map
struct WorldInfo{
    game_time: u32,
    is_day: bool,
    moon_phase: MoonPhase,
    blood_moon: bool,
    eclipse: bool,
    world_width: u32,
    world_height: u32,
    spawn_tile: Point2D<u64>,

}