# `bevy_pareventwriter`
A simple library for [Bevy](https://github.com/bevyngine/bevy) to write events in parallel.


### Why?
Bevy has the [`EventWriter<T>`](https://docs.rs/bevy_ecs/latest/bevy_ecs/event/struct.EventWriter.html)
 system parameter. However, if you are parallel iterating through something such as a query
 ([`Query::par_iter`](https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.Query.html#method.par_iter)),
 `EventWriter`s are unusable.

[`ParallelEventWriter`](https://docs.rs/bevy_pareventwriter/struct.ParallelEventWriter.html) keeps
 thread-local event queues, which are merged and sent after the system finishes.


### Example
Here is a crude example of where this might be useful.
```rust
fn parallel_event_system(
    mut query : Query<(Entity, &Velocity)>,
    par_writer : ParallelEventWriter<Supersonic>
) {
    query.par_iter().for_each(|(entity, velocity)| {
        if velocity.magnitude() > 343.2 {
            par_writer.write(Supersonic { entity });
        }
    });
}
```
`ParallelEventWriter` can be used to write events from inside of the parallel iteration.
