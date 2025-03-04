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
    AssignmentsFilter, ContractIface, ContractOp, DataAllocation, IfaceClass, IfaceId, IfaceWrapper,
};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{bp_tx_stl, rgb_contract_stl, AssetSpec, ContractTerms};
use rgbstd::{Allocation, ContractId, SchemaId, Txid, WitnessInfo};
use strict_types::stl::std_stl;
use strict_types::{CompileError, LibBuilder, TypeLib};

use super::{AttachmentType, EngravingData, ItemsCount, Rgb21, TokenData, LIB_NAME_RGB21};
use crate::rgb20::Rgb20Info;

pub const RGB21_UNIQUE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x62, 0x3f, 0x7e, 0xfe, 0xb2, 0x47, 0x48, 0x6d, 0x37, 0x8a, 0x9c, 0xb4, 0xbe, 0x88, 0x4b, 0x31,
    0xd3, 0x53, 0x8e, 0xf3, 0x6d, 0xd1, 0xfd, 0xad, 0x1d, 0x38, 0x84, 0x78, 0x07, 0x99, 0xb3, 0xeb,
]);

pub const RGB21_IFACE_ID: IfaceId = IfaceId::from_array([
    0x69, 0x43, 0xe5, 0xa0, 0x2a, 0xcd, 0x2b, 0x96, 0x0b, 0xb6, 0xbf, 0x4c, 0xcc, 0x9d, 0x99, 0xce,
    0xb9, 0xac, 0x73, 0x63, 0x94, 0xda, 0x94, 0x93, 0xd4, 0xe1, 0x08, 0x3a, 0x1f, 0xc0, 0x00, 0xc7,
]);

fn _rgb21_stl() -> Result<TypeLib, CompileError> {
    LibBuilder::new(libname!(LIB_NAME_RGB21), tiny_bset! {
        std_stl().to_dependency(),
        bp_tx_stl().to_dependency(),
        rgb_contract_stl().to_dependency()
    })
    .transpile::<TokenData>()
    .transpile::<EngravingData>()
    .transpile::<ItemsCount>()
    .transpile::<Allocation>()
    .transpile::<AttachmentType>()
    .compile()
}

/// Generates strict type library providing data types for RGB21 interface.
pub fn rgb21_stl() -> TypeLib { _rgb21_stl().expect("invalid strict type RGB21 library") }

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rgb21Wrapper<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> IfaceWrapper<S> for Rgb21Wrapper<S> {
    type Info = Rgb20Info;

    fn with(iface: ContractIface<S>) -> Self {
        if !Rgb21::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB21 interface");
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

impl<S: ContractStateRead> Rgb21Wrapper<S> {
    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB21 interface requires global `spec`")
            .next()
            .expect("RGB21 interface requires global state `spec` to have at least one item");
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB21 interface requires global `terms`")
            .next()
            .expect("RGB21 interface requires global state `terms` to have at least one item");
        ContractTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn token_data(&self) -> TokenData {
        let strict_val = &self
            .0
            .global("tokens")
            .expect("RGB21 interface requires global `tokens`")
            .next()
            .expect("RGB21 interface requires global state `tokens` to have at least one item");
        TokenData::from_strict_val_unchecked(strict_val)
    }

    pub fn engraving_data(&self) -> impl Iterator<Item = EngravingData> + '_ {
        self.0
            .global("engravings")
            .expect("RGB21 interface requires global state `engravings`")
            .map(|strict_val| EngravingData::from_strict_val_unchecked(&strict_val))
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = DataAllocation> + 'c {
        self.0
            .data("assetOwner", filter)
            .expect("RGB21 interface requires `assetOwner` state")
    }

    pub fn history(
        &self,
        filter_outpoints: impl AssignmentsFilter + Clone,
        filter_witnesses: impl AssignmentsFilter + Clone,
    ) -> Vec<ContractOp> {
        self.0.history(filter_outpoints, filter_witnesses)
    }
}
