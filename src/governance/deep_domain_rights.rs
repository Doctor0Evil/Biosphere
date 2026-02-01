#![forbid(unsafe_code)]

use std::collections::HashSet;

/// Minimal lifeforce representation (aligned with lifeforce governor).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifeforceBand {
    Green,
    Yellow,
    Red,
}

#[derive(Clone, Debug)]
pub struct LifeforceSnapshot {
    pub scalar01: f32,
    pub band: LifeforceBand,
}

/// B-layer kinds, aligned with deep_object_router.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BiophysicalLayer {
    B1Surface,
    B2Structural,
    B3Deep,
    B4Rare,
    Excluded,
}

/// Parsed view of deep-domain-rights.aln (minimal fields needed).
#[derive(Clone, Debug)]
pub struct LayerPolicy {
    pub allow_echo: bool,
    pub allow_excavation: bool,
    pub max_daily_epochs: u32,
    pub max_epochs_per_turn: u32,
    pub min_lf_scalar01: f32,
    pub min_lf_bands: HashSet<LifeforceBand>,
    pub max_ecoenergynj_per_epoch: f32,
    pub require_brain_token_min: f32,
    pub require_draculawave_min: f32,
    pub require_irreversible_token: bool,
    pub require_intent_tag: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DeepDomainRightsProfile {
    pub host_id: String,
    pub profile_id: String,
    pub sovereign_owner_did: String,
    pub allowed_roles: HashSet<String>,
    pub denied_roles: HashSet<String>,
    pub ai_platforms_may_execute: bool,
    pub layer_policies: std::collections::BTreeMap<BiophysicalLayer, LayerPolicy>,
    pub b2_max_epochs_per_day: u32,
    pub b3_max_epochs_per_day: u32,
    pub b4_max_epochs_per_day: u32,
    pub brain_tokens_daily_budget: f32,
    pub draculawave_daily_budget: f32,
    pub eco_nj_daily_budget: f32,
    pub on_budget_exhaustion_mode: String,
    pub require_transcripthash: bool,
    pub require_human_readable_explanation: bool,
    pub irreversible_token_scope_id: String,
    pub forbid_external_host_routing: bool,
}

/// Verification result for profile invariants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeepDomainRightsStatus {
    RightsSafe,
    ViolatesInvariant(Vec<String>),
}

impl DeepDomainRightsProfile {
    /// Mechanical check that this profile satisfies all deep-domain invariants
    /// consistent with host-augmentation-right and Organichain sovereignty.
    pub fn verify_rights_safe(&self) -> DeepDomainRightsStatus {
        let mut errors = Vec::new();

        // 1. Sovereign owner + roles
        if self.sovereign_owner_did != self.host_id {
            errors.push("sovereign_owner_did must equal host-id; no external owner".into());
        }
        if !self.allowed_roles.contains("augmented-citizen") {
            errors.push("Role 'augmented-citizen' must be allowed for deep-domain control".into());
        }
        for bad in ["vendor-generic", "sandbox", "pure-machine"] {
            if !self.denied_roles.contains(bad) {
                errors.push(format!(
                    "Role '{}' must be explicitly denied from deep-domain mutation",
                    bad
                ));
            }
        }
        if self.ai_platforms_may_execute {
            errors.push("AI platforms may not execute deep-domain state changes, propose-only".into());
        }

        // 2. Layer policies: B3/B4 must be strictly tighter than B2.
        let b2 = self.layer_policies.get(&BiophysicalLayer::B2Structural);
        let b3 = self.layer_policies.get(&BiophysicalLayer::B3Deep);
        let b4 = self.layer_policies.get(&BiophysicalLayer::B4Rare);

        if let (Some(b2), Some(b3)) = (b2, b3) {
            if b3.min_lf_scalar01 < b2.min_lf_scalar01 {
                errors.push("B3.min_lifeforce must be >= B2.min_lifeforce".into());
            }
            if b3.max_ecoenergynj_per_epoch > b2.max_ecoenergynj_per_epoch {
                errors.push("B3.max_ecoenergynj_per_epoch must be <= B2.max_ecoenergynj_per_epoch".into());
            }
            if self.b3_max_epochs_per_day > self.b2_max_epochs_per_day {
                errors.push("b3_max_epochs_per_day must be <= b2_max_epochs_per_day".into());
            }
        } else {
            errors.push("Layer policies for B2 and B3 must be defined".into());
        }

        if let (Some(b3p), Some(b4p)) = (b3, b4) {
            if b4p.min_lf_scalar01 < b3p.min_lf_scalar01 {
                errors.push("B4.min_lifeforce must be >= B3.min_lifeforce".into());
            }
            if b4p.max_ecoenergynj_per_epoch > b3p.max_ecoenergynj_per_epoch {
                errors.push("B4.max_ecoenergynj_per_epoch must be <= B3.max_ecoenergynj_per_epoch".into());
            }
            if self.b4_max_epochs_per_day > self.b3_max_epochs_per_day {
                errors.push("b4_max_epochs_per_day must be <= b3_max_epochs_per_day".into());
            }
            if !b4p.require_irreversible_token {
                errors.push("B4 access must require an IrreversibleToken".into());
            }
        } else {
            errors.push("Layer policies for B3 and B4 must be defined".into());
        }

        // 3. Budgets must be positive and finite.
        if self.brain_tokens_daily_budget <= 0.0 {
            errors.push("brain_tokens_daily_budget must be > 0".into());
        }
        if self.draculawave_daily_budget <= 0.0 {
            errors.push("draculawave_daily_budget must be > 0".into());
        }
        if self.eco_nj_daily_budget <= 0.0 {
            errors.push("eco_nj_daily_budget must be > 0".into());
        }
        if self.on_budget_exhaustion_mode != "log-only-no-new-excavation" {
            errors.push("on_budget_exhaustion_mode must be 'log-only-no-new-excavation'".into());
        }

        // 4. Consent / irreversibility hooks.
        if !self.require_transcripthash {
            errors.push("require_transcripthash must be true for any deep-domain turn".into());
        }
        if !self.require_human_readable_explanation {
            errors.push("require_human_readable_explanation must be true".into());
        }
        if self.irreversible_token_scope_id.is_empty() {
            errors.push("irreversible_token_scope_id must be non-empty".into());
        }

        // 5. Microspace sovereignty.
        if !self.forbid_external_host_routing {
            errors.push("forbid_external_host_routing must be true; microspace is sovereign".into());
        }

        if errors.is_empty() {
            DeepDomainRightsStatus::RightsSafe
        } else {
            DeepDomainRightsStatus::ViolatesInvariant(errors)
        }
    }

    /// Check whether a single excavation request is allowed under current rights.
    #[allow(clippy::too_many_arguments)]
    pub fn can_excavate_epoch(
        &self,
        layer: BiophysicalLayer,
        lf: &LifeforceSnapshot,
        ecoenergynj: f32,
        brain_tokens_available: f32,
        draculawave_available: f32,
        daily_layer_epochs_used: u32,
        has_irreversible_token: bool,
        intent_tag: Option<&str>,
    ) -> bool {
        use BiophysicalLayer::*;

        if matches!(layer, Excluded | B1Surface) {
            return false;
        }

        let policy = match self.layer_policies.get(&layer) {
            Some(p) => p,
            None => return false,
        };

        if !policy.allow_excavation {
            return false;
        }

        // Lifeforce band + scalar
        if lf.scalar01 < policy.min_lf_scalar01 {
            return false;
        }
        if !policy.min_lf_bands.is_empty() && !policy.min_lf_bands.contains(&lf.band) {
            return false;
        }

        // Eco per-epoch
        if ecoenergynj > policy.max_ecoenergynj_per_epoch {
            return false;
        }

        // Daily count
        if daily_layer_epochs_used >= policy.max_daily_epochs {
            return false;
        }

        // Token capacities (host-bound, non-financial)
        if brain_tokens_available < policy.require_brain_token_min {
            return false;
        }
        if draculawave_available < policy.require_draculawave_min {
            return false;
        }

        // Irreversible token & intent tag, when required.
        if policy.require_irreversible_token && !has_irreversible_token {
            return false;
        }
        if let Some(required_tag) = &policy.require_intent_tag {
            if intent_tag != Some(required_tag.as_str()) {
                return false;
            }
        }

        true
    }
}
