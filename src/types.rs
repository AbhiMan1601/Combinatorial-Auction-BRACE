use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Represents a good/item in the auction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Good {
    pub id: String,
    pub name: String,
}

/// A bundle is a set of goods
pub type Bundle = HashSet<Good>;

/// A hashable key for bundles (sorted good IDs)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BundleKey {
    good_ids: Vec<String>,
}

impl BundleKey {
    fn from_bundle(bundle: &Bundle) -> Self {
        let mut good_ids: Vec<String> = bundle.iter().map(|g| g.id.clone()).collect();
        good_ids.sort();
        Self { good_ids }
    }
}

/// Represents an agent (bidder) in the auction
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    /// Initial endowment of goods
    pub endowment: Bundle,
    /// Preference ranking: higher value = more preferred
    /// Maps bundles to preference values
    preferences: HashMap<BundleKey, f64>,
    /// Store bundles for iteration
    bundles: Vec<Bundle>,
}

impl Agent {
    pub fn new(id: String, endowment: Bundle) -> Self {
        Self {
            id,
            endowment,
            preferences: HashMap::new(),
            bundles: Vec::new(),
        }
    }

    /// Add a preference for a bundle
    pub fn add_preference(&mut self, bundle: Bundle, value: f64) {
        let key = BundleKey::from_bundle(&bundle);
        self.preferences.insert(key, value);
        self.bundles.push(bundle);
    }

    /// Get preference value for a bundle, defaulting to 0.0
    pub fn preference(&self, bundle: &Bundle) -> f64 {
        let key = BundleKey::from_bundle(bundle);
        self.preferences.get(&key).copied().unwrap_or(0.0)
    }

    /// Check if agent prefers bundle1 over bundle2
    pub fn prefers(&self, bundle1: &Bundle, bundle2: &Bundle) -> bool {
        self.preference(bundle1) > self.preference(bundle2)
    }

    /// Get all bundles the agent has preferences for
    pub fn preference_bundles(&self) -> &[Bundle] {
        &self.bundles
    }
}

/// An allocation maps agents to their assigned bundles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    pub assignments: HashMap<String, Bundle>,
}

impl Allocation {
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
        }
    }

    pub fn assign(&mut self, agent_id: String, bundle: Bundle) {
        self.assignments.insert(agent_id, bundle);
    }

    pub fn get_bundle(&self, agent_id: &str) -> Option<&Bundle> {
        self.assignments.get(agent_id)
    }
}

impl Default for Allocation {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an auction run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub allocation: Allocation,
    pub prices: HashMap<String, f64>, // Price per good
    pub total_welfare: f64,
    pub is_feasible: bool,
    pub is_individually_rational: bool,
    pub is_ordinal_efficient: bool,
}

