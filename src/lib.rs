#![doc = include_str!("../README.md")]


use bevy_ecs::{
    component::Tick,
    message::{ Message, Messages },
    query::FilteredAccessSet,
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


/// An alternative to [`MessageWriter`](bevy_ecs::message::MessageWriter) that can be used in parallel
///  contexts, such as those in [`Query::par_iter`](bevy_ecs::system::Query::par_iter).
///
/// ### Note
/// Because send application order will depend on how many threads are ran, non-commutative sends
///  may result in non-deterministic results.
///
/// ### Example
/// ```rust
/// fn parallel_message_system(
///     mut query : Query<(Entity, &Velocity)>,
///     par_writer : ParallelMessageWriter<Supersonic>
/// ) {
///     query.par_iter().for_each(|(entity, velocity)| {
///         if velocity.magnitude() > 343.2 {
///             par_writer.write(Supersonic { entity });
///         }
///     });
/// }
/// ```
pub struct ParallelMessageWriter<'state, E>
where
    E : Message
{
    messages : &'state Parallel<Messages<E>>
}

impl<E> ParallelMessageWriter<'_, E>
where
    E : Message
{

    /// Writes an `message`, which can later be read by [`MessageReader`](bevy_ecs::message::MessageReader)s.
    ///  Unlike [`MessageWriter::write`](bevy_ecs::message::MessageWriter::write), this method does not
    ///  return the [ID](bevy_ecs::message::MessageId) of the written `message`.
    ///
    /// See [`Messages`] for details.
    #[inline]
    pub fn write(&self, message : E) {
        _ = self.messages.scope(|e| e.send(message));
    }

    /// Sends a list of `message`s all at once, which can later be read by
    ///  [`MessageReader`](bevy_ecs::message::MessageReader)s. This is more efficient than sending each message
    ///  individually. Unlike [`MessageWriter::write_batch`](bevy_ecs::message::MessageWriter::write_batch),
    ///  this method does not return the [IDs](bevy_ecs::message::MessageId) of the written `message`s.
    ///
    /// See [`Messages`] for details.
    #[inline]
    pub fn write_batch(&self, messages : impl IntoIterator<Item = E>) {
        _ = self.messages.scope(|e| e.send_batch(messages));
    }

    /// Writes the default value of the `message`. Useful when the message is an empty struct. Unlike
    ///  Unlike [`MessageWriter::write_default`](bevy_ecs::message::MessageWriter::write_default), this method
    ///  does not return the [IDs](bevy_ecs::message::MessageId) of the written `message`s.
    ///
    /// See [`Messages`] for details.
    #[inline]
    pub fn write_default(&self)
    where
        E : Default
    { _ = self.messages.scope(|e| e.send_default()); }

}


unsafe impl<E> SystemParam for ParallelMessageWriter<'_, E>
where
    E : Message
{
    type State                = Parallel<Messages<E>>;
    type Item<'world, 'state> = ParallelMessageWriter<'state, E>;

    #[inline]
    fn init_state(
        _ : &mut World
    ) -> Self::State {
        Parallel::default()
    }

    #[inline]
    fn init_access(
        state                : &Self::State,
        system_meta          : &mut SystemMeta,
        component_access_set : &mut FilteredAccessSet,
        world                : &mut World
    ) { }

    #[inline]
    unsafe fn get_param<'world, 'state>(
        state : &'state mut Self::State,
        _     : &SystemMeta,
        _     : UnsafeWorldCell<'world>,
        _     : Tick,
    ) -> Self::Item<'world, 'state> {
        ParallelMessageWriter {
            messages : state
        }
    }

    fn apply(
        state : &mut Self::State,
        _     : &SystemMeta,
        world : &mut World
    ) {
        world.write_message_batch(state.iter_mut().flat_map(|e| e.update_drain()));
    }

}
