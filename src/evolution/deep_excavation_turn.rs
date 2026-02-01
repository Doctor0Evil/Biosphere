#![forbid(unsafe_code)]

use crate::deep_object_router::{
    BiophysicalLayer,
    DeepObjectRouter,
    DeepRoutingDecision,
    EpochInventory,
    LifeforceBand,
    LifeforceState,
};
use crate::governance::deep_domain_rights::{
    DeepDomainRightsProfile,
};
use crate::telemetry::brain_token_ledger::BrainTokenWindow;
use crate::telemetry::dw_token_ledger::DraculaWaveWindow;

/// Minimal view of an evolution-turn context.
#[derive(Clone, Debug)]
pub struct EvolutionTurnContext {
    pub query_id: String,
    pub intent_kind: String,
    pub intent_tag: Option<String>,
    pub host_role: String,
    pub irreversible_token_present: bool,
}

/// Result of a governed deep-excavation selection.
#[derive(Clone, Debug)]
pub struct GovernedExcavationOutcome {
    pub selected_epochs: Vec<(usize, DeepRoutingDecision)>,
    pub skipped_epochs: usize,
    pub reason_summary: String,
}

/// Main entry point: given epochs, lifeforce, and rights, choose which
/// epochs may be used for deep-object excavation in this evolution turn.
pub fn governed_select_deep_epochs(
    turn_ctx: &EvolutionTurnContext,
    rights: &DeepDomainRightsProfile,
    router: &DeepObjectRouter,
    epochs: &[EpochInventory],
    lf: &LifeforceState,
    brain_window: &BrainTokenWindow,
    dw_window: &DraculaWaveWindow,
    daily_layer_usage: &std::collections::BTreeMap<BiophysicalLayer, u32>,
) -> GovernedExcavationOutcome {
    use BiophysicalLayer::*;

    // 0. Enforce host role (must be augmented-citizen as per rights).
    if !rights.allowed_roles.contains(&turn_ctx.host_role) {
        return GovernedExcavationOutcome {
            selected_epochs: Vec::new(),
            skipped_epochs: epochs.len(),
            reason_summary: format!(
                "Denied: host role '{}' not authorized for deep-domain excavation",
                turn_ctx.host_role
            ),
        };
    }

    // 1. Convert lifeforce to snapshot type.
    let lf_snapshot = crate::lifeforce_to_snapshot(lf);

    // 2. Compute DeepRoutingDecision for each epoch.
    let mut scored: Vec<(usize, DeepRoutingDecision, &EpochInventory)> = epochs
        .iter()
        .enumerate()
        .map(|(idx, e)| {
            let decision = router.decide(e, lf);
            (idx, decision, e)
        })
        .collect();

    // 3. Sort by excavation_weight01 descending (rarest, safest, deepest first).
    scored.sort_by(|a, b| {
        b.1
            .excavation_weight01
            .partial_cmp(&a.1.excavation_weight01)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // 4. Iterate and keep only those allowed by rights + budgets per layer.
    let mut selected = Vec::new();
    let mut skipped = 0usize;
    let mut local_usage = daily_layer_usage.clone();

    for (idx, decision, epoch) in scored.into_iter() {
        if !decision.eligible_for_excavation {
            skipped += 1;
            continue;
        }

        let layer = decision.layer;
        if matches!(layer, Excluded | B1Surface) {
            skipped += 1;
            continue;
        }

        let used_so_far = *local_usage.get(&layer).unwrap_or(&0u32);

        let brain_avail = brain_window.available_capacity();
        let dw_avail = dw_window.available_capacity();
        let eco_nj = epoch.ecoenergynj;

        let allowed = rights.can_excavate_epoch(
            layer,
            &lf_snapshot,
            eco_nj,
            brain_avail,
            dw_avail,
            used_so_far,
            turn_ctx.irreversible_token_present,
            turn_ctx.intent_tag.as_deref(),
        );

        if !allowed {
            skipped += 1;
            continue;
        }

        // Update local usage and token windows (spend is handled upstream).
        *local_usage.entry(layer).or_insert(0) += 1;
        selected.push((idx, decision));
    }

    let reason = if selected.is_empty() {
        format!(
            "No epochs selected for deep excavation; {} were skipped by rights/lifeforce/eco gates",
            skipped
        )
    } else {
        format!(
            "Selected {} epochs for governed deep excavation; {} skipped",
            selected.len(),
            skipped
        )
    };

    GovernedExcavationOutcome {
        selected_epochs: selected,
        skipped_epochs: skipped,
        reason_summary: reason,
    }
}

/// Helper to map LifeforceState into LifeforceSnapshot for rights checks.
fn lifeforce_to_snapshot(lf: &LifeforceState) -> crate::governance::deep_domain_rights::LifeforceSnapshot {
    use LifeforceBand::*;
    let band = match lf.band {
        crate::deep_object_router::LifeforceBand::Green => Green,
        crate::deep_object_router::LifeforceBand::Yellow => Yellow,
        crate::deep_object_router::LifeforceBand::Red => Red,
    };
    crate::governance::deep_domain_rights::LifeforceSnapshot {
        scalar01: lf.lifeforcescalar01,
        band,
    }
}
