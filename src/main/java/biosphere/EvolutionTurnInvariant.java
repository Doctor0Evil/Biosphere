package biosphere;

public interface EvolutionTurnInvariant {

    // Unique ID, e.g. "SMART-EVO-TURN-NEUROPC".
    String id();

    // Verify that a given proposal and block obey per-turn evolution budgets.
    boolean verifyTurnBudget(String validatorAlias,
                             String proposalId,
                             String blockId);

    // Verify that each AI-chat-driven evolution turn has a transcript hash,
    // ChatContext summary, and irreversible token linkage when required.
    boolean verifyTraceabilityAndConsent(String proposalId,
                                         String transcripthash,
                                         boolean irreversibleRequired);

    // Verify that microspace sovereignty constraints were applied
    // (self-only, host-bounded, no external negative-energy routing).
    boolean verifyMicrospaceSovereignty(String proposalId,
                                        String sovereigntyProfileId);
}
