use planck_ecs::*;
struct A(f32);
struct B(f32);

pub struct Benchmark(Vec<Entity>, Components<A>, Components<B>);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = Entities::default();
        let mut comp1 = Components::<A>::default();
        let comp2 = Components::<B>::default();

        let entities = (0..10000)
            .map(|_| {
                let e = entities.create();
                comp1.insert(e, A(0.0));
                e
            })
            .collect();
        Self(entities, comp1, comp2)
    }

    pub fn run(&mut self) {
        let b_storage = &mut self.2;
        for entity in &self.0 {
            b_storage.insert(*entity, B(0.0));
        }

        for entity in &self.0 {
            b_storage.remove(*entity);
        }
    }
}
