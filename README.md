# `bevy_parmessagewriter`
A simple library for [Bevy](https://github.com/bevyngine/bevy) to write messages in parallel.


### Why?
Bevy has the [`MessageWriter<T>`](https://docs.rs/bevy_ecs/latest/bevy_ecs/message/struct.MessageWriter.html)
 system parameter. However, if you are parallel iterating through something such as a query
 ([`Query::par_iter`](https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.Query.html#method.par_iter)),
 `MessageWriter`s are unusable.

[`ParallelMessageWriter`](https://docs.rs/bevy_parmessagewriter/struct.ParallelMessageWriter.html) keeps
 thread-local message queues, which are merged and sent after the system finishes.


### Example
Here is a crude example of where this might be useful.
```rust
fn parallel_message_system(
    mut query : Query<(Entity, &Velocity)>,
    par_writer : ParallelMessageWriter<Supersonic>
) {
    query.par_iter().for_each(|(entity, velocity)| {
        if velocity.magnitude() > 343.2 {
            par_writer.write(Supersonic { entity });
        }
    });
}
```
`ParallelMessageWriter` can be used to write messages from inside of the parallel iteration.
