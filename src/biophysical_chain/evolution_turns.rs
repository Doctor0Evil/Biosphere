//! Per-turn automation helpers for AI-chat–driven cybernetical evolution.
//! Integrates with NeuroAutomationPipeline and OrganichainConsensus.

use serde::{Deserialize, Serialize};

use crate::biophysical_chain_neuroautomationpipeline::{
    BiophysicalConstraints,
    BiophysicalPattern,
    ChatContext,
    EvolutionCategory,
    EvolutionProposal,
    Reversibility,
};
use crate::organichainconsensus::OrganichainConsensus;

/// 1) Turn-level evolution intent flag.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvolutionIntentKind {
    None,
    MicroEpoch,
    SingleStep,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionIntent {
    pub kind: EvolutionIntentKind,
    /// BRAIN, WAVE, BLOOD, OXYGEN, etc., if inferred.
    pub domain: Option<String>,
    /// Host-chosen risk tier label, e.g. "low", "medium", "experimental".
    pub risk_tier: Option<String>,
}

/// Simple intent detector from chat summary and text tags.
pub fn detect_evolution_intent(summary: &str) -> EvolutionIntent {
    let lower = summary.to_lowercase();
    if lower.contains("cybernetical-evolution") || lower.contains("evolution-step") {
        EvolutionIntent {
            kind: EvolutionIntentKind::MicroEpoch,
            domain: Some("BRAIN".into()),
            risk_tier: Some("low".into()),
        }
    } else {
        EvolutionIntent {
            kind: EvolutionIntentKind::None,
            domain: None,
            risk_tier: None,
        }
    }
}

/// 2) Turn-to-proposal compiler using fixed templates.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EvolutionTemplateId {
    MicroEpochVisualFocusV1,
    MicroEpochHapticGuidanceLeftRightV1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionTemplateRequest {
    pub template: EvolutionTemplateId,
    pub chat: ChatContext,
}

/// Compile a template request into a constrained EvolutionProposal.
pub fn compile_template_to_proposal(
    req: EvolutionTemplateRequest,
) -> EvolutionProposal {
    match req.template {
        EvolutionTemplateId::MicroEpochVisualFocusV1 => {
            // Uses existing safe visual pattern from your .neuro/.neupat assets.
            let pattern = BiophysicalPattern {
                target: "cortex.visual.v1".into(),
                intensity: 0.25,
                duration: std::time::Duration::from_secs(45),
                reversibility: Reversibility::FullyReversible,
            };
            EvolutionProposal {
                id: crate::biophysical_chain_neuroautomationpipeline::ChainId::new(
                    "proposal-microepoch-visual-focus-v1",
                ),
                chat: req.chat,
                category: EvolutionCategory::SensoryAugmentation,
                patterns: vec![pattern],
                justificationuri: Some(
                    "neuro://pattern/PAT.VisualEdgeEnhanceKernelV1".into(),
                ),
                // filled by other modules: keyops, irreversible_token, etc.
            }
        }
        EvolutionTemplateId::MicroEpochHapticGuidanceLeftRightV1 => {
            let left = BiophysicalPattern {
                target: "peripheral.haptics.leftarm".into(),
                intensity: 0.20,
                duration: std::time::Duration::from_secs(30),
                reversibility: Reversibility::FullyReversible,
            };
            let right = BiophysicalPattern {
                target: "peripheral.haptics.rightarm".into(),
                intensity: 0.20,
                duration: std::time::Duration::from_secs(30),
                reversibility: Reversibility::FullyReversible,
            };
            EvolutionProposal {
                id: crate::biophysical_chain_neuroautomationpipeline::ChainId::new(
                    "proposal-microepoch-haptic-guidance-lr-v1",
                ),
                chat: req.chat,
                category: EvolutionCategory::CognitiveScaffolding,
                patterns: vec![left, right],
                justificationuri: Some(
                    "neuro://pattern/PAT.HapticGuidancePulsePairV1".into(),
                ),
            }
        }
    }
}

/// 3) Per-turn eligibility result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TurnEligibility {
    Eligible,
    LogOnly(String),
}

/// Check Organichain evolution-interval limits plus host constraints.
pub fn check_turn_eligibility(
    organichain: &mut OrganichainConsensus,
    validator_alias: &str,
    now: std::time::SystemTime,
) -> TurnEligibility {
    if !organichain.cantakeevolutionstep(validator_alias, now) {
        return TurnEligibility::LogOnly(
            "evolution-interval budget or spacing exhausted for today".into(),
        );
    }
    TurnEligibility::Eligible
}

/// 4) SMART metadata + epoch tagging.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TurnSmartMeta {
    pub learning_epoch: u64,
    pub qpudatashard_id: String,
    pub entropy_seed_hex: String,
}

impl TurnSmartMeta {
    pub fn default_for_epoch(epoch: u64, shard: &str) -> Self {
        Self {
            learning_epoch: epoch,
            qpudatashard_id: shard.to_string(),
            entropy_seed_hex: "0xEVO-TURN-SEED-0001".into(),
        }
    }
}

/// 5) Attach irreversible guard requirements per turn.
pub fn require_irreversible_token_if_needed(
    constraints: &BiophysicalConstraints,
    proposal: &EvolutionProposal,
) -> bool {
    if !constraints.requireirreversibleconfirmation {
        return true;
    }
    let mut needs_token = false;
    for p in &proposal.patterns {
        if p.reversibility == Reversibility::Irreversible {
            needs_token = true;
            break;
        }
    }
    if !needs_token {
        return true;
    }
    proposal.irreversibletoken.is_some()
}
