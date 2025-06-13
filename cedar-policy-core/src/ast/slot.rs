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

use crate::ast::SlotId;
use crate::validator::{json_schema::Type, RawName};
use serde::{Deserialize, Serialize};

/// Struct which holds the data for a generalized slot
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct SlotTypePositionAnnotations(BTreeMap<SlotId, SlotTypePosition>);

impl SlotTypePositionAnnotations {
    /// Create a new empty `SlotTypePositionAnnotations` (with no slots)
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Get an SlotTypePositionAnnotations by key
    pub fn get(&self, key: &SlotId) -> Option<&SlotTypePosition> {
        self.0.get(key)
    }

    /// Iterate over all SlotTypePositionAnnotations
    pub fn iter(&self) -> impl Iterator<Item = (&SlotId, &SlotTypePosition)> {
        self.0.iter()
    }

    /// Tell if it's empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for SlotTypePositionAnnotations {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<(SlotId, SlotTypePosition)> for SlotTypePositionAnnotations {
    fn from_iter<T: IntoIterator<Item = (SlotId, SlotTypePosition)>>(iter: T) -> Self {
        Self(BTreeMap::from_iter(iter))
    }
}

impl From<BTreeMap<SlotId, SlotTypePosition>> for SlotTypePositionAnnotations {
    fn from(value: BTreeMap<SlotId, SlotTypePosition>) -> Self {
        Self(value)
    }
}

// We need to include the position of the slot so when we are typechecking we can use the type that is stored
// with that position since otherwise we would not know which type to use

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
    t: Option<Type<RawName>>,
    position: Option<ScopePosition>,
}

impl SlotTypePosition {
    /// Create a new slot type position
    pub fn new(t: Option<Type<RawName>>, position: Option<ScopePosition>) -> Self {
        Self { t, position }
    }
}
