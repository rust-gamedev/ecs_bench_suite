use brood::{entities, registry, World};
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Transform([f32; 16]);

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

type Registry = registry!(Transform, Position, Rotation, Velocity);

pub struct Benchmark(World<Registry>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.extend(entities!((Transform::default(), Position::default(), Rotation::default(), Velocity::default()); 1_000));

        Self(world)
    }

    pub fn run(&mut self) {
        let encoded = ron::to_string(&self.0).unwrap();
        ron::from_str::<World<Registry>>(&encoded).unwrap();
    }
}
