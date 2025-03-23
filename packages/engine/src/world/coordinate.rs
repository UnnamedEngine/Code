use super::{ChunkPosition, CHUNK_SIZE};

pub type WorldUnit = i64;
pub type ChunkUnit = usize;
pub type IndexUnit = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coordinate {
    /// Coordinates for the entire world.
    World(WorldUnit, WorldUnit, WorldUnit),
    /// Coordinates inside of a chunk.
    Chunk(usize, usize, usize),
    /// Coordiantes inside of a chunk's index.
    Index(IndexUnit),
}

impl Coordinate {
    pub fn world_to_chunk(
        x: WorldUnit,
        y: WorldUnit,
        z: WorldUnit,
    ) -> (ChunkUnit, ChunkUnit, ChunkUnit) {
        let adjust = |value: WorldUnit| {
            let remainder = value % CHUNK_SIZE as WorldUnit;
            if remainder < 0 {
                (remainder + CHUNK_SIZE as WorldUnit) as ChunkUnit
            } else {
                remainder as ChunkUnit
            }
        };

        (adjust(x), adjust(y), adjust(z))
    }

    pub fn chunk_to_world(
        x: ChunkUnit,
        y: ChunkUnit,
        z: ChunkUnit,
        chunk_position: &ChunkPosition,
    ) -> (WorldUnit, WorldUnit, WorldUnit) {
        let x = (chunk_position.x * CHUNK_SIZE as i32) as WorldUnit
            + x as WorldUnit;
        let y = (chunk_position.y * CHUNK_SIZE as i32) as WorldUnit
            + y as WorldUnit;
        let z = (chunk_position.z * CHUNK_SIZE as i32) as WorldUnit
            + z as WorldUnit;

        (x, y, z)
    }

    pub fn chunk_to_index(x: ChunkUnit, y: ChunkUnit, z: ChunkUnit) -> usize {
        x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_SIZE)
    }

    pub fn index_to_chunk(index: usize) -> (ChunkUnit, ChunkUnit, ChunkUnit) {
        let x = index % CHUNK_SIZE;
        let y = (index / CHUNK_SIZE) % CHUNK_SIZE;
        let z = index / (CHUNK_SIZE * CHUNK_SIZE);
        (x, y, z)
    }

    pub fn to_world(&self, chunk_position: &ChunkPosition) -> Self {
        match self {
            Coordinate::World(_, _, _) => *self,
            Coordinate::Chunk(x, y, z) => {
                let world_coordinates =
                    Self::chunk_to_world(*x, *y, *z, chunk_position);
                Coordinate::World(
                    world_coordinates.0,
                    world_coordinates.1,
                    world_coordinates.2,
                )
            }
            Coordinate::Index(index) => {
                let chunk_coordinates = Self::index_to_chunk(*index);
                let world_coordinates = Self::chunk_to_world(
                    chunk_coordinates.0,
                    chunk_coordinates.1,
                    chunk_coordinates.2,
                    chunk_position,
                );
                Coordinate::World(
                    world_coordinates.0,
                    world_coordinates.1,
                    world_coordinates.2,
                )
            }
        }
    }

    pub fn to_chunk(&self) -> Self {
        match self {
            Coordinate::World(x, y, z) => {
                let chunk_coordinates = Self::world_to_chunk(*x, *y, *z);
                Coordinate::Chunk(
                    chunk_coordinates.0,
                    chunk_coordinates.1,
                    chunk_coordinates.2,
                )
            }
            Coordinate::Chunk(_, _, _) => *self,
            Coordinate::Index(index) => {
                let chunk_coordinates = Self::index_to_chunk(*index);
                Coordinate::Chunk(
                    chunk_coordinates.0,
                    chunk_coordinates.1,
                    chunk_coordinates.2,
                )
            }
        }
    }

    pub fn to_index(&self) -> Self {
        match self {
            Coordinate::World(x, y, z) => {
                let chunk_coordinates = Self::world_to_chunk(*x, *y, *z);
                let index_coordinates = Self::chunk_to_index(
                    chunk_coordinates.0,
                    chunk_coordinates.1,
                    chunk_coordinates.2,
                );
                Coordinate::Index(index_coordinates)
            }
            Coordinate::Chunk(x, y, z) => {
                let index_coordinates = Self::chunk_to_index(*x, *y, *z);
                Coordinate::Index(index_coordinates)
            }
            Coordinate::Index(_) => *self,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn world_to_chunk() {
        let world_coord = Coordinate::World(324, 6, -452);
        let chunk_coord = world_coord.to_chunk();

        assert_eq!(chunk_coord, Coordinate::Chunk(4, 6, 28));
    }

    #[test]
    fn world_to_index() {
        let world_coord = Coordinate::World(-324, 140, 9420);
        let index_coord = world_coord.to_index();

        assert_eq!(index_coord, Coordinate::Index(12700))
    }

    #[test]
    fn chunk_to_world() {
        let chunk_coord = Coordinate::Chunk(0, 12, 4);
        let chunk_position = ChunkPosition { x: 0, y: -2, z: 32 };
        let world_coord = chunk_coord.to_world(&chunk_position);

        assert_eq!(world_coord, Coordinate::World(0, -52, 1028));
    }

    #[test]
    fn chunk_to_index() {
        let chunk_coord = Coordinate::Chunk(4, 15, 31);
        let index_coord = chunk_coord.to_index();

        assert_eq!(index_coord, Coordinate::Index(32228));
    }

    #[test]
    fn index_to_world() {
        let index_coord = Coordinate::Index(19008);
        let chunk_position = ChunkPosition {
            x: -452,
            y: 17,
            z: -4,
        };
        let world_coord = index_coord.to_world(&chunk_position);

        assert_eq!(world_coord, Coordinate::World(-14464, 562, -110));
    }

    #[test]
    fn index_to_chunk() {
        let index_coord = Coordinate::Index(6473);
        let chunk_coord = index_coord.to_chunk();

        assert_eq!(chunk_coord, Coordinate::Chunk(9, 10, 6))
    }
}
