// ============================================================================
// PHASE 26: TOPOLOGICAL DATA ANALYSIS (PERSISTENT HOMOLOGY)
// ============================================================================
// Scientific mechanism: TDA treats discrete points (network nodes) as a 
// continuous geometric space using a Vietoris-Rips simplicial complex.
// By computing the Betti numbers across increasing radii, we detect holes.
//
// beta_0 = Number of connected components
// beta_1 = Number of 1-dimensional holes (loops surrounding a void)
// 
// Origin uses this coordinate-free math to "feel" if a massive dead-zone
// has formed in the center of the Swarm, indicating structural failure.
// ============================================================================

pub struct Point {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct VietorisRipsComplex {
    pub vertices: usize,
    pub edges: usize,
    pub faces: usize,
    pub connected_components: usize,
}

impl VietorisRipsComplex {
    /// Builds a Vietoris-Rips complex from a set of points at a given radius R
    pub fn build(points: &[Point], radius: f64) -> Self {
        let n = points.len();
        let mut edges = 0;
        let mut faces = 0;
        
        let mut adj = vec![vec![false; n]; n];

        // 1. Find all edges (distance <= R)
        for i in 0..n {
            for j in (i+1)..n {
                if points[i].distance(&points[j]) <= radius {
                    adj[i][j] = true;
                    adj[j][i] = true;
                    edges += 1;
                }
            }
        }

        // 2. Find all faces (triangles where all 3 edges exist)
        for i in 0..n {
            for j in (i+1)..n {
                if adj[i][j] {
                    for k in (j+1)..n {
                        if adj[i][k] && adj[j][k] {
                            faces += 1;
                        }
                    }
                }
            }
        }

        // 3. Compute beta_0 (Connected Components) using DFS
        let mut visited = vec![false; n];
        let mut connected_components = 0;
        for i in 0..n {
            if !visited[i] {
                connected_components += 1;
                Self::dfs(i, &adj, &mut visited);
            }
        }

        Self {
            vertices: n,
            edges,
            faces,
            connected_components,
        }
    }

    fn dfs(node: usize, adj: &[Vec<bool>], visited: &mut Vec<bool>) {
        visited[node] = true;
        for neighbor in 0..adj.len() {
            if adj[node][neighbor] && !visited[neighbor] {
                Self::dfs(neighbor, adj, visited);
            }
        }
    }

    /// Computes the first Betti number (beta_1), which is the number of 1D holes.
    /// Uses the Euler characteristic: chi = V - E + F = beta_0 - beta_1 + beta_2.
    /// Assuming a localized 2D structure (beta_2 = 0), we solve for beta_1.
    pub fn compute_betti_1(&self) -> usize {
        // chi = V - E + F
        let euler_characteristic = self.vertices as isize - self.edges as isize + self.faces as isize;
        // beta_1 = beta_0 - chi
        let b1 = self.connected_components as isize - euler_characteristic;
        
        if b1 < 0 { 0 } else { b1 as usize }
    }
}

/// Scans a neighborhood for topological voids by checking Persistence.
/// Returns the maximum beta_1 (holes) found across scales.
pub fn scan_for_persistent_voids(points: &[Point]) -> Option<usize> {
    let mut max_holes = 0;
    
    // Slide the scale R from 10.0 to 30.0 to test for Persistence
    for r in [10.0, 15.0, 20.0, 25.0, 30.0].iter() {
        let complex = VietorisRipsComplex::build(points, *r);
        let holes = complex.compute_betti_1();
        if holes > max_holes {
            max_holes = holes;
        }
    }

    if max_holes > 0 {
        Some(max_holes)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_void_detection() {
        // Create a solid cluster of 4 points forming a square with crossing diagonals
        let solid_points = vec![
            Point { id: 0, x: 0.0, y: 0.0 },
            Point { id: 1, x: 10.0, y: 0.0 },
            Point { id: 2, x: 0.0, y: 10.0 },
            Point { id: 3, x: 10.0, y: 10.0 },
        ];
        
        // At radius 15.0, diagonals connect. Faces = 4. V=4, E=6, F=4.
        // chi = 4 - 6 + 4 = 2. beta_0 = 1. beta_1 = 1 - 2 < 0 -> 0 holes.
        let solid_complex = VietorisRipsComplex::build(&solid_points, 15.0);
        assert_eq!(solid_complex.compute_betti_1(), 0, "Solid topology should have NO holes.");

        // Create a "ring" of 4 points forming a hollow square (distance 10)
        // No diagonal connections at radius 11.0
        let ring_points = vec![
            Point { id: 0, x: 0.0, y: 0.0 },
            Point { id: 1, x: 10.0, y: 0.0 },
            Point { id: 2, x: 10.0, y: 10.0 },
            Point { id: 3, x: 0.0, y: 10.0 },
        ];

        let ring_complex = VietorisRipsComplex::build(&ring_points, 11.0);
        // V=4, E=4, F=0. chi = 4 - 4 + 0 = 0. beta_0 = 1. beta_1 = 1 - 0 = 1.
        assert_eq!(ring_complex.compute_betti_1(), 1, "Ring topology MUST have 1 persistent hole.");
        
        let void_scan = scan_for_persistent_voids(&ring_points);
        assert!(void_scan.is_some(), "Persistent Homology failed to detect the geometric void.");
    }
}
