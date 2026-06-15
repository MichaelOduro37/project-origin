// ============================================================================
// PHASE 22: CATEGORY THEORY (COMPOSITIONALITY & INTERFACES)
// ============================================================================
// Scientific mechanism: In a massive compute swarm, disjoint "Cells" (WASM 
// microservices, ML shards) must frequently compose workflows. Naive binding
// causes runtime schema crashes.
//
// We model the system as a mathematical Category:
// - Objects: Data Schemas (e.g., RawAudio, Text, Embedding)
// - Morphisms: Translation Adapters (e.g., AudioToText, TextToEmbedding)
//
// By traversing this Category, the Swarm can mathematically prove if Cell A
// can be composed with Cell B, and dynamically inject the required adapter 
// morphisms (f o g), completely preventing runtime integration failures.
// ============================================================================

use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SchemaObject(pub String);

#[derive(Clone, Debug)]
pub struct Morphism {
    pub name: String,
    pub source: SchemaObject,
    pub target: SchemaObject,
}

pub struct SchemaCategory {
    // Map of Source -> List of outgoing Morphisms
    morphisms: HashMap<SchemaObject, Vec<Morphism>>,
}

impl SchemaCategory {
    pub fn new() -> Self {
        Self {
            morphisms: HashMap::new(),
        }
    }

    /// Registers a new mathematical Morphism (adapter) in the Category.
    pub fn add_morphism(&mut self, name: &str, source: &str, target: &str) {
        let src_obj = SchemaObject(source.to_string());
        let tgt_obj = SchemaObject(target.to_string());
        
        let m = Morphism {
            name: name.to_string(),
            source: src_obj.clone(),
            target: tgt_obj,
        };

        self.morphisms.entry(src_obj).or_insert_with(Vec::new).push(m);
    }

    /// Attempts to compose two disjoint Schemas.
    /// Returns the sequence of Morphisms (f o g o h...) needed to safely
    /// translate from `source` to `target`.
    pub fn compose(&self, source: &str, target: &str) -> Option<Vec<Morphism>> {
        let start = SchemaObject(source.to_string());
        let end = SchemaObject(target.to_string());

        if start == end {
            return Some(vec![]); // Identity morphism (Zero adapters needed)
        }

        // Standard BFS for shortest categorical path
        let mut queue: VecDeque<(SchemaObject, Vec<Morphism>)> = VecDeque::new();
        let mut visited: HashSet<SchemaObject> = HashSet::new();

        queue.push_back((start.clone(), vec![]));
        visited.insert(start);

        while let Some((current_obj, path)) = queue.pop_front() {
            if current_obj == end {
                return Some(path);
            }

            if let Some(outgoing) = self.morphisms.get(&current_obj) {
                for m in outgoing {
                    if !visited.contains(&m.target) {
                        visited.insert(m.target.clone());
                        let mut new_path = path.clone();
                        new_path.push(m.clone());
                        queue.push_back((m.target.clone(), new_path));
                    }
                }
            }
        }

        None // No valid composition exists
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorical_composition() {
        let mut cat = SchemaCategory::new();
        
        // Define our available Morphisms
        cat.add_morphism("whisper_adapter", "RawAudio", "TextString");
        cat.add_morphism("bert_adapter", "TextString", "VectorEmbedding");
        cat.add_morphism("compression_adapter", "VectorEmbedding", "SparseSketch");

        // Try to compose RawAudio directly to a SparseSketch
        let composition = cat.compose("RawAudio", "SparseSketch");
        
        assert!(composition.is_some());
        let path = composition.unwrap();
        
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].name, "whisper_adapter");
        assert_eq!(path[1].name, "bert_adapter");
        assert_eq!(path[2].name, "compression_adapter");
        
        // Try an impossible composition
        let impossible = cat.compose("SparseSketch", "RawAudio");
        assert!(impossible.is_none());
    }
}
