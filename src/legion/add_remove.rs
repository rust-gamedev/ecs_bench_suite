use legion::*;

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entities = world.extend((0..10000).map(|_| (A(0.0),))).to_vec();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.entry(*entity).unwrap().add_component(B(0.0));
        }

        for entity in &self.1 {
            self.0.entry(*entity).unwrap().remove_component::<B>();
        }
    }
}
