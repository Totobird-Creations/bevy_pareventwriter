#![doc = include_str!("../README.md")]


use bevy_ecs::{
    component::Tick,
    event::{ Event, Events },
    system::{
        SystemParam,
        SystemMeta
    },
    world::{
        World,
        unsafe_world_cell::UnsafeWorldCell
    }
};
use bevy_utils::Parallel;


/// An alternative to [`EventWriter`](bevy_ecs::event::EventWriter) that can be used in parallel
///  contexts, such as those in [`Query::par_iter`](bevy_ecs::system::Query::par_iter).
///
/// ### Note
/// Because send application order will depend on how many threads are ran, non-commutative sends
///  may result in non-deterministic results.
///
/// ### Example
/// ```rust
/// fn parallel_event_system(
///     mut query : Query<(Entity, &Velocity)>,
///     par_writer : ParallelEventWriter<Supersonic>
/// ) {
///     query.par_iter().for_each(|(entity, velocity)| {
///         if velocity.magnitude() > 343.2 {
///             par_writer.write(Supersonic { entity });
///         }
///     });
/// }
/// ```
pub struct ParallelEventWriter<'state, E>
where
    E : Event
{
    events : &'state Parallel<Events<E>>
}

impl<E> ParallelEventWriter<'_, E>
where
    E : Event
{

    /// Writes an `event`, which can later be read by [`EventReader`](bevy_ecs::event::EventReader)s.
    ///  Unlike [`EventWriter::write`](bevy_ecs::event::EventWriter::write), this method does not
    ///  return the [ID](bevy_ecs::event::EventId) of the written `event`.
    ///
    /// See [`Events`] for details.
    #[inline]
    pub fn write(&self, event : E) {
        _ = self.events.scope(|e| e.send(event));
    }

    /// Sends a list of `event`s all at once, which can later be read by
    ///  [`EventReader`](bevy_ecs::event::EventReader)s. This is more efficient than sending each event
    ///  individually. Unlike [`EventWriter::write_batch`](bevy_ecs::event::EventWriter::write_batch),
    ///  this method does not return the [IDs](bevy_ecs::event::EventId) of the written `event`s.
    ///
    /// See [`Events`] for details.
    #[inline]
    pub fn write_batch(&self, events : impl IntoIterator<Item = E>) {
        _ = self.events.scope(|e| e.send_batch(events));
    }

    /// Writes the default value of the `event`. Useful when the event is an empty struct. Unlike
    ///  Unlike [`EventWriter::write_default`](bevy_ecs::event::EventWriter::write_default), this method
    ///  does not return the [IDs](bevy_ecs::event::EventId) of the written `event`s.
    ///
    /// See [`Events`] for details.
    #[inline]
    pub fn write_default(&self)
    where
        E : Default
    { _ = self.events.scope(|e| e.send_default()); }

}


unsafe impl<E> SystemParam for ParallelEventWriter<'_, E>
where
    E : Event
{
    type State                = Parallel<Events<E>>;
    type Item<'world, 'state> = ParallelEventWriter<'state, E>;

    #[inline]
    fn init_state(
        _ : &mut World,
        _ : &mut SystemMeta
    ) -> Self::State {
        Parallel::default()
    }

    #[inline]
    unsafe fn get_param<'world, 'state>(
        state : &'state mut Self::State,
        _     : &SystemMeta,
        _     : UnsafeWorldCell<'world>,
        _     : Tick,
    ) -> Self::Item<'world, 'state> {
        ParallelEventWriter {
            events : state
        }
    }

    fn apply(
        state : &mut Self::State,
        _     : &SystemMeta,
        world : &mut World
    ) {
        world.send_event_batch(state.iter_mut().flat_map(|e| e.update_drain()));
    }

}
