use legion::*;
use query::Query;
use storage::PackOptions;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            $world.extend((0..20).map(|_| ($variants(0.0), Data(1.0))));
        )*
    };
}

struct Data(f32);

pub struct Benchmark(World, Query<Write<Data>>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
        world.pack(PackOptions::force());

        let query = Write::<Data>::query();

        Self(world, query)
    }

    pub fn run(&mut self) {
        self.1.for_each_mut(&mut self.0, |data| {
            data.0 *= 2.0;
        });
    }
}
