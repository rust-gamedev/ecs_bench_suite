use edict::prelude::*;

macro_rules! create_entities {
    ($world:ident; $entities:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            $entities.extend($world.spawn_batch((0..20).map(|_| ($variants(0.0), Data(1.0)))));
        )*
    };
}

struct Data(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut entities = Vec::new();

        create_entities!(world; entities; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world, entities)
    }

    pub fn run(&mut self) {
        self.0.for_each_mut::<&mut Data, _>(|data| data.0 *= 2.0);
    }
}
