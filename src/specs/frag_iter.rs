use specs::prelude::*;
use specs_derive::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            #[derive(Component)]
            #[storage(VecStorage)]
            struct $variants(f32);
            $world.register::<$variants>();
            (0..20)
            .for_each(|_| {$world.create_entity().with($variants(0.0)).with(Data(1.0)).build();});
        )*
    };
}
#[derive(Component)]
#[storage(VecStorage)]
struct Data(f32);

struct FragIterSystem;

impl<'a> System<'a> for FragIterSystem {
    type SystemData = WriteStorage<'a, Data>;

    fn run(&mut self, mut data_storage: Self::SystemData) {
        for data in (&mut data_storage).join() {
            data.0 *= 2.0;
        }
    }
}
pub struct Benchmark(World, FragIterSystem);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<Data>();
        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world, FragIterSystem)
    }

    pub fn run(&mut self) {
        self.1.run_now(&self.0)
    }
}
