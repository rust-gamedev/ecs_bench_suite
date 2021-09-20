use sparsey::prelude::*;

macro_rules! create_entities {
    ($world:ident; $($variant:ident),*) => {
        $(
            struct $variant(f32);
            $world.register::<$variant>();
            $world.create_entities((0..20).map(|_| ($variant(0.0), Data(1.0))));
        )*
    };
}

struct Data(f32);

pub struct Benchmark(World, ());

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<Data>();
        
        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world, ())
    }

    pub fn run(&mut self) {
        let mut data = self.0.borrow::<CompMut<Data>>();

        (&mut data).iter().for_each(|mut data| {
            data.0 *= 2.0;
        });
    }
}
