use specs::prelude::*;
use specs_derive::*;
#[derive(Component)]
#[storage(VecStorage)]
struct A(f32);
#[derive(Component)]
#[storage(VecStorage)]
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<A>();
        world.register::<B>();
        let entities = (0..10000)
            .map(|_| world.create_entity().with(A(0.0)).build())
            .collect();
        Self(world, entities)
    }

    pub fn run(&mut self) {
        let mut b_storage = self.0.write_storage::<B>();
        for entity in &self.1 {
            b_storage.insert(*entity, B(0.0)).unwrap();
        }

        for entity in &self.1 {
            b_storage.remove(*entity);
        }
    }
}
