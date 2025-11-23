use brace_sybil::{CombinatorialAuction, Agent, Good};
use std::collections::HashSet;

fn main() {
    println!("BRACE Combinatorial Auction Example\n");

    // Create goods
    let good_a = Good {
        id: "A".to_string(),
        name: "Good A".to_string(),
    };
    let good_b = Good {
        id: "B".to_string(),
        name: "Good B".to_string(),
    };
    let good_c = Good {
        id: "C".to_string(),
        name: "Good C".to_string(),
    };
    let goods = vec![good_a.clone(), good_b.clone(), good_c.clone()];

    // Create agents with endowments and preferences
    let mut agent1 = Agent::new("Agent1".to_string(), {
        let mut e = HashSet::new();
        e.insert(good_a.clone());
        e
    });

    // Agent 1 prefers bundle {B, C} most, then {A, B}, then {A}
    let mut bundle_bc = HashSet::new();
    bundle_bc.insert(good_b.clone());
    bundle_bc.insert(good_c.clone());
    agent1.add_preference(bundle_bc, 10.0);

    let mut bundle_ab = HashSet::new();
    bundle_ab.insert(good_a.clone());
    bundle_ab.insert(good_b.clone());
    agent1.add_preference(bundle_ab, 7.0);

    let mut bundle_a = HashSet::new();
    bundle_a.insert(good_a.clone());
    agent1.add_preference(bundle_a, 5.0);

    let mut agent2 = Agent::new("Agent2".to_string(), {
        let mut e = HashSet::new();
        e.insert(good_b.clone());
        e
    });

    // Agent 2 prefers bundle {A, C} most, then {B, C}, then {B}
    let mut bundle_ac = HashSet::new();
    bundle_ac.insert(good_a.clone());
    bundle_ac.insert(good_c.clone());
    agent2.add_preference(bundle_ac, 12.0);

    let mut bundle_bc2 = HashSet::new();
    bundle_bc2.insert(good_b.clone());
    bundle_bc2.insert(good_c.clone());
    agent2.add_preference(bundle_bc2, 8.0);

    let mut bundle_b = HashSet::new();
    bundle_b.insert(good_b.clone());
    agent2.add_preference(bundle_b, 4.0);

    let mut agent3 = Agent::new("Agent3".to_string(), {
        let mut e = HashSet::new();
        e.insert(good_c.clone());
        e
    });

    // Agent 3 prefers bundle {A, B} most, then {A, C}, then {C}
    let mut bundle_ab3 = HashSet::new();
    bundle_ab3.insert(good_a.clone());
    bundle_ab3.insert(good_b.clone());
    agent3.add_preference(bundle_ab3, 9.0);

    let mut bundle_ac3 = HashSet::new();
    bundle_ac3.insert(good_a.clone());
    bundle_ac3.insert(good_c.clone());
    agent3.add_preference(bundle_ac3, 6.0);

    let mut bundle_c = HashSet::new();
    bundle_c.insert(good_c.clone());
    agent3.add_preference(bundle_c, 3.0);

    let agents = vec![agent1, agent2, agent3];

    // Create and run auction
    let auction = CombinatorialAuction::new(agents, goods, 0.01);
    let result = auction.run();

    // Display results
    println!("Auction Results:");
    println!("===============\n");

    println!("Allocation:");
    for (agent_id, bundle) in &result.allocation.assignments {
        let good_names: Vec<String> = bundle.iter().map(|g| g.name.clone()).collect();
        println!("  {}: [{}]", agent_id, good_names.join(", "));
    }

    println!("\nPrices:");
    for (good_id, price) in &result.prices {
        println!("  {}: {:.2}", good_id, price);
    }

    println!("\nTotal Welfare: {:.2}", result.total_welfare);
    println!("\nProperties:");
    println!("  Feasible: {}", result.is_feasible);
    println!("  Individually Rational: {}", result.is_individually_rational);
    println!("  Ordinal Efficient: {}", result.is_ordinal_efficient);
}

