use brace_sybil::{CombinatorialAuction, Agent, Good};
use std::collections::HashSet;

#[test]
fn test_simple_auction() {
    // Create two goods
    let good_a = Good {
        id: "A".to_string(),
        name: "Good A".to_string(),
    };
    let good_b = Good {
        id: "B".to_string(),
        name: "Good B".to_string(),
    };
    let goods = vec![good_a.clone(), good_b.clone()];

    // Create two agents
    let mut agent1 = Agent::new("Agent1".to_string(), {
        let mut e = HashSet::new();
        e.insert(good_a.clone());
        e
    });
    
    let mut bundle_ab = HashSet::new();
    bundle_ab.insert(good_a.clone());
    bundle_ab.insert(good_b.clone());
    agent1.add_preference(bundle_ab, 10.0);
    
    let mut bundle_a = HashSet::new();
    bundle_a.insert(good_a.clone());
    agent1.add_preference(bundle_a, 5.0);

    let mut agent2 = Agent::new("Agent2".to_string(), {
        let mut e = HashSet::new();
        e.insert(good_b.clone());
        e
    });
    
    let mut bundle_ab2 = HashSet::new();
    bundle_ab2.insert(good_a.clone());
    bundle_ab2.insert(good_b.clone());
    agent2.add_preference(bundle_ab2, 8.0);
    
    let mut bundle_b = HashSet::new();
    bundle_b.insert(good_b.clone());
    agent2.add_preference(bundle_b, 4.0);

    let agents = vec![agent1, agent2];

    // Run auction
    let auction = CombinatorialAuction::new(agents, goods, 0.01);
    let result = auction.run();

    // Verify properties
    assert!(result.is_feasible, "Allocation should be feasible");
    assert!(
        result.is_individually_rational,
        "Allocation should be individually rational"
    );
    assert!(
        result.is_ordinal_efficient,
        "Allocation should be ordinally efficient"
    );

    // Verify all agents have allocations
    assert_eq!(result.allocation.assignments.len(), 2);
}

#[test]
fn test_individual_rationality() {
    let good_a = Good {
        id: "A".to_string(),
        name: "Good A".to_string(),
    };
    let goods = vec![good_a.clone()];

    let mut agent = Agent::new("Agent1".to_string(), {
        let mut e = HashSet::new();
        e.insert(good_a.clone());
        e
    });

    let mut bundle_a = HashSet::new();
    bundle_a.insert(good_a.clone());
    agent.add_preference(bundle_a, 5.0);

    let agents = vec![agent];
    let auction = CombinatorialAuction::new(agents, goods, 0.01);
    let result = auction.run();

    // Agent should keep their endowment (individual rationality)
    assert!(result.is_individually_rational);
    assert!(result.allocation.get_bundle("Agent1").is_some());
}

