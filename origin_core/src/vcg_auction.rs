// ============================================================================
// PHASE 18: OPTIMAL AUCTION THEORY & VICKREY-CLARKE-GROVES (VCG) MECHANISM
// ============================================================================
// Scientific mechanism: VCG is a mathematically perfect auction design.
// Instead of charging winners what they bid, it charges them the "social cost" 
// they inflict on others. 
//
// Social Cost = (Total Value if Agent i didn't exist) - (Total Value of others when i exists)
//
// This perfectly aligns individual incentives with global efficiency. The 
// strict dominant strategy for every node in the Swarm is to bid their exact 
// true valuation. Spoofing or market manipulation becomes mathematically useless.
// ============================================================================

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Bid {
    pub agent_id: String,
    pub resources_requested: usize, // e.g., gigabytes of storage or CPU cores
    pub valuation: f64,             // true value the agent places on the resources
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VCGResult {
    pub agent_id: String,
    pub resources_allocated: usize,
    pub bid_valuation: f64,
    pub vcg_payment: f64, // The truthful social cost
}

pub struct VCGAuction {
    pub total_capacity: usize,
}

impl VCGAuction {
    pub fn new(total_capacity: usize) -> Self {
        Self { total_capacity }
    }

    /// Resolves the auction using the VCG Mechanism.
    /// This uses a dynamic programming approach (0-1 Knapsack) to find the optimal 
    /// allocation that maximizes total social welfare.
    pub fn resolve(&self, bids: &[Bid]) -> Vec<VCGResult> {
        if bids.is_empty() || self.total_capacity == 0 {
            return vec![];
        }

        // 1. Find optimal allocation with ALL agents
        let (max_val_all, winners_all) = self.optimal_allocation(bids);

        let mut results = Vec::new();

        // 2. Calculate VCG payment for each winner
        for winner in &winners_all {
            // Calculate value of the other winning agents when this winner is present
            let value_others_with_i = max_val_all - winner.valuation;

            // Calculate the optimal allocation if this winner had NEVER participated
            let mut bids_without_i = bids.to_vec();
            bids_without_i.retain(|b| b.agent_id != winner.agent_id);
            
            let (max_val_without_i, _) = self.optimal_allocation(&bids_without_i);

            // VCG Payment = (Value of network without i) - (Value of others with i)
            let payment = max_val_without_i - value_others_with_i;

            results.push(VCGResult {
                agent_id: winner.agent_id.clone(),
                resources_allocated: winner.resources_requested,
                bid_valuation: winner.valuation,
                vcg_payment: payment.max(0.0), // Floating point safety
            });
        }

        results
    }

    /// Helper: Solves the 0-1 Knapsack problem to maximize total valuation
    fn optimal_allocation(&self, bids: &[Bid]) -> (f64, Vec<Bid>) {
        let n = bids.len();
        let w = self.total_capacity;

        // DP table: dp[i][j] stores the max valuation using first i bids and capacity j
        // We scale the capacity if it's large, but assume it's a manageable integer here.
        let mut dp = vec![vec![0.0; w + 1]; n + 1];

        for i in 1..=n {
            let bid = &bids[i - 1];
            for j in 0..=w {
                if bid.resources_requested <= j {
                    let val_include = bid.valuation + dp[i - 1][j - bid.resources_requested];
                    let val_exclude = dp[i - 1][j];
                    dp[i][j] = val_include.max(val_exclude);
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        // Backtrack to find the winning bids
        let mut winners = Vec::new();
        let mut res = dp[n][w];
        let mut cap = w;

        for i in (1..=n).rev() {
            if res <= 0.0 {
                break;
            }
            if (res - dp[i - 1][cap]).abs() > 1e-9 {
                // Item i-1 was included
                let bid = &bids[i - 1];
                winners.push(bid.clone());
                res -= bid.valuation;
                cap -= bid.resources_requested;
            }
        }

        (dp[n][w], winners)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vcg_auction_truthfulness() {
        let auction = VCGAuction::new(10); // 10 units of capacity

        // Bidders
        let bids = vec![
            Bid { agent_id: "A".into(), resources_requested: 5, valuation: 10.0 }, // Value 2 per unit
            Bid { agent_id: "B".into(), resources_requested: 5, valuation: 10.0 }, // Value 2 per unit
            Bid { agent_id: "C".into(), resources_requested: 6, valuation: 11.0 }, // Value < 2 per unit
        ];

        let results = auction.resolve(&bids);
        
        assert_eq!(results.len(), 2); // A and B should win (Total 20.0 > 11.0)
        
        let a_res = results.iter().find(|r| r.agent_id == "A").unwrap();
        let b_res = results.iter().find(|r| r.agent_id == "B").unwrap();

        // If A didn't exist, B and C can't both fit (5+6=11 > 10). Only C wins, providing 11.0.
        // Total value with A is 20.0. Value of others with A is 10.0 (B's value).
        // VCG Payment for A = 11.0 - 10.0 = 1.0.
        assert!((a_res.vcg_payment - 1.0).abs() < 1e-9);
        assert!((b_res.vcg_payment - 1.0).abs() < 1e-9);
    }
}
