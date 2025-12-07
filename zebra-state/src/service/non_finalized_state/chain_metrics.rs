//! Metrics for non-finalized state chain tracking.
//!
//! These metrics help track chain forks, stale blocks, and reorgs
//! to calculate stale block rate (similar to uncle rate in Ethereum).

use zebra_chain::block;

/// Track a chain dropped due to MAX_NON_FINALIZED_CHAIN_FORKS limit.
///
/// This is a resource protection mechanism, not consensus-related.
/// When too many concurrent forks exist, the lowest-work chains are dropped.
pub(crate) fn chain_fork_limit_dropped(chain_length: u64) {
    metrics::counter!("state.non_finalized.fork_limit.chains_dropped").increment(1);
    metrics::counter!("state.non_finalized.fork_limit.blocks_dropped").increment(chain_length);
    metrics::histogram!("state.non_finalized.fork_limit.chain_length").record(chain_length as f64);

    tracing::debug!(
        chain_length,
        "chain dropped due to MAX_NON_FINALIZED_CHAIN_FORKS limit"
    );
}

/// Track a stale side chain that was dropped during finalization.
///
/// These are valid chains that lost the consensus race to the best chain.
/// This is the primary metric for calculating stale block rate.
pub(crate) fn stale_chain_dropped(chain_length: u64) {
    metrics::counter!("state.non_finalized.consensus.stale_chains").increment(1);
    metrics::counter!("state.non_finalized.consensus.stale_blocks").increment(chain_length);
    metrics::histogram!("state.non_finalized.consensus.stale_chain_length").record(chain_length as f64);

    tracing::debug!(
        chain_length,
        "stale side chain dropped during finalization"
    );
}

/// Track when a block is successfully committed to non-finalized state.
///
/// This includes blocks on all forks (best chain and side chains).
/// Used to calculate total blocks seen vs blocks finalized.
pub(crate) fn block_committed() {
    metrics::counter!("state.non_finalized.blocks.committed").increment(1);
}

/// Track when a reorg occurs (best chain changes).
///
/// Records both the reorg event and the depth (how far back the fork point was).
pub(crate) fn reorg_detected(depth: u32, blocks_replaced: u64) {
    metrics::counter!("state.non_finalized.reorgs.count").increment(1);
    metrics::counter!("state.non_finalized.reorgs.blocks_replaced").increment(blocks_replaced);
    metrics::histogram!("state.non_finalized.reorgs.depth").record(depth as f64);

    tracing::info!(
        depth,
        blocks_replaced,
        "blockchain reorganization detected"
    );
}

/// Track when competing blocks at the same height are detected.
///
/// This indicates a fork is occurring in real-time.
pub(crate) fn fork_detected(height: block::Height, fork_depth: u32) {
    metrics::counter!("state.non_finalized.forks.detected").increment(1);
    metrics::histogram!("state.non_finalized.forks.depth").record(fork_depth as f64);

    tracing::debug!(
        ?height,
        fork_depth,
        "competing blocks detected at same height"
    );
}

/// Update the gauge tracking current number of active chain forks.
pub(crate) fn update_active_forks(count: usize) {
    metrics::gauge!("state.non_finalized.forks.active").set(count as f64);
}
