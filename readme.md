# ECS Bench Suite

A suite of benchmarks designed to test and compare Rust ECS library performance across a variety of challenging circumstances.

The full benchmark report is available [here](https://tomgillen.github.io/ecs_bench_suite/target/criterion/report/index.html)

|                  | legion      | legion 0.2.4 | bevy     | shipyard   |
|------------------|:-----------:|:------------:|:--------:|:----------:|
| simple_insert    | **0.403ms** | 0.991ms      | 0.878ms  | 2.6247ms   |
| simple_iter      | **13.4us**  | 13.5us       | 14.1us   | 86.0us     |
| frag_iter        | **0.509us** | 1.80us       | 1.34us   | 1.05us     |
| heavy_compute    | **0.720ms** | 4.3426ms     | 4.6295ms | 0.727ms    |
| schedule         | **53.5us**  | 135us        | 95.4us   | 467us      |
| add_remove       | 5.55ms      | 3.08ms       | -        | **2.91ms** |
| serialize_text   | **16.9ms**  | -            | -        |            |
| serialize_binary | **6.59ms**  | -            | -        |            |

## The Benchmarks

### Simple Insert

This benchmark is designed to test the base cost of constructing entities and moving components into the ECS.

Inserts 10,000 entities, each with 4 components: `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)`.

### Simple Iter

This benchmark is designed to test the core overheads involved in component iteration in best-case conditions. The iteration should occur on a single CPU core.

Dataset: 10,000 entities, each with 4 components: `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)`.

Test: Iterate through all entities with `Position` and `Velocity`, and add velocity onto position.

### Fragmented Iter

This benchmark is designed to test how the ECS handles iteration through a fragmented dataset. The iteration should occur on a single CPU core.

Dataset: 26 component types (`A(f32)` through `Z(f32)`), each with 20 entities plus a `Data(f32)` component.

Test: Iterate through all entities with a `Data` component and double its value.

### System Scheduling

This benchmark is designed to test how efficiently the ECS can schedule multiple independent systems on a multi-core CPU. Each system should execute on a single CPU core.

Dataset:

* 10,000 entities with `(A, B)` components.
* 10,000 entities with `(A, B, C)` components.
* 10,000 entities with `(A, B, C, D)` components.
* 10,000 entities with `(A, B, C, E)` components.

Test:

Three systems accessing the following components mutably, where each system swaps the values stored in each component:

* `(A, B)`
* `(C, D)`
* `(C, E)`

### Heavy Compute

This benchmark is designed to test the ECS's ability to scale when it is allowed to run a system over multiple CPU cores.

Dataset: 10,000 entities with a `mat4x4` component.

Test: Iterate through all `mat4x4` components, and invert the matrix 10 times.

### Add/Remove Component

This benchmark is designed to test how quickly the ECS can add and then remove a component from an existing entity.

Dataset: 1,000 entities with a single `A` component.

Test: Iterate through all entities, adding a `B` component. Then iterate through all entities again, removing their `B` component.

### Serialize

This benchmark is designed to test how quickly the ECS and serialize and deserialize its entities in both text (RON) and binary (bincode) formats.

Dataset: 1000 entities with `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)` components.

Test: Serialize all entities to RON and bincode formats in-memory. Then deserialize back into the ECS. The RON and bincode formats should be separate benchmark tests.
