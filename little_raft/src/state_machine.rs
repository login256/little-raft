use crate::message::LogEntry;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

/// TransitionState describes the state of a particular transition.
#[derive(Clone, Debug, PartialEq)]
pub enum TransitionState {
    /// Queued transitions have been received from the user but have not been
    /// processed yet. They are in the queue.
    ///
    Queued,

    /// Committed transitions have not yet been applied to the state machine but
    /// have already been replicated across the cluster such that they are
    /// guaranteed to be present in the log of all future cluster leaders.
    Committed,

    /// Applied transitions have been replicated across the cluster and have
    /// been applied to the local state machine.
    Applied,
}

/// StateMachineTransition describes a user-defined transition that can be
/// applied to the state machine replicated by Raft.
pub trait StateMachineTransition: Clone + Debug {
    /// TransitionID is used to identify the transition.
    type TransitionID: Eq;

    /// get_id is used by the Replica to identify the transition to be able to
    /// call register_transition_state.
    fn get_id(&self) -> Self::TransitionID;
}

/// StateMachine describes a user-defined state machine that is replicated
/// across the cluster. Raft can Replica whatever distributed state machine can
/// implement this trait.
#[async_trait]
pub trait StateMachine<T>
where
    T: StateMachineTransition,
{
    /// This is a hook that the local Replica will call each time the state of a
    /// particular transition changes. It is up to the user what to do with that
    /// information.
    async fn register_transition_state(&mut self, transition_id: T::TransitionID, state: TransitionState);

    /// When a particular transition is ready to be applied, the Replica will
    /// call apply_transition to apply said transition to the local state
    /// machine.
    async fn apply_transition(&mut self, transition: T);

    /// This function is used to receive transitions from the user that need to
    /// be applied to the replicated state machine. Note that only the Leader
    /// Replica processes transitions and only when notified via the
    /// recv_transition channel. All other Replicas poll for transitions and
    /// discard them. get_pending_transitions must not return the same
    /// transition twice.
    async fn get_pending_transitions(&mut self) -> Vec<T>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HardState
{
    pub cur_term: usize,
    pub cur_vote: Option<usize>,
}

pub trait Storage<T>
where
    T: StateMachineTransition,
{
    fn push_entry(&mut self, entry: LogEntry<T>);
    fn truncate_entries(&mut self, index: usize);
    fn store_term(&mut self, term: usize);
    fn store_vote(&mut self, vote: Option<usize>);
    fn get_term(&self) -> usize;
    fn get_vote(&self) -> Option<usize>;
    fn entries(&mut self, low: usize, high: usize) -> Vec<LogEntry<T>>;
    fn last_index(&self) -> usize;
    fn first_index(&self) -> usize;
    //fn snapshot()
}
