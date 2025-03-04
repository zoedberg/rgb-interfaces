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
    AssignmentsFilter, ContractIface, FungibleAllocation, IfaceClass, IfaceId, IfaceWrapper,
};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{ContractTerms, Details, Name};
use rgbstd::{ContractId, SchemaId, Txid, WitnessInfo};
use strict_encoding::InvalidRString;

use super::{Issue, Rgb25, Rgb25Info};
use crate::IssuerWrapper;

pub const RGB25_BASE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x96, 0x85, 0x74, 0xcd, 0x51, 0x94, 0x32, 0x7d, 0x5e, 0x40, 0xfa, 0x7b, 0x4c, 0xba, 0xbb, 0x7a,
    0x22, 0x00, 0x1c, 0x0c, 0xbe, 0xfe, 0x40, 0x0b, 0x7d, 0xd7, 0x1e, 0x5a, 0x69, 0x3f, 0x89, 0x9e,
]);

pub const RGB25_IFACE_ID: IfaceId = IfaceId::from_array([
    0x7f, 0x0b, 0xb8, 0xe8, 0xfe, 0x3e, 0xda, 0x6d, 0x38, 0x10, 0xed, 0x1c, 0x4b, 0xc6, 0x19, 0x2c,
    0xe4, 0x8f, 0xc1, 0x28, 0x53, 0x1d, 0x4a, 0x4c, 0xb5, 0x77, 0x97, 0xea, 0x40, 0x46, 0x3c, 0x31,
]);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rgb25Wrapper<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> IfaceWrapper<S> for Rgb25Wrapper<S> {
    type Info = Rgb25Info;

    fn with(iface: ContractIface<S>) -> Self {
        if !Rgb25::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB25 interface");
        }
        Self(iface)
    }

    fn info(&self) -> Self::Info { todo!() }

    #[inline]
    fn contract_id(&self) -> ContractId { self.0.contract_id() }

    #[inline]
    fn schema_id(&self) -> SchemaId { self.0.state.schema_id() }

    #[inline]
    fn witness_info(&self, witness_id: Txid) -> Option<WitnessInfo> {
        self.0.witness_info(witness_id)
    }
}

impl<S: ContractStateRead> Rgb25Wrapper<S> {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb25>>(
        issuer: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet::<C>(issuer, name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb25>>(
        issuer: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet_det::<C>(issuer, name, precision)
    }

    pub fn name(&self) -> Name {
        let strict_val = &self
            .0
            .global("name")
            .expect("RGB25 interface requires global `name`")
            .next()
            .expect("RGB25 interface requires global state `name`");
        Name::from_strict_val_unchecked(strict_val)
    }

    pub fn details(&self) -> Option<Details> {
        self.0
            .global("details")
            .expect("RGB25 interface requires global state `details`")
            .next()
            .map(|strict_val| Details::from_strict_val_unchecked(&strict_val))
    }

    pub fn precision(&self) -> Precision {
        let strict_val = &self
            .0
            .global("precision")
            .expect("RGB25 interface requires global state `precision`")
            .next()
            .expect("RGB25 interface requires global state `precision` to have at least one item");
        Precision::from_strict_val_unchecked(strict_val)
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("assetOwner", filter)
            .expect("RGB25 interface requires `assetOwner` state")
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB25 interface requires global state `issuedSupply`")
            .map(|strict_val| Amount::from_strict_val_unchecked(&strict_val))
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .into_iter()
            .flatten()
            .map(|strict_val| Amount::from_strict_val_unchecked(&strict_val))
            .sum()
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB25 interface requires global state `terms`")
            .next()
            .expect("RGB25 interface requires global state `terms` to have at least one item");
        ContractTerms::from_strict_val_unchecked(strict_val)
    }
}
