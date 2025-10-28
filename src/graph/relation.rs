// Relation types for graph edges

/// Common relation types for agent memory
#[allow(dead_code)]
pub struct RelationType;

#[allow(dead_code)]
impl RelationType {
    /// Generic related-to relationship
    pub const RELATED_TO: &'static str = "related_to";

    /// Causal relationship (A caused B)
    pub const CAUSED_BY: &'static str = "caused_by";

    /// Temporal relationship (A happened before B)
    pub const BEFORE: &'static str = "before";

    /// Temporal relationship (A happened after B)
    pub const AFTER: &'static str = "after";

    /// Reference relationship (A references B)
    pub const REFERENCES: &'static str = "references";

    /// Similarity relationship
    pub const SIMILAR_TO: &'static str = "similar_to";

    /// Contradiction relationship
    pub const CONTRADICTS: &'static str = "contradicts";

    /// Supports/reinforces relationship
    pub const SUPPORTS: &'static str = "supports";
}
