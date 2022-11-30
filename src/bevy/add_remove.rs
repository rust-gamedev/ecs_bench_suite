use bevy_ecs::prelude::*;

#[derive(bevy_ecs::component::Component)]
struct A(f32);

#[derive(bevy_ecs::component::Component)]
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entities = world
            .spawn_batch((0..10000).map(|_| (A(0.0),)))
            .collect::<Vec<_>>();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.entity_mut(*entity).insert(B(0.0));
        }

        for entity in &self.1 {
            self.0.entity_mut(*entity).remove::<B>();
        }
    }
}
