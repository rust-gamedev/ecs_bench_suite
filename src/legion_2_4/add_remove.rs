use legion_2_4::prelude::*;

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entities = world.insert((), (0..10000).map(|_| (A(0.0),))).to_vec();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.add_component(*entity, B(0.0)).unwrap();
        }

        for entity in &self.1 {
            self.0.remove_component::<B>(*entity).unwrap();
        }
    }
}
