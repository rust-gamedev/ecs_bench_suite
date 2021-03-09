use planck_ecs::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            $world.initialize::<Components<$variants>>();
            (0..20)
            .for_each(|_| {
                let e = $world.get_mut::<Entities>().unwrap().create();
                $world.get_mut::<Components<_>>().unwrap().insert(e, $variants(0.0));
                $world.get_mut::<Components<_>>().unwrap().insert(e, Data(1.0));
            });
        )*
    };
}
struct Data(f32);

fn frag_iter_system(data_storage: &mut Components<Data>) -> SystemResult {
    for data in join!(&mut data_storage) {
        data.0 *= 2.0;
    }
    Ok(())
}

pub struct Benchmark(World, System);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.initialize::<Entities>();
        world.initialize::<Components<Data>>();
        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world, frag_iter_system.system())
    }

    pub fn run(&mut self) {
        self.1.run(&self.0).unwrap();
    }
}
