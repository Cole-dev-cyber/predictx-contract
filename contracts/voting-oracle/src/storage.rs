use crate::DataKey;
use predictx_shared::VoteTally;
use soroban_sdk::Env;

// ── Vote tally storage ────────────────────────────────────────────────────────

/// Read the vote tally for a poll, if one has been stored yet.
///
/// Tally data lives in *temporary* storage: it is only needed during the
/// voting window (matching the tier guidance in the shared `DataKey` layout).
pub fn read_tally(env: &Env, poll_id: u64) -> Option<VoteTally> {
    env.storage().temporary().get(&DataKey::VoteTally(poll_id))
}

/// Store the vote tally for a poll.
pub fn write_tally(env: &Env, tally: &VoteTally) {
    env.storage()
        .temporary()
        .set(&DataKey::VoteTally(tally.poll_id), tally);
}
