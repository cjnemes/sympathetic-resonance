//! Serde helper functions for serializing types with special requirements
//!
//! This module provides custom serialization/deserialization helpers for types
//! that don't have default JSON representations, particularly HashMaps with enum keys.

use std::collections::HashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::systems::factions::FactionId;
use crate::systems::knowledge::LearningMethod;
use crate::core::world_state::Direction;

/// Serialize HashMap<FactionId, V> as Vec<(FactionId, V)> for JSON compatibility
pub fn serialize_faction_map<V, S>(
    map: &HashMap<FactionId, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let vec: Vec<_> = map.iter().collect();
    vec.serialize(serializer)
}

/// Deserialize HashMap<FactionId, V> from Vec<(FactionId, V)>
pub fn deserialize_faction_map<'de, V, D>(
    deserializer: D,
) -> Result<HashMap<FactionId, V>, D::Error>
where
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let vec: Vec<(FactionId, V)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

/// Serialize HashMap<LearningMethod, V> as Vec<(LearningMethod, V)> for JSON compatibility
pub fn serialize_learning_method_map<V, S>(
    map: &HashMap<LearningMethod, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let vec: Vec<_> = map.iter().collect();
    vec.serialize(serializer)
}

/// Deserialize HashMap<LearningMethod, V> from Vec<(LearningMethod, V)>
pub fn deserialize_learning_method_map<'de, V, D>(
    deserializer: D,
) -> Result<HashMap<LearningMethod, V>, D::Error>
where
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let vec: Vec<(LearningMethod, V)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

/// Serialize HashMap<i32, V> as Vec<(i32, V)> for JSON compatibility
pub fn serialize_i32_map<V, S>(
    map: &HashMap<i32, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let vec: Vec<_> = map.iter().collect();
    vec.serialize(serializer)
}

/// Deserialize HashMap<i32, V> from Vec<(i32, V)>
pub fn deserialize_i32_map<'de, V, D>(
    deserializer: D,
) -> Result<HashMap<i32, V>, D::Error>
where
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let vec: Vec<(i32, V)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

/// Serialize HashMap<(FactionId, FactionId), V> as Vec<((FactionId, FactionId), V)> for JSON compatibility
pub fn serialize_faction_pair_map<V, S>(
    map: &HashMap<(FactionId, FactionId), V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let vec: Vec<_> = map.iter().collect();
    vec.serialize(serializer)
}

/// Deserialize HashMap<(FactionId, FactionId), V> from Vec<((FactionId, FactionId), V)>
pub fn deserialize_faction_pair_map<'de, V, D>(
    deserializer: D,
) -> Result<HashMap<(FactionId, FactionId), V>, D::Error>
where
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let vec: Vec<((FactionId, FactionId), V)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

/// Serialize HashMap<Direction, V> as Vec<(Direction, V)> for JSON compatibility
pub fn serialize_direction_map<V, S>(
    map: &HashMap<Direction, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let vec: Vec<_> = map.iter().collect();
    vec.serialize(serializer)
}

/// Deserialize HashMap<Direction, V> from Vec<(Direction, V)>
pub fn deserialize_direction_map<'de, V, D>(
    deserializer: D,
) -> Result<HashMap<Direction, V>, D::Error>
where
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let vec: Vec<(Direction, V)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}
