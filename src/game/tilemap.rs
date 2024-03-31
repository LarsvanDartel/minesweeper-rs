use bevy::prelude::*;
use rand::{seq::IteratorRandom as _, seq::SliceRandom as _, thread_rng};

#[cfg(feature = "debug")]
use colored::Colorize as _;

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub entity: Option<Entity>,
    pub cover: Option<Entity>,
    pub flag: Option<Entity>,
}

impl Tile {
    fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            entity: None,
            cover: None,
            flag: None,
        }
    }

    pub fn is_bomb(&self) -> bool {
        self.tile_type.is_bomb()
    }
}

/// A tilemap of the game board
pub struct TileMap {
    /// Size (rows, columns) of the tilemap
    size: UVec2,

    /// Number of bombs in the tilemap
    bomb_count: u32,

    /// Grid of tiles
    grid: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Create a new empty tilemap with the given size
    pub fn empty(size: UVec2) -> Self {
        Self {
            size,
            bomb_count: 0,
            grid: vec![vec![Tile::new(TileType::Empty); size.x as usize]; size.y as usize],
        }
    }

    /// Set the number of bombs in the tilemap and places them randomly
    pub fn set_bombs(&mut self, bomb_count: u32) {
        assert!(
            bomb_count <= self.size.x * self.size.y,
            "Bomb count exceeds grid size"
        );

        self.bomb_count = bomb_count;
        let mut rng = thread_rng();

        let mut positions = (0..self.size.x)
            .flat_map(|x| (0..self.size.y).map(move |y| (x, y)))
            .collect::<Vec<_>>();

        positions.shuffle(&mut rng);

        for pos in positions.into_iter().take(bomb_count as usize) {
            self.get_tile_mut(pos.into()).unwrap().tile_type = TileType::Bomb;
        }

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = UVec2::new(x, y);
                if self.get_tile(pos).unwrap().is_bomb() {
                    continue;
                }

                let count = self.bomb_count(pos);
                if count > 0 {
                    self.get_tile_mut(pos).unwrap().tile_type = TileType::Number(count);
                }
            }
        }
    }

    /// Returns the tile at the given position
    pub fn get_tile(&self, pos: UVec2) -> Option<&Tile> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
    }

    /// Returns the mutable tile at the given position
    pub fn get_tile_mut(&mut self, pos: UVec2) -> Option<&mut Tile> {
        self.grid
            .get_mut(pos.y as usize)
            .and_then(|row| row.get_mut(pos.x as usize))
    }

    /// Returns the neighboring tiles of the given position
    pub fn get_neighbors(&self, pos: UVec2) -> impl Iterator<Item = UVec2> + '_ {
        let x = pos.x as i32;
        let y = pos.y as i32;

        NEIGHBOR_OFFSETS.iter().filter_map(move |(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= self.size.x as i32 || ny < 0 || ny >= self.size.y as i32 {
                None
            } else {
                Some(UVec2::new(nx as u32, ny as u32))
            }
        })
    }

    /// Returns the bomb count at a given position
    pub fn bomb_count(&self, pos: UVec2) -> usize {
        self.get_neighbors(pos)
            .filter(|&pos| self.get_tile(pos).unwrap().is_bomb())
            .count()
    }

    /// Returns the size of the tilemap
    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// Finds a random empty tile in the tilemap
    pub fn find_empty_tile(&self) -> Option<UVec2> {
        let mut rng = thread_rng();

        (0..self.size.x)
            .flat_map(|x| (0..self.size.y).map(move |y| UVec2::new(x, y)))
            .filter(|pos| matches!(self.get_tile(*pos).unwrap().tile_type, TileType::Empty))
            .choose(&mut rng)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.grid.iter().flat_map(|row| row.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.grid.iter_mut().flat_map(|row| row.iter_mut())
    }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.grid {
            for tile in row {
                write!(f, "{:?} ", tile.tile_type)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Bomb,
    Number(usize),
}

impl TileType {
    pub fn is_bomb(&self) -> bool {
        matches!(self, TileType::Bomb)
    }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileType::Bomb => "*".bright_red(),
                TileType::Empty => "0".black(),
                TileType::Number(n) => match n {
                    1 => "1".bright_blue(),
                    2 => "2".bright_green(),
                    3 => "3".bright_red(),
                    4 => "4".blue(),
                    5 => "5".red(),
                    6 => "6".cyan(),
                    7 => "7".black(),
                    8 => "8".bright_black(),
                    _ => unreachable!(),
                },
            }
        )
    }
}
