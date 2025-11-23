use crate::types::{Agent, Allocation, AuctionResult, Good};
use crate::brace::BRACEMechanism;

/// Main combinatorial auction interface
pub struct CombinatorialAuction {
    agents: Vec<Agent>,
    goods: Vec<Good>,
    mechanism: BRACEMechanism,
}

impl CombinatorialAuction {
    pub fn new(agents: Vec<Agent>, goods: Vec<Good>, epsilon: f64) -> Self {
        Self {
            agents,
            goods,
            mechanism: BRACEMechanism::new(epsilon),
        }
    }

    /// Run the auction and return the result
    pub fn run(&self) -> AuctionResult {
        // Compute allocation using BRACE mechanism
        let (allocation, prices) = self.mechanism.compute_allocation(&self.agents, &self.goods);

        // Verify properties
        let is_feasible = self.mechanism.verify_feasibility(&allocation, &self.goods);
        let is_individually_rational = 
            self.mechanism.verify_individual_rationality(&self.agents, &allocation);
        let is_ordinal_efficient = 
            self.mechanism.verify_ordinal_efficiency(&self.agents, &allocation);

        // Calculate total welfare
        let total_welfare = self.calculate_welfare(&allocation);

        // Convert prices to HashMap format
        let prices_map = prices.all_prices().clone();

        AuctionResult {
            allocation,
            prices: prices_map,
            total_welfare,
            is_feasible,
            is_individually_rational,
            is_ordinal_efficient,
        }
    }

    /// Calculate total welfare (sum of preferences)
    fn calculate_welfare(&self, allocation: &Allocation) -> f64 {
        self.agents
            .iter()
            .filter_map(|agent| {
                allocation
                    .get_bundle(&agent.id)
                    .map(|bundle| agent.preference(bundle))
            })
            .sum()
    }

    /// Get agents
    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    /// Get goods
    pub fn goods(&self) -> &[Good] {
        &self.goods
    }
}

