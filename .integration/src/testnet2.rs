// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::Ledger;
use snarkvm_algorithms::{MerkleParameters, CRH};
use snarkvm_dpc::{
    testnet2::{instantiated::*, parameters::PublicParameters},
    Account,
    AccountScheme,
    DPCScheme,
    Storage,
};
use snarkvm_parameters::{LedgerMerkleTreeParameters, Parameter};
use snarkvm_utilities::bytes::FromBytes;

use rand::{CryptoRng, Rng};
use std::sync::Arc;

pub type MerkleTreeLedger<S> = Ledger<Testnet2Transaction, CommitmentMerkleParameters, S>;

pub fn setup_or_load_parameters<R: Rng + CryptoRng, S: Storage>(
    verify_only: bool,
    rng: &mut R,
) -> (
    Arc<CommitmentMerkleParameters>,
    <Testnet2DPC as DPCScheme<MerkleTreeLedger<S>>>::NetworkParameters,
) {
    // TODO (howardwu): Resolve this inconsistency on import structure with a new model once MerkleParameters are refactored.
    let crh_parameters =
        <MerkleTreeCRH as CRH>::Parameters::read(&LedgerMerkleTreeParameters::load_bytes().unwrap()[..])
            .expect("read bytes as hash for MerkleParameters in ledger");
    let merkle_tree_hash_parameters = <CommitmentMerkleParameters as MerkleParameters>::H::from(crh_parameters);
    let ledger_merkle_tree_parameters = Arc::new(From::from(merkle_tree_hash_parameters));

    // let parameters = match <InstantiatedDPC as DPCScheme<MerkleTreeLedger<S>>>::NetworkParameters::load(verify_only) {
    //     Ok(parameters) => parameters,
    //     Err(err) => {
    //         println!("error - {}, re-running parameter Setup", err);
    //         <InstantiatedDPC as DPCScheme<MerkleTreeLedger<S>>>::setup(&ledger_merkle_tree_parameters, rng)
    //             .expect("DPC setup failed")
    //     }
    // };

    let parameters = <Testnet2DPC as DPCScheme<MerkleTreeLedger<S>>>::setup(&ledger_merkle_tree_parameters, rng)
        .expect("DPC setup failed");

    (ledger_merkle_tree_parameters, parameters)
}

pub fn generate_test_accounts<R: Rng + CryptoRng, S: Storage>(
    parameters: &PublicParameters<Components>,
    rng: &mut R,
) -> [Account<Components>; 3] {
    let signature_parameters = &parameters.system_parameters.account_signature;
    let commitment_parameters = &parameters.system_parameters.account_commitment;
    let encryption_parameters = &parameters.system_parameters.account_encryption;

    let genesis_account =
        Account::new(signature_parameters, commitment_parameters, encryption_parameters, rng).unwrap();
    let account_1 = Account::new(signature_parameters, commitment_parameters, encryption_parameters, rng).unwrap();
    let account_2 = Account::new(signature_parameters, commitment_parameters, encryption_parameters, rng).unwrap();

    [genesis_account, account_1, account_2]
}
