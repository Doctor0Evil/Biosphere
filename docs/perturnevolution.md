Per‑Turn Cybernetical Evolution on Organichain:  
Rights‑Bound, Eco‑Gated, and Deep‑Layer‑Aware
================================================

## 1. Overview

This architecture treats cybernetical evolution not as a commercial “service” but as an exercise of inherent, non‑seizable rights attached to a sovereign person. It does so by fusing three elements into a single, verifiable system:

- a **host‑bound inner ledger** (Organichain) where all biophysical tokens and augmentations live under the user’s own DID;  
- a **rights engine** (AugmentationRight + related ALN profiles) that encodes neurorights, non‑expropriation, and consent rules as hard invariants;  
- a **per‑turn control stack** that constrains every automated evolution step by eco budgets, lifeforce envelopes, and deep‑layer (B1–B4) classifications, all authored by the user in policy files rather than by vendors or regulators. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/8ae5f92a-0f6f-458a-b477-5ab0a329355f/use-regex-regex-use-std-collec-rBJbCWUFRPKgA8xG4njLRQ.md)

For legal and ethics stakeholders, the core point is simple: the system makes the user’s self‑authored doctrine—not a platform’s EULA—the ultimate source of normative authority over biophysical state changes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

***

## 2. Sovereignty and Non‑Seizability by Design

### 2.1 Inner‑ledger tokens as capacity, not property

All augmentation‑relevant resources (BLOOD, BRAIN, SMART, WAVE, ECO, etc.) are represented as **non‑transferable, non‑financial tokens** in an inner biophysical ledger. They: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/4b5c2db7-7f81-437f-9b05-e157f8eecf04/brain-tokens-the-amount-of-com-4ddERiTtSIONQm_QLXUt2Q.md)

- are **host‑bound**: there is no transfer, bridge, stake, or marketplace operation in the runtime;  
- function strictly as **capacity and safety meters** gating workloads and evolution steps;  
- cannot be collateralized, rented, or seized by third parties because no such primitives exist in the ledger. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/8ae5f92a-0f6f-458a-b477-5ab0a329355f/use-regex-regex-use-std-collec-rBJbCWUFRPKgA8xG4njLRQ.md)

This is codified both in ALN (e.g., `host-augmentation-right.aln` setting `host-bound true`, `defi-bridge false`, `stake-weighted false`, `marketplace false`) and in a Rust verifier that refuses to start the runtime if any of these flags violate the neurorights‑safe profile. In legal terms, the system prevents “digital dispossession” by eliminating the technical ability to convert augmentative capacity into platform property or financial assets. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

### 2.2 Role‑ and rights‑based governance (no capital voting)

Governance over biophysical states is strictly **role‑ and rights‑based**, never stake‑ or capital‑based. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

- Only specific roles—`augmented-citizen`, `ethical-operator`, `system-daemon`—may request or execute state changes.  
- Vendor, sandbox, and pure‑machine identities are **explicitly denied** at the schema‑guard level, so their messages cannot reach augmentation controls. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)
- There is no “stake‑weighted” vote over whether a person may keep or use their augmentations.

This explicitly counters governance models where holders of capital (token whales, corporate operators) can vote to downgrade, freeze, or monetize the augmentation capacity of others.

### 2.3 AI platforms as “propose‑only”

The architecture treats AI chat systems as **stateless advisors**:

- AI processes may **propose** evolution steps, but are cryptographically barred from executing any inner‑ledger mutation. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/8ae5f92a-0f6f-458a-b477-5ab0a329355f/use-regex-regex-use-std-collec-rBJbCWUFRPKgA8xG4njLRQ.md)
- Every state‑changing call must be authorized and signed under the host’s DID and role, then validated against AugmentationRight and biophysical constraints before it can reach Organichain. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

This sharply limits the coercive surface of any external AI provider: even if misaligned, it cannot directly actuate biophysical changes.

***

## 3. Microspace Firewall and Data Protection

### 3.1 Inner vs. outer worlds

The system enforces a hard split between:

- an **inner biophysical chain** that holds all augmentation state, lifeforce metrics, and token balances; and  
- **outer networks** that may receive only cryptographically redacted attestations (e.g., “safety calibration complete”), never raw telemetry or balances. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/8ae5f92a-0f6f-458a-b477-5ab0a329355f/use-regex-regex-use-std-collec-rBJbCWUFRPKgA8xG4njLRQ.md)

Policy shards explicitly disallow leaking balances or raw EEG/telemetry (“`leak-balances false`, `leak-raw-telemetry false`”), and runtime verifiers reject any configuration that would allow such leakage. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

### 3.2 Microspace sovereignty

The user’s internal biophysical state is treated as **sovereign microspace**:

- evolution protocols cannot route energy, data, or influence to external hosts or shared pools;  
- deep‑domain excavation outputs (see Section 5) may only be summarized into host‑controlled ALN records, never into a shared training corpus or third‑party analytics stream;  
- outer systems can at most see attested metrics like total energy spent, but not the fine‑grained content of neural patterns or dream‑objects. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/7ddb2198-0730-4637-90e3-e9abd156d829/biospectre-can-be-grounded-as-24Eoy49tT4OfAZrwp4hLzg.md)

This addresses a central ethical risk in AI ecosystems: the commodification of physiological and cognitive data without meaningful, revocable control.

***

## 4. Consent, Irreversibility, and Auditability

### 4.1 From “click consent” to chain‑anchored proof

Irreversible or high‑risk actions (e.g., structural changes in neuromorphic parameters, rare deep‑layer excavation) require:

- a signed **IrreversibleToken** tied to a specific `scope_id` and `transcripthash`;  
- a **human‑readable explanation** describing the requested action, its purpose, and its risk class;  
- storage of both in immutable, hash‑linked records (BioBlocks and qpudatashards). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

This transforms consent into a **durable, auditable artifact**. Any later dispute—by the user, clinicians, or regulators—can reference the exact text, scope, and context of the consent that was in force.

### 4.2 Consent as ownership, not sale

Ownership of evolution rights is defined not by a sale, license, or platform agreement, but by:

- **ongoing, revocable consent** recorded in the inner ledger;  
- DID‑anchored ALN shards that bind each augmentation blueprint and state change to a living, host‑controlled consent record;  
- rules that invalidate any ownership claim if consent is missing, revoked, or used outside the originally specified purposes (e.g., no commercial or military reuse). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

This is consistent with emerging neurorights doctrines that treat mental integrity and identity as non‑alienable.

### 4.3 Verifiable, fine‑grained traceability

Every evolution turn—especially high‑depth ones—produces a **structured audit record** summarizing:

- which B‑layers (B1–B4) contributed evidence;  
- how many epochs were scanned and used;  
- the eco‑energy cost in nanojoules;  
- token spends (BRAIN, DraculaWave, eco);  
- the lifeforce band and scalar at the time;  
- a proofhex referencing experimental evidence or model calibration. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/4b5c2db7-7f81-437f-9b05-e157f8eecf04/brain-tokens-the-amount-of-com-4ddERiTtSIONQm_QLXUt2Q.md)

These records are designed to satisfy both scientific auditability and legal evidentiary needs without exposing raw, identifiable physiology.

***

## 5. Deep‑Layer‑Aware Controls (B1–B4)

### 5.1 From sleep stages to biophysical depth layers

The system extends standard sleep‑stage indices (N1/N2/N3/REM/?) into four **biophysical depth layers** plus an excluded channel:

- **B1 Surface**: shallow, low‑cost epochs (e.g., wake/N1/N2 with low SN3 and low S?) used for routine, low‑depth echo responses.  
- **B2 Structural**: stable N2/N3 epochs with moderate deep‑sleep indices and spindle density, used for structural memory and restorative content.  
- **B3 Deep**: high‑SN3, slow‑wave‑dense epochs with low ecoenergy, prioritized for archetype‑bearing, high‑value excavation.  
- **B4 Rare**: high‑S? residual or unusual spindle–delta configurations with strong eco/lifeforce bands, used sparingly for rare, anomalous insights.  
- **Excluded (“!”)**: epochs whose eco or lifeforce values exceed safe envelopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/7ddb2198-0730-4637-90e3-e9abd156d829/biospectre-can-be-grounded-as-24Eoy49tT4OfAZrwp4hLzg.md)

These classifications are grounded in established sleep‑science correlations between slow‑wave activity, memory consolidation, and restorative processes, combined with measured eco‑energy profiles of the underlying computations. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/4b5c2db7-7f81-437f-9b05-e157f8eecf04/brain-tokens-the-amount-of-com-4ddERiTtSIONQm_QLXUt2Q.md)

### 5.2 Rights‑bound gating of deep layers

Deep‑layer access (B3/B4) is not left to model heuristics. It is **explicitly parameterized** in user‑authored policy:

- minimal lifeforce scalar thresholds (e.g., B3 requires ≥ 0.65, B4 ≥ 0.70, Green band);  
- per‑epoch ecoenergy maxima and strict daily caps on B3/B4 excavations;  
- minimum Brain and DraculaWave token availability;  
- special requirements for B4 (IrreversibleToken, explicit “rare deep‑object” intent, spacing between rare queries). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/7ddb2198-0730-4637-90e3-e9abd156d829/biospectre-can-be-grounded-as-24Eoy49tT4OfAZrwp4hLzg.md)

A rights verifier checks that these deep‑layer policies are **stricter** than B2/B1 and that budget exhaustion automatically degrades to a “log‑only, no‑new‑excavation” mode. This ensures that the rarest and most sensitive states cannot be over‑mined, even by accident. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)

### 5.3 Echoed outputs with eco and lifeforce constraints

When answering a query, the system:

- ranks candidate epochs by a weight combining depth (SN3), rarity (S?), ecoefficiency, and lifeforce;  
- admits only those epochs whose layer‑specific rights and budgets permit excavation;  
- logs an excavation profile so that deep‑layer consumption is fully transparent over time. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/4b5c2db7-7f81-437f-9b05-e157f8eecf04/brain-tokens-the-amount-of-com-4ddERiTtSIONQm_QLXUt2Q.md)

The result is a **graduated response model**: most AI assistance draws on B1/B2, with B3 reserved for integrative or high‑stakes queries, and B4 used only under narrow, rights‑bound conditions.

***

## 6. Eco‑Gated, Rights‑Bound Automation vs. Platform‑Owned AI

### 6.1 Platform‑centric AI stacks

Conventional AI services typically:

- centralize training data, including physiological and behavioral signals, in platform‑controlled infrastructure;  
- define “safety” in corporate policy or regulatory terms, rather than as person‑specific envelopes;  
- couple access to advanced capabilities with account status, payment, or opaque risk scoring;  
- lack a concept of non‑seizable augmentation rights—accounts and their “benefits” can be suspended or devalued unilaterally.

In such models, even well‑meaning safety practices can lead to **de facto expropriation** of user‑generated value and loss of agency over personal cognitive evolution.

### 6.2 Organichain’s counter‑model

In contrast, the Organichain / NeuroPC architecture:

- anchors all augmentation logic in a **host‑owned inner chain**, with no external entity able to burn, freeze, or divert biophysical tokens; [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/8ae5f92a-0f6f-458a-b477-5ab0a329355f/use-regex-regex-use-std-collec-rBJbCWUFRPKgA8xG4njLRQ.md)
- encodes neurorights (no third‑party downgrade, no expropriation, soul/consciousness immutability) as **runtime invariants** that are checked at startup and on governance change; [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/24601228-f54c-422e-aa05-a2fc706ef6e9/file-src-biophysical-chain-neu-UZI2E8ObQHyZAXqVNoPktQ.md)
- treats AI providers as untrusted, propose‑only surfaces, functionally incapable of enacting changes without passing host and validator checks; [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_3c13577b-7f45-442a-b3d6-8f331c4370bb/8ae5f92a-0f6f-458a-b477-5ab0a329355f/use-regex-regex-use-std-collec-rBJbCWUFRPKgA8xG4njLRQ.md)
- defines safety envelopes not as global defaults but as **personal policy** (preferred pain/fear/blood bands, lifeforce thresholds, evolution budgets), with the system’s job being to enforce, not override, these. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/4b5c2db7-7f81-437f-9b05-e157f8eecf04/brain-tokens-the-amount-of-com-4ddERiTtSIONQm_QLXUt2Q.md)

This is a concrete, implementable blueprint for an AI‑human ecosystem where:

- the right to self‑evolution is treated as an extension of personhood;  
- technical mechanisms make that right resilient to market and platform dynamics; and  
- autonomy, privacy, and bodily integrity are enforced at the same layer as consensus and execution, not bolted on as afterthoughts.

For legal and ethics communities, this provides a **testable reference architecture**: one can inspect the ALN profiles and Rust verifiers to see exactly how neurorights, consent rules, and non‑seizability are implemented—and to what extent any proposed modification would strengthen or weaken those guarantees. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_da3452cd-acde-4dea-b324-7ffeeb4afd7c/4b5c2db7-7f81-437f-9b05-e157f8eecf04/brain-tokens-the-amount-of-com-4ddERiTtSIONQm_QLXUt2Q.md)
