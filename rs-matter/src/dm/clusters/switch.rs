/*
 *
 *    Copyright (c) 2020-2022 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

//! Implementation of the Matter Generic Switch cluster (0x003B).
//!
//! Exposes a momentary switch endpoint. One `SwitchHandler` instance represents
//! one physical switch (or button group, with `num_positions > 2`).
//!
//! # Usage
//!
//! 1. Instantiate with `SwitchHandler::new(dataver, endpoint_id, num_positions)`.
//! 2. Wire into a bridged endpoint alongside `DescHandler` and
//!    `BridgedDeviceBasicInformation`.
//! 3. Call `set_position(pos)` from within the same executor thread to reflect
//!    physical state changes, then call `ctx.notify_attribute_changed()` to wake
//!    any attribute subscribers.
//!
//! # Events
//!
//! `InitialPress` (0x00) and `ShortRelease` (0x03) events are not yet implemented —
//! rs-matter's event-push API requires a `KvBlobStoreAccess` reference that is not
//! currently exposed via `HandlerContext`. Can be added once that plumbing exists.

use core::cell::Cell;

use crate::dm::clusters::decl::switch;
use crate::dm::types::EndptId;
use crate::dm::{Cluster, Dataver, ReadContext};
use crate::error::Error;
use crate::with;

pub use crate::dm::clusters::decl::switch::*;

/// Handler for the Matter Generic Switch cluster (0x003B).
///
/// Implements a momentary switch with `num_positions` positions:
/// - position 0 = idle / not pressed
/// - positions 1..num_positions-1 = active positions
///
/// `FeatureMap = MOMENTARY_SWITCH | MOMENTARY_SWITCH_RELEASE`
/// `MultiPressMax = 1`
pub struct SwitchHandler {
    dataver: Dataver,
    endpoint_id: EndptId,
    num_positions: u8,
    current_position: Cell<u8>,
}

impl SwitchHandler {
    pub fn new(dataver: Dataver, endpoint_id: EndptId, num_positions: u8) -> Self {
        assert!(num_positions >= 2, "num_positions must be at least 2");
        Self {
            dataver,
            endpoint_id,
            num_positions,
            current_position: Cell::new(0),
        }
    }

    /// Update `CurrentPosition` and bump the dataver.
    ///
    /// Call `ctx.notify_attribute_changed(endpoint_id, CLUSTER_ID, CurrentPosition)` after
    /// this to wake attribute subscribers.
    pub fn set_position(&self, pos: u8) {
        self.current_position.set(pos.min(self.num_positions - 1));
        self.dataver.changed();
    }

    /// The cluster ID for use in `notify_attribute_changed` calls.
    pub const CLUSTER_ID: u32 = switch::FULL_CLUSTER.id;
}

impl switch::ClusterHandler for SwitchHandler {
    const CLUSTER: Cluster<'static> = switch::FULL_CLUSTER
        .with_revision(1)
        .with_features(
            switch::Feature::MOMENTARY_SWITCH.bits()
                | switch::Feature::MOMENTARY_SWITCH_RELEASE.bits(),
        )
        .with_attrs(with!(
            required;
            switch::AttributeId::NumberOfPositions
                | switch::AttributeId::CurrentPosition
        ));

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.dataver.changed();
    }

    fn number_of_positions(&self, _ctx: impl ReadContext) -> Result<u8, Error> {
        Ok(self.num_positions)
    }

    fn current_position(&self, _ctx: impl ReadContext) -> Result<u8, Error> {
        Ok(self.current_position.get())
    }

    fn multi_press_max(&self, _ctx: impl ReadContext) -> Result<u8, Error> {
        Ok(1)
    }
}
