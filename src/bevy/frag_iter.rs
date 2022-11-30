use bevy_ecs::prelude::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            #[derive(bevy_ecs::component::Component)]
            struct $variants(f32);

            $world.spawn_batch((0..20).map(|_| ($variants(0.0), Data(1.0))));
        )*
    };
}

#[derive(bevy_ecs::component::Component)]
struct Data(f32);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world)
    }

    pub fn run(&mut self) {
        let mut query = self.0.query::<&mut Data>();

        for mut data in query.iter_mut(&mut self.0) {
            data.0 *= 2.0;
        }
    }
}
