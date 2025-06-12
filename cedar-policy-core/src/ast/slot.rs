/*
 * Copyright Cedar Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use crate::validator::{json_schema::Type, RawName}; 
use crate::ast::Slot;

/// Struct which holds the data for a generalized slot
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct SlotsTypePosition(BTreeMap<Slot, SlotTypePosition>); 

impl SlotsTypePosition {
    /// Create a new empty `SlotsTypePosition` (with no slots)
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl Default for SlotsTypePosition {
    fn default() -> Self {
        Self::new()
    }
}

/// Enum for scope position of generalized slots
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ScopePosition {
    /// Principal position in scope
    Principal,
    /// Resource position in scope
    Resource,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]

/// Stores the position and type for generalized slots
pub struct SlotTypePosition {
    position: Option<ScopePosition>,
    t: Option<Type<RawName>>, 
}