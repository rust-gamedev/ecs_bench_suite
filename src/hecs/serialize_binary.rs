use hecs::{serialize::column::*, *};
use serde::{de::SeqAccess, ser::SerializeTuple, Deserialize, Serialize};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Transform([f32; 16]);

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

struct SerContext;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum ComponentId {
    Transform,
    Position,
    Rotation,
    Velocity,
}

impl SerializeContext for SerContext {
    fn component_count(&self, archetype: &Archetype) -> usize {
        use std::any::TypeId;
        archetype
            .component_types()
            .filter(|&x| {
                x == TypeId::of::<Transform>()
                    || x == TypeId::of::<Position>()
                    || x == TypeId::of::<Rotation>()
                    || x == TypeId::of::<Velocity>()
            })
            .count()
    }

    fn serialize_component_ids<S: SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        out: &mut S,
    ) -> Result<(), S::Error> {
        if archetype.has::<Transform>() {
            out.serialize_element(&ComponentId::Transform)?;
        }
        if archetype.has::<Position>() {
            out.serialize_element(&ComponentId::Position)?;
        }
        if archetype.has::<Rotation>() {
            out.serialize_element(&ComponentId::Rotation)?;
        }
        if archetype.has::<Velocity>() {
            out.serialize_element(&ComponentId::Velocity)?;
        }
        Ok(())
    }

    fn serialize_components<S: SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        out: &mut S,
    ) -> Result<(), S::Error> {
        try_serialize::<Transform, _>(archetype, out)?;
        try_serialize::<Position, _>(archetype, out)?;
        try_serialize::<Rotation, _>(archetype, out)?;
        try_serialize::<Velocity, _>(archetype, out)?;
        Ok(())
    }
}

struct DeContext {
    components: Vec<ComponentId>,
}

impl DeserializeContext for DeContext {
    fn deserialize_component_ids<'de, A>(&mut self, mut seq: A) -> Result<ColumnBatchType, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.components.clear();
        let mut batch = ColumnBatchType::new();
        while let Some(id) = seq.next_element()? {
            match id {
                ComponentId::Transform => {
                    batch.add::<Transform>();
                }
                ComponentId::Position => {
                    batch.add::<Position>();
                }
                ComponentId::Rotation => {
                    batch.add::<Rotation>();
                }
                ComponentId::Velocity => {
                    batch.add::<Velocity>();
                }
            }
            self.components.push(id);
        }
        Ok(batch)
    }

    fn deserialize_components<'de, A>(
        &mut self,
        entity_count: u32,
        mut seq: A,
        batch: &mut ColumnBatchBuilder,
    ) -> Result<(), A::Error>
    where
        A: SeqAccess<'de>,
    {
        for &component in &self.components {
            match component {
                ComponentId::Transform => {
                    deserialize_column::<Transform, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Position => {
                    deserialize_column::<Position, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Rotation => {
                    deserialize_column::<Rotation, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Velocity => {
                    deserialize_column::<Velocity, _>(entity_count, &mut seq, batch)?;
                }
            }
        }
        Ok(())
    }
}

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn_batch((0..1000).map(|_| {
            (
                Transform::default(),
                Position::default(),
                Rotation::default(),
                Velocity::default(),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let Self(world) = self;
        let mut encoded = Vec::new();
        serialize(
            &world,
            &mut SerContext,
            &mut bincode::Serializer::new(&mut encoded, bincode::options()),
        )
        .unwrap();
        deserialize(
            &mut DeContext {
                components: Vec::new(),
            },
            &mut bincode::Deserializer::from_slice(&encoded, bincode::options()),
        )
        .unwrap();
    }
}
