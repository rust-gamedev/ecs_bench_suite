use shipyard::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            $world.run(
                | mut entities: EntitiesViewMut,
                mut data: ViewMut<Data>,
                mut variants: ViewMut<$variants> | {
                for _ in (0..20) {
                    entities.add_entity(
                        (&mut variants, &mut data),
                        ($variants(0.0), Data(1.0)),
                    );
                }
            });
        )*
    };
}

struct Data(f32);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::default();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.run(|mut data: ViewMut<Data>| {
            for data in (&mut data).iter() {
                data.0 *= 2.0;
            }
        });
    }
}
