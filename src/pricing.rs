use crate::types::{Agent, Bundle, Good};
use std::collections::HashMap;

/// Price vector for goods
#[derive(Debug, Clone)]
pub struct PriceVector {
    prices: HashMap<String, f64>,
}

impl PriceVector {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
        }
    }

    pub fn from_map(prices: HashMap<String, f64>) -> Self {
        Self { prices }
    }

    pub fn set_price(&mut self, good_id: String, price: f64) {
        self.prices.insert(good_id, price);
    }

    pub fn get_price(&self, good_id: &str) -> f64 {
        self.prices.get(good_id).copied().unwrap_or(0.0)
    }

    /// Calculate the price of a bundle
    pub fn bundle_price(&self, bundle: &Bundle) -> f64 {
        bundle.iter().map(|good| self.get_price(&good.id)).sum()
    }

    /// Calculate net utility: preference value minus price
    pub fn net_utility(&self, agent: &Agent, bundle: &Bundle) -> f64 {
        agent.preference(bundle) - self.bundle_price(bundle)
    }

    /// Find the demand set: bundles that maximize net utility
    pub fn demand_set(&self, agent: &Agent) -> Vec<Bundle> {
        let mut best_utility = f64::NEG_INFINITY;
        let mut demand = Vec::new();

        for bundle in agent.preference_bundles() {
            let utility = self.net_utility(agent, bundle);
            if utility > best_utility {
                best_utility = utility;
                demand.clear();
                demand.push(bundle.clone());
            } else if (utility - best_utility).abs() < 1e-9 {
                demand.push(bundle.clone());
            }
        }

        demand
    }

    pub fn all_prices(&self) -> &HashMap<String, f64> {
        &self.prices
    }
}

impl Default for PriceVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute approximate competitive equilibrium prices
/// This implements a price adjustment algorithm to find prices
/// that support the BRACE allocation
pub fn compute_equilibrium_prices(
    agents: &[Agent],
    goods: &[Good],
    allocation: &crate::types::Allocation,
    epsilon: f64,
) -> PriceVector {
    let mut prices = PriceVector::new();
    
    // Initialize prices to zero
    for good in goods {
        prices.set_price(good.id.clone(), 0.0);
    }

    // Iterative price adjustment
    let max_iterations = 1000;
    let step_size = 0.1;

    for _ in 0..max_iterations {
        let mut price_changes = HashMap::new();
        
        // For each agent, check if their allocation is in their demand set
        for agent in agents {
            if let Some(allocated_bundle) = allocation.get_bundle(&agent.id) {
                let demand = prices.demand_set(agent);
                
                // Check if allocated bundle is in demand
                let in_demand = demand.iter().any(|b| {
                    b.len() == allocated_bundle.len() && 
                    b.iter().all(|g| allocated_bundle.contains(g))
                });
                
                // If allocated bundle is not in demand, adjust prices
                if !in_demand {
                    // Increase prices of goods in allocated bundle
                    for good in allocated_bundle {
                        *price_changes.entry(good.id.clone()).or_insert(0.0) += step_size;
                    }
                }
            }
        }

        // Check convergence before applying changes
        let max_change = price_changes.values().map(|&v: &f64| v.abs()).fold(0.0, f64::max);
        if max_change < epsilon {
            break;
        }

        // Apply price changes
        for (good_id, change) in &price_changes {
            let current = prices.get_price(good_id);
            prices.set_price(good_id.clone(), current + change);
        }
    }

    prices
}

