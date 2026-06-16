// ============================================================================
// PHASE 66: MYCORRHIZAL NETWORKS (THE WOOD WIDE WEB)
// ============================================================================
// Scientific mechanism: Ecology / Mycology
//
// Traditional nodes are selfish. If a node is overwhelmed, it crashes.
// Origin behaves like a forest connected by a fungal Mycorrhizal Network.
// If a node exhausts its compute ("starving sapling"), idle nodes ("canopy 
// trees") shuttle their excess compute through the network to subsidize it.
// The network absorbs DDoS attacks collectively.
// ============================================================================

#[derive(Debug, Clone)]
pub struct ResourceProfile {
    pub compute_capacity: usize,
    pub current_load: usize,
}

impl ResourceProfile {
    pub fn new(compute_capacity: usize, current_load: usize) -> Self {
        Self { compute_capacity, current_load }
    }

    pub fn is_starving(&self) -> bool {
        self.current_load > self.compute_capacity
    }

    pub fn idle_capacity(&self) -> usize {
        if self.current_load < self.compute_capacity {
            self.compute_capacity - self.current_load
        } else {
            0
        }
    }
}

pub struct MycelialNode {
    pub id: usize,
    pub resources: ResourceProfile,
}

impl MycelialNode {
    pub fn new(id: usize, capacity: usize, load: usize) -> Self {
        Self {
            id,
            resources: ResourceProfile::new(capacity, load),
        }
    }

    pub fn apply_ddos_load(&mut self, spike: usize) {
        self.resources.current_load += spike;
    }
}

pub struct MycelialNetwork {
    pub nodes: Vec<MycelialNode>,
}

impl MycelialNetwork {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: MycelialNode) {
        self.nodes.push(node);
    }

    /// Shuttles idle compute from canopy trees to any starving nodes.
    /// Returns a list of strings describing the successful shuttles.
    pub fn shuttle_resources(&mut self) -> Vec<String> {
        let mut shuttle_logs = Vec::new();

        // 1. Identify Starving Nodes
        let mut starving_indices = Vec::new();
        for (i, node) in self.nodes.iter().enumerate() {
            if node.resources.is_starving() {
                starving_indices.push(i);
            }
        }

        // 2. Identify Canopy Trees (Nodes with idle capacity)
        let mut canopy_indices = Vec::new();
        for (i, node) in self.nodes.iter().enumerate() {
            if node.resources.idle_capacity() > 0 {
                canopy_indices.push(i);
            }
        }

        // 3. Fungal Shuttling
        for starving_idx in starving_indices {
            let mut deficit = self.nodes[starving_idx].resources.current_load - self.nodes[starving_idx].resources.compute_capacity;
            
            for canopy_idx in &mut canopy_indices {
                if deficit == 0 { break; } // Starving node is fully subsidized
                if *canopy_idx == starving_idx { continue; } // Can't subsidize self

                let available = self.nodes[*canopy_idx].resources.idle_capacity();
                if available > 0 {
                    let transferred = std::cmp::min(deficit, available);
                    
                    // Canopy tree absorbs the load
                    self.nodes[*canopy_idx].resources.current_load += transferred;
                    // Starving node sheds the load
                    self.nodes[starving_idx].resources.current_load -= transferred;
                    
                    deficit -= transferred;

                    shuttle_logs.push(format!(
                        "Canopy Node {} shuttled {} compute units to starving Node {}",
                        self.nodes[*canopy_idx].id, transferred, self.nodes[starving_idx].id
                    ));
                }
            }
        }

        shuttle_logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mycorrhizal_resource_shuttling() {
        let mut network = MycelialNetwork::new();
        
        // Node 1: Small capacity, normal load
        network.add_node(MycelialNode::new(1, 100, 50));
        
        // Node 2 (Canopy Tree): Massive capacity, low load
        network.add_node(MycelialNode::new(2, 1000, 200));

        // DDoS hits Node 1
        network.nodes[0].apply_ddos_load(150);
        
        // Node 1 load is now 200. Capacity is 100. It is starving by 100 units.
        assert!(network.nodes[0].resources.is_starving());

        // Shuttle resources
        let logs = network.shuttle_resources();

        // Validate Node 1 is no longer starving (load reduced to exactly its capacity)
        assert!(!network.nodes[0].resources.is_starving());
        assert_eq!(network.nodes[0].resources.current_load, 100);

        // Validate Node 2 absorbed the load
        assert_eq!(network.nodes[1].resources.current_load, 300); // 200 original + 100 subsidized

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Canopy Node 2 shuttled 100 compute units to starving Node 1");
    }
}
