extern crate euclid;

use self::euclid::{Point2D};

use std::io::{Read, Write};

///Representation of a tile
pub struct Tile{
    
}
/// Representation of the worldmap
pub struct Map{

    /// If I ever create a custom world gen, this will be necessary
    original_world: bool,

    /// List of all tiles, can be indexed like tiles[x][y]
    /// tiles[0][0] is the bottomleftmost tile
    tiles: Vec<Vec<Tile>>,

    /// Position of the Spawn
    spawn_point: Point2D<u64>,

    /// Maximum height, that should be generated
    /// Only relevant in non-original maps
    max_height: u64,

    /// Maximum width, that should be generated
    /// Only relevant in non-original maps
    max_width: u64,
}


//TODO: Implement world loading/saving from/to a file

/*
impl Read for Map{

}

impl Write for Map{

}
*/