use brood::{
    entities,
    query::{filter, result, views},
    registry, World,
};

macro_rules! define_entities {
    ($($component:ident),*) => {
        $(
            #[derive(Clone)]
            struct $component(f32);
        )*
    }
}

macro_rules! create_entities {
    ($world:ident; $($component:ident),*) => {
        $(
            $world.extend(entities!(($component(0.0), Data(1.0)); 20));
        )*
    };
}

#[derive(Clone)]
struct Data(f32);
define_entities!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

type Registry =
    registry!(Data, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

pub struct Benchmark(World<Registry>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world)
    }

    pub fn run(&mut self) {
        for result!(data) in self.0.query::<views!(&mut Data), filter::None>() {
            data.0 *= 2.0;
        }
    }
}
