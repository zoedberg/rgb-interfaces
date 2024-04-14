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

use std::collections::HashMap;

use rgbstd::interface::{
    AmountChange, ContractIface, FungibleAllocation, Iface, IfaceId, IfaceOp, OutpointFilter,
    RightsAllocation, WitnessFilter,
};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::stl::{rgb_std_stl, AssetSpec, AssetTerms};
use rgbstd::{AssetTag, XWitnessId};
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::iface::*;
use super::{Features, PrimaryIssue};
use crate::{IfaceWrapper, IssuerWrapper};

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct Rgb20(ContractIface);

impl From<ContractIface> for Rgb20 {
    fn from(iface: ContractIface) -> Self {
        if !Rgb20::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB20 interface");
        }
        Self(iface)
    }
}

impl IfaceWrapper for Rgb20 {
    const IFACE_NAME: &'static str = "RGB20";
    const IFACE_IDS: &'static [IfaceId] = &[
        // simple
        IfaceId::from_array([
            0x58, 0x8d, 0xaf, 0x57, 0x88, 0x37, 0xe0, 0x42, 0xa1, 0x35, 0xd9, 0x4a, 0x20, 0x8b,
            0x2a, 0x71, 0xa2, 0x2d, 0x79, 0xd5, 0x62, 0xba, 0x35, 0x83, 0x98, 0xa5, 0x8c, 0xe6,
            0x6b, 0xeb, 0xf4, 0x87,
        ]),
        // all
        IfaceId::from_array([
            0x67, 0xef, 0x0c, 0x35, 0xab, 0xef, 0x01, 0x88, 0x56, 0xf5, 0xe8, 0x51, 0xaf, 0x96,
            0xc2, 0xab, 0xdd, 0xff, 0x64, 0x31, 0xec, 0xce, 0x73, 0xf0, 0x12, 0x1e, 0xe3, 0x64,
            0xcc, 0xf5, 0xca, 0x69,
        ]),
    ];

    type Features = Features;
    fn iface(features: Features) -> Iface {
        let mut iface = named_asset().expect_extended(fungible(), "RGB20Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB20Renamable");
        }
        if features.inflation.is_fixed() {
            iface = iface.expect_extended(fixed(), "RGB20Fixed");
        } else if features.inflation.is_inflatible() {
            iface = iface.expect_extended(inflatable(), "RGB20Inflatible");
        }
        if features.inflation.is_replacable() {
            iface = iface.expect_extended(replaceable(), "RGB20Replacable");
        } else if features.inflation.is_burnable() {
            iface = iface.expect_extended(burnable(), "RGB20Burnable");
        }
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB20Reservable");
        }
        if features == Features::ALL {
            iface.name = Self::IFACE_NAME.into();
        }
        iface
    }

    fn stl() -> TypeLib { rgb_std_stl() }
}

impl Rgb20 {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet::<C>(ticker, name, details, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet_det::<C>(ticker, name, details, precision, asset_tag)
    }

    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB20 interface requires global state `spec`")[0];
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn balance(&self, filter: impl OutpointFilter) -> Amount {
        self.allocations(filter)
            .map(|alloc| alloc.state)
            .sum::<Amount>()
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("assetOwner", filter)
            .expect("RGB20 interface requires `assetOwner` state")
    }

    pub fn inflation_allowance_allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("inflationAllowance", filter)
            .expect("RGB20 interface requires `inflationAllowance` state")
    }

    pub fn update_right<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("updateRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn burn_epoch<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnEpoch", filter)
            .expect("RGB20 interface requires `burnEpoch` state")
    }

    pub fn burn_right<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn contract_terms(&self) -> AssetTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB20 interface requires global `terms`")[0];
        AssetTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB20 interface requires global `issuedSupply`")
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .unwrap_or_default()
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_replaced_supply(&self) -> Amount {
        self.0
            .global("replacedSupply")
            .unwrap_or_default()
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_supply(&self) -> Amount { self.total_issued_supply() - self.total_burned_supply() }

    pub fn transfer_history(
        &self,
        witness_filter: impl WitnessFilter + Copy,
        outpoint_filter: impl OutpointFilter + Copy,
    ) -> HashMap<XWitnessId, IfaceOp<AmountChange>> {
        self.0
            .fungible_ops("assetOwner", witness_filter, outpoint_filter)
            .expect("state name is not correct")
    }
}
