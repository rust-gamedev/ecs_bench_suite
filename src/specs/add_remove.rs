use specs::prelude::*;

struct A(f32);
impl Component for A {
    type Storage = VecStorage<Self>;
}
struct B(f32);
impl Component for B {
    type Storage = VecStorage<Self>;
}

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<A>();
        let entities = (0..10000)
            .map(|_| world.create_entity().with(A(0.0)).build())
            .collect();
        world.register::<B>();
        Self(world, entities)
    }

    pub fn run(&mut self) {
        let mut b_storage = self.0.write_storage::<B>();
        for entity in &self.1 {
            b_storage.insert(*entity, B(0.0));
        }

        for entity in &self.1 {
            b_storage.remove(*entity);
        }
    }
}
