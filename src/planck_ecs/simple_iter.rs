use cgmath::*;
use planck_ecs::*;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);
#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(Components<Velocity>, Components<Position>);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = Entities::default();
        let mut position_storage = Components::<Position>::default();
        let mut velocity_storage = Components::<Velocity>::default();
        (0..10000).for_each(|_| {
            let e = entities.create();
            position_storage.insert(e, Position(Vector3::unit_x()));
            velocity_storage.insert(e, Velocity(Vector3::unit_x()));
        });

        Self(velocity_storage, position_storage)
    }

    pub fn run(&mut self) {
        let velocity_storage = &mut self.0;
        let position_storage = &mut self.1;
        for (velocity, mut position) in join!(&mut velocity_storage && &mut position_storage) {
            position.as_mut().unwrap().0 += velocity.unwrap().0;
        }
    }
}
