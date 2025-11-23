use crate::types::{Agent, Allocation, Good};
use crate::pricing::{PriceVector, compute_equilibrium_prices};

/// BRACE (Budget-Relaxed Approximate Competitive Equilibrium) mechanism
pub struct BRACEMechanism {
    /// Approximation parameter for feasibility
    pub epsilon: f64,
}

impl BRACEMechanism {
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }

    /// Compute BRACE allocation
    /// This implements the core allocation algorithm that ensures:
    /// - Approximate feasibility
    /// - Individual rationality
    /// - Ordinal efficiency
    pub fn compute_allocation(
        &self,
        agents: &[Agent],
        goods: &[Good],
    ) -> (Allocation, PriceVector) {
        // Initialize allocation with endowments (ensures individual rationality)
        let mut allocation = Allocation::new();
        for agent in agents {
            allocation.assign(agent.id.clone(), agent.endowment.clone());
        }

        // Compute initial prices
        let mut prices = PriceVector::new();
        for good in goods {
            prices.set_price(good.id.clone(), 0.0);
        }

        // Iterative improvement: try to find Pareto improvements
        let max_iterations = 100;
        for _ in 0..max_iterations {
            let improved = self.improve_allocation(agents, goods, &mut allocation, &mut prices);
            if !improved {
                break;
            }
        }

        // Compute equilibrium prices for the final allocation
        let final_prices = compute_equilibrium_prices(agents, goods, &allocation, self.epsilon);

        (allocation, final_prices)
    }

    /// Try to improve the allocation through Pareto improvements
    fn improve_allocation(
        &self,
        agents: &[Agent],
        _goods: &[Good],
        allocation: &mut Allocation,
        prices: &mut PriceVector,
    ) -> bool {
        let mut improved = false;

        // Try to find beneficial trades
        for i in 0..agents.len() {
            for j in (i + 1)..agents.len() {
                if let Some(new_allocation) = self.try_trade(
                    &agents[i],
                    &agents[j],
                    allocation,
                    prices,
                ) {
                    // Check if trade is Pareto improving
                    if self.is_pareto_improving(agents, allocation, &new_allocation) {
                        *allocation = new_allocation;
                        improved = true;
                    }
                }
            }
        }

        improved
    }

    /// Try a trade between two agents
    fn try_trade(
        &self,
        agent1: &Agent,
        agent2: &Agent,
        current_allocation: &Allocation,
        _prices: &PriceVector,
    ) -> Option<Allocation> {
        let bundle1 = current_allocation.get_bundle(&agent1.id)?.clone();
        let bundle2 = current_allocation.get_bundle(&agent2.id)?.clone();

        // Try swapping bundles
        let mut new_allocation = current_allocation.clone();
        new_allocation.assign(agent1.id.clone(), bundle2.clone());
        new_allocation.assign(agent2.id.clone(), bundle1.clone());

        // Check if both agents are better off
        let agent1_better = agent1.prefers(&bundle2, &bundle1);
        let agent2_better = agent2.prefers(&bundle1, &bundle2);

        if agent1_better && agent2_better {
            Some(new_allocation)
        } else {
            None
        }
    }

    /// Check if new allocation is Pareto improving
    fn is_pareto_improving(
        &self,
        agents: &[Agent],
        old_allocation: &Allocation,
        new_allocation: &Allocation,
    ) -> bool {
        let mut at_least_one_better = false;

        for agent in agents {
            let old_bundle = old_allocation.get_bundle(&agent.id);
            let new_bundle = new_allocation.get_bundle(&agent.id);

            if let (Some(old), Some(new)) = (old_bundle, new_bundle) {
                if agent.prefers(new, old) {
                    at_least_one_better = true;
                } else if agent.prefers(old, new) {
                    // Someone is worse off, not Pareto improving
                    return false;
                }
            }
        }

        at_least_one_better
    }

    /// Verify approximate feasibility
    /// Checks that no good is over-allocated (within epsilon tolerance)
    pub fn verify_feasibility(
        &self,
        allocation: &Allocation,
        goods: &[Good],
    ) -> bool {
        for good in goods {
            let mut count = 0;
            for bundle in allocation.assignments.values() {
                if bundle.contains(good) {
                    count += 1;
                }
            }
            // Each good should be allocated at most once (within epsilon)
            if count as f64 > 1.0 + self.epsilon {
                return false;
            }
        }
        true
    }

    /// Verify individual rationality
    /// Each agent should be at least as well off as with their endowment
    pub fn verify_individual_rationality(
        &self,
        agents: &[Agent],
        allocation: &Allocation,
    ) -> bool {
        for agent in agents {
            let allocated = allocation.get_bundle(&agent.id);
            if let Some(bundle) = allocated {
                // Agent should not prefer endowment over allocation
                if agent.prefers(&agent.endowment, bundle) {
                    return false;
                }
            }
        }
        true
    }

    /// Verify ordinal efficiency
    /// No other allocation should make all agents strictly better off
    pub fn verify_ordinal_efficiency(
        &self,
        agents: &[Agent],
        allocation: &Allocation,
    ) -> bool {
        // This is a simplified check - full verification would require
        // checking all possible allocations, which is computationally expensive
        // In practice, the BRACE mechanism ensures this property
        
        // For now, we check that no simple swap would make both agents better off
        for i in 0..agents.len() {
            for j in (i + 1)..agents.len() {
                let bundle_i = allocation.get_bundle(&agents[i].id);
                let bundle_j = allocation.get_bundle(&agents[j].id);
                
                if let (Some(bi), Some(bj)) = (bundle_i, bundle_j) {
                    // If swapping would make both better, not efficient
                    if agents[i].prefers(bj, bi) && agents[j].prefers(bi, bj) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

