use shipyard::*;

#[derive(Component)]
struct A(f32);

#[derive(Component)]
struct B(f32);

pub struct Benchmark(World, Vec<EntityId>);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::default();

        let entities = world.run(|mut entities: EntitiesViewMut, mut a: ViewMut<A>| {
            let mut entity_ids = Vec::new();
            for _ in 0..10_000 {
                let entity = entities.add_entity(&mut a, A(0.0));
                entity_ids.push(entity);
            }
            entity_ids
        });

        Self(world, entities)
    }

    pub fn run(&mut self) {
        self.0.run(|entities: EntitiesViewMut, mut b: ViewMut<B>| {
            for entity in &self.1 {
                entities.add_component(*entity, &mut b, B(0.0));
            }
        });

        self.0.run(|mut b: ViewMut<B>| {
            for entity in &self.1 {
                b.remove(*entity);
            }
        });
    }
}
