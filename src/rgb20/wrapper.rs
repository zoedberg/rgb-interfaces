// RGB interfaces by LNP/BP Standards Association
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rgbstd::interface::{
    AssignmentsFilter, ContractIface, ContractOp, FungibleAllocation, IfaceClass, IfaceId,
    IfaceWrapper, RightsAllocation,
};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{AssetSpec, ContractTerms, Details};
use rgbstd::{ContractId, SchemaId, Txid, WitnessInfo};
use strict_encoding::InvalidRString;

use super::{Inflation, PrimaryIssue, Rgb20, Rgb20Info};
use crate::IssuerWrapper;

pub const RGB20_FIXED_IFACE_ID: IfaceId = IfaceId::from_array([
    0x9a, 0x9c, 0x75, 0x65, 0xab, 0x09, 0x78, 0x0e, 0xeb, 0xb5, 0x7b, 0x12, 0x7c, 0x47, 0x47, 0xcb,
    0x80, 0x1b, 0xc4, 0xbc, 0x99, 0x00, 0x3e, 0xa5, 0xd5, 0x62, 0x2a, 0x34, 0x63, 0x2a, 0x31, 0x0d,
]);
pub const RGB20_RENAMABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x46, 0x36, 0x53, 0x99, 0xba, 0xd5, 0x41, 0x06, 0xe9, 0x98, 0x21, 0x7b, 0x81, 0x05, 0x57, 0x43,
    0x44, 0xfa, 0xda, 0xe6, 0x5a, 0xd2, 0xec, 0xd9, 0x1e, 0xa7, 0xc1, 0x9a, 0xcf, 0xd9, 0x83, 0xac,
]);
pub const RGB20_INFLATABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xf6, 0x87, 0x32, 0x54, 0xf6, 0xd1, 0xbd, 0x87, 0x4c, 0xac, 0xd5, 0xbf, 0xe8, 0x3d, 0x47, 0xed,
    0xfa, 0xc3, 0x2a, 0xfb, 0x3f, 0x1c, 0xc3, 0x5f, 0xbd, 0xc5, 0xfb, 0x78, 0xa7, 0x8a, 0x4f, 0x16,
]);
pub const RGB20_INFLATABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x3a, 0x14, 0xd6, 0xad, 0x94, 0x35, 0xc1, 0xe0, 0x10, 0x71, 0xcf, 0xdf, 0x07, 0x15, 0x25, 0x0b,
    0xa7, 0x8c, 0x73, 0xf2, 0xf6, 0xf7, 0x41, 0x60, 0x04, 0xb6, 0x38, 0x6c, 0x54, 0xdc, 0xec, 0xb3,
]);
pub const RGB20_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x7f, 0xe0, 0x3c, 0xb1, 0xa9, 0x54, 0x84, 0xaa, 0x21, 0xc4, 0xaa, 0x1c, 0xd3, 0x9e, 0x51, 0x28,
    0x2a, 0x10, 0xf1, 0xc7, 0x41, 0xaa, 0x61, 0xb4, 0x33, 0x93, 0x2f, 0xb8, 0x10, 0xb5, 0x9a, 0x9e,
]);
pub const RGB20_RENAMABLE_INFLATABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xab, 0x23, 0xbb, 0x04, 0x98, 0xba, 0x70, 0x01, 0x18, 0x39, 0x12, 0xa4, 0x52, 0x26, 0x47, 0x06,
    0x02, 0xb1, 0xda, 0x41, 0x9e, 0x58, 0xca, 0x04, 0xb1, 0x48, 0x3b, 0xc5, 0x59, 0xeb, 0xc2, 0x2d,
]);
pub const RGB20_REPLACABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x65, 0x0d, 0xc2, 0xae, 0x09, 0x0e, 0xfb, 0x7a, 0x4d, 0xe5, 0x09, 0x74, 0x68, 0x85, 0x8b, 0x6b,
    0xc3, 0x79, 0xde, 0x93, 0xf2, 0x28, 0xd3, 0xe8, 0x1e, 0x10, 0x2d, 0x1a, 0xaa, 0x9f, 0x1c, 0x41,
]);
pub const RGB20_RENAMABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x7c, 0xd0, 0x3a, 0xc6, 0xbc, 0x2c, 0xc9, 0x18, 0x60, 0xef, 0x9d, 0xf0, 0xde, 0xb9, 0xa6, 0x9b,
    0x16, 0xc8, 0x0c, 0xfe, 0x56, 0xc1, 0xf0, 0x61, 0xf1, 0x56, 0x8d, 0x9c, 0x3f, 0x6c, 0x9c, 0xdf,
]);
pub const RGB20_RENAMABLE_INFLATABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xaf, 0x55, 0x4a, 0x38, 0x9f, 0x1f, 0xbf, 0x58, 0x3a, 0x98, 0x92, 0x29, 0xb2, 0x27, 0x9c, 0xb9,
    0xdf, 0x75, 0xaa, 0x78, 0x3f, 0xd8, 0x51, 0x68, 0xd9, 0xb9, 0x5d, 0x86, 0x28, 0x43, 0x5d, 0xe2,
]);
pub const RGB20_FULL_IFACE_ID: IfaceId = IfaceId::from_array([
    0xfc, 0x76, 0x7c, 0xaa, 0xf9, 0x2d, 0xa4, 0xfe, 0xcc, 0x22, 0xd7, 0x23, 0x51, 0x0b, 0xe9, 0xf6,
    0xed, 0x33, 0xc3, 0x27, 0x49, 0xe2, 0x23, 0x46, 0x29, 0x4e, 0xf3, 0x9a, 0x48, 0x2b, 0xd9, 0x85,
]);

#[derive(Clone, Eq, PartialEq, Debug, From)]
pub struct Rgb20Wrapper<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> IfaceWrapper<S> for Rgb20Wrapper<S> {
    type Info = Rgb20Info;

    fn with(iface: ContractIface<S>) -> Self {
        if !Rgb20::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB20 interface");
        }
        Self(iface)
    }

    fn info(&self) -> Self::Info {
        let spec = self.spec();
        let terms = self.contract_terms();
        Rgb20Info {
            contract: self.0.info.clone(),
            ticker: spec.ticker.to_string(),
            name: spec.name.to_string(),
            details: spec.details.as_ref().map(Details::to_string),
            terms: terms.text.to_string(),
            attach: terms.media,
            precision: spec.precision,
            features: self.features(),
            issued: self.total_issued_supply(),
            burned: self.total_burned_supply(),
            replaced: self.total_replaced_supply(),
        }
    }

    #[inline]
    fn contract_id(&self) -> ContractId { self.0.contract_id() }

    #[inline]
    fn schema_id(&self) -> SchemaId { self.0.state.schema_id() }

    #[inline]
    fn witness_info(&self, witness_id: Txid) -> Option<WitnessInfo> {
        self.0.witness_info(witness_id)
    }
}

impl<S: ContractStateRead> Rgb20Wrapper<S> {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb20>>(
        issuer: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet::<C>(issuer, ticker, name, details, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb20>>(
        issuer: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet_det::<C>(issuer, ticker, name, details, precision)
    }

    pub fn features(&self) -> Rgb20 {
        let renaming = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "rename");
        let inflatable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "issue");
        let burnable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "burn");
        let replaceable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "replace");

        let inflation = match (inflatable, burnable, replaceable) {
            (true, true, true) => Inflation::Replaceable,
            (true, true, false) => Inflation::InflatableBurnable,
            (false, true, false) => Inflation::Burnable,
            (true, false, false) => Inflation::Inflatable,
            (false, false, false) => Inflation::Fixed,
            (true, false, true) | (false, false, true) => {
                panic!("replaceable asset with no burn enabled")
            }
            (false, true, true) => panic!("replaceable but non-inflatible asset"),
        };

        Rgb20 {
            renaming,
            inflation,
        }
    }

    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB20 interface requires global state `spec`")
            .next()
            .expect("RGB20 interface requires global state `spec` to have at least one item");
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn balance(&self, filter: impl AssignmentsFilter) -> Amount {
        self.allocations(filter)
            .map(|alloc| alloc.state)
            .sum::<Amount>()
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("assetOwner", filter)
            .expect("RGB20 interface requires `assetOwner` state")
    }

    pub fn inflation_allowance_allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("inflationAllowance", filter)
            .expect("RGB20 interface requires `inflationAllowance` state")
    }

    pub fn update_right<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("updateRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn burn_epoch<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnEpoch", filter)
            .expect("RGB20 interface requires `burnEpoch` state")
    }

    pub fn burn_right<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB20 interface requires global `terms`")
            .next()
            .expect("RGB20 interface requires global state `terms` to have at least one item");
        ContractTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB20 interface requires global `issuedSupply`")
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    // Max supply for the inflation asset, if there is no `max supply`, then it will
    // default to the non-inflatable asset `issued supply`
    pub fn max_supply(&self) -> Amount {
        self.0
            .global("maxSupply")
            .unwrap_or_else(|_| {
                self.0
                    .global("issuedSupply")
                    .expect("RGB20 interface requires global `issuedSupply`")
            })
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .into_iter()
            .flatten()
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    pub fn total_replaced_supply(&self) -> Amount {
        self.0
            .global("replacedSupply")
            .into_iter()
            .flatten()
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    pub fn total_supply(&self) -> Amount { self.total_issued_supply() - self.total_burned_supply() }

    pub fn history(
        &self,
        filter_outpoints: impl AssignmentsFilter + Clone,
        filter_witnesses: impl AssignmentsFilter + Clone,
    ) -> Vec<ContractOp> {
        self.0.history(filter_outpoints, filter_witnesses)
    }
}
