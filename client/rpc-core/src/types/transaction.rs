// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2015-2020 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::{sync::{Arc, Mutex}, collections::HashMap};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use ethereum_types::{H160, H256, H512, U64, U256};
use ethereum::{AccessListItem, TransactionV0, TransactionV1, TransactionV2};
use sha3::{Keccak256, Digest};
use crate::types::Bytes;

/// Transaction
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	/// Hash
	pub hash: H256,
	/// Nonce
	pub nonce: U256,
	/// Block hash
	pub block_hash: Option<H256>,
	/// Block number
	pub block_number: Option<U256>,
	/// Transaction Index
	pub transaction_index: Option<U256>,
	/// Sender
	pub from: H160,
	/// Recipient
	pub to: Option<H160>,
	/// Transfered value
	pub value: U256,
	/// Gas Price
	#[cfg_attr(feature = "std", serde(skip_serializing_if = "Option::is_none"))]
	pub gas_price: Option<U256>,
	/// Max BaseFeePerGas the user is willing to pay.
	#[cfg_attr(feature = "std", serde(skip_serializing_if = "Option::is_none"))]
	pub max_fee_per_gas: Option<U256>,
	/// The miner's tip.
	#[cfg_attr(feature = "std", serde(skip_serializing_if = "Option::is_none"))]
	pub max_priority_fee_per_gas: Option<U256>,
	/// Gas
	pub gas: U256,
	/// Data
	pub input: Bytes,
	/// Creates contract
	pub creates: Option<H160>,
	/// Raw transaction data
	pub raw: Bytes,
	/// Public key of the signer.
	pub public_key: Option<H512>,
	/// The network id of the transaction, if any.
	pub chain_id: Option<U64>,
	/// The standardised V field of the signature (0 or 1).
	pub standard_v: U256,
	/// The standardised V field of the signature.
	pub v: U256,
	/// The R field of the signature.
	pub r: U256,
	/// The S field of the signature.
	pub s: U256,
	/// TODO! Pre-pay to warm storage access.
	#[cfg_attr(feature = "std", serde(skip_serializing_if = "Option::is_none"))]
	pub access_list: Option<Vec<AccessListItem>>,
}

impl From<TransactionV0> for Transaction {
	fn from(transaction: TransactionV0) -> Self {
		let serialized = {
			let envelope = ethereum::Transaction::V0(transaction.clone());
			envelope.serialize()
		};
		Transaction {
			hash: H256::from_slice(
				Keccak256::digest(&serialized).as_slice()
			),
			nonce: transaction.nonce,
			block_hash: None,
			block_number: None,
			transaction_index: None,
			from: H160::default(),
			to: None,
			value: transaction.value,
			gas_price: Some(transaction.gas_price),
			max_fee_per_gas: None,
			max_priority_fee_per_gas: None,
			gas: transaction.gas_limit,
			input: Bytes(transaction.clone().input),
			creates: None,
			raw: Bytes(serialized.to_vec()),
			public_key: None,
			chain_id: transaction.signature.chain_id().map(U64::from),
			standard_v: U256::from(transaction.signature.standard_v()),
			v: U256::from(transaction.signature.v()),
			r: U256::from(transaction.signature.r().as_bytes()),
			s: U256::from(transaction.signature.s().as_bytes()),
			access_list: None,
		}
	}
}
impl From<TransactionV1> for Transaction {
	fn from(transaction: TransactionV1) -> Self {
		let serialized = {
			let envelope = ethereum::Transaction::V1(transaction.clone());
			envelope.serialize()
		};
		Transaction {
			hash: H256::from_slice(
				Keccak256::digest(&serialized).as_slice()
			),
			nonce: transaction.nonce,
			block_hash: None,
			block_number: None,
			transaction_index: None,
			from: H160::default(),
			to: None,
			value: transaction.value,
			gas_price: Some(transaction.gas_price),
			max_fee_per_gas: None,
			max_priority_fee_per_gas: None,
			gas: transaction.gas_limit,
			input: Bytes(transaction.clone().input),
			creates: None,
			raw: Bytes(serialized.to_vec()),
			public_key: None,
			chain_id: Some(U64::from(transaction.chain_id)),
			standard_v: U256::from(transaction.odd_y_parity as u8),
			v: U256::from(transaction.odd_y_parity as u8), // TODO
			r: U256::from(transaction.r.as_bytes()),
			s: U256::from(transaction.s.as_bytes()),
			access_list: Some(transaction.access_list),
		}
	}
}
impl From<TransactionV2> for Transaction {
	fn from(transaction: TransactionV2) -> Self {
		let serialized = {
			let envelope = ethereum::Transaction::V2(transaction.clone());
			envelope.serialize()
		};
		Transaction {
			hash: H256::from_slice(
				Keccak256::digest(&serialized).as_slice()
			),
			nonce: transaction.nonce,
			block_hash: None,
			block_number: None,
			transaction_index: None,
			from: H160::default(),
			to: None,
			value: transaction.value,
			gas_price: None,
			max_fee_per_gas: Some(transaction.max_fee_per_gas),
			max_priority_fee_per_gas: Some(transaction.max_priority_fee_per_gas),
			gas: transaction.gas_limit,
			input: Bytes(transaction.clone().input),
			creates: None,
			raw: Bytes(serialized.to_vec()),
			public_key: None,
			chain_id: Some(U64::from(transaction.chain_id)),
			standard_v: U256::from(transaction.odd_y_parity as u8),
			v: U256::from(transaction.odd_y_parity as u8), // TODO
			r: U256::from(transaction.r.as_bytes()),
			s: U256::from(transaction.s.as_bytes()),
			access_list: Some(transaction.access_list),
		}
	}
}

/// Local Transaction Status
#[derive(Debug)]
pub enum LocalTransactionStatus {
	/// Transaction is pending
	Pending,
	/// Transaction is in future part of the queue
	Future,
	/// Transaction was mined.
	Mined(Transaction),
	/// Transaction was removed from the queue, but not mined.
	Culled(Transaction),
	/// Transaction was dropped because of limit.
	Dropped(Transaction),
	/// Transaction was replaced by transaction with higher gas price.
	Replaced(Transaction, U256, H256),
	/// Transaction never got into the queue.
	Rejected(Transaction, String),
	/// Transaction is invalid.
	Invalid(Transaction),
	/// Transaction was canceled.
	Canceled(Transaction),
}

impl Serialize for LocalTransactionStatus {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer
	{
		use self::LocalTransactionStatus::*;

		let elems = match *self {
			Pending | Future => 1,
			Mined(..) | Culled(..) | Dropped(..) | Invalid(..) | Canceled(..) => 2,
			Rejected(..) => 3,
			Replaced(..) => 4,
		};

		let status = "status";
		let transaction = "transaction";

		let mut struc = serializer.serialize_struct("LocalTransactionStatus", elems)?;
		match *self {
			Pending => struc.serialize_field(status, "pending")?,
			Future => struc.serialize_field(status, "future")?,
			Mined(ref tx) => {
				struc.serialize_field(status, "mined")?;
				struc.serialize_field(transaction, tx)?;
			},
			Culled(ref tx) => {
				struc.serialize_field(status, "culled")?;
				struc.serialize_field(transaction, tx)?;
			},
			Dropped(ref tx) => {
				struc.serialize_field(status, "dropped")?;
				struc.serialize_field(transaction, tx)?;
			},
			Canceled(ref tx) => {
				struc.serialize_field(status, "canceled")?;
				struc.serialize_field(transaction, tx)?;
			},
			Invalid(ref tx) => {
				struc.serialize_field(status, "invalid")?;
				struc.serialize_field(transaction, tx)?;
			},
			Rejected(ref tx, ref reason) => {
				struc.serialize_field(status, "rejected")?;
				struc.serialize_field(transaction, tx)?;
				struc.serialize_field("error", reason)?;
			},
			Replaced(ref tx, ref gas_price, ref hash) => {
				struc.serialize_field(status, "replaced")?;
				struc.serialize_field(transaction, tx)?;
				struc.serialize_field("hash", hash)?;
				struc.serialize_field("gasPrice", gas_price)?;
			},
		}

		struc.end()
	}
}

/// Geth-compatible output for eth_signTransaction method
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct RichRawTransaction {
	/// Raw transaction RLP
	pub raw: Bytes,
	/// Transaction details
	#[serde(rename = "tx")]
	pub transaction: Transaction
}

pub struct PendingTransaction {
	pub transaction: Transaction,
	pub at_block: u64
}

impl PendingTransaction {
	pub fn new(transaction: Transaction, at_block: u64) -> Self {
		Self { transaction, at_block }
	}
}

pub type PendingTransactions = Option<Arc<Mutex<HashMap<H256, PendingTransaction>>>>;
