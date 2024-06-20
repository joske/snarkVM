// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use aleo_std::StorageMode;
use console::{
    network::{CanaryV0, MainnetV0},
    program::{FromBytes, Network},
};
use ledger_block::Block;
use ledger_store::helpers::rocksdb::ConsensusDB;
use snarkvm_ledger::Ledger;
use tracing::info;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

// type CurrentNetwork = MainnetV0;
// const STORAGE_MODE: StorageMode = StorageMode::Development(0);
type CurrentNetwork = CanaryV0;
const STORAGE_MODE: StorageMode = StorageMode::Production;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::default().with_filter(EnvFilter::from_default_env()))
        .init();
    // Initialize the genesis block.
    info!("ledger loading");
    let genesis = Block::<CurrentNetwork>::read_le(CurrentNetwork::genesis_bytes()).unwrap();
    // Initialize the ledger.
    let _ledger = Ledger::<CurrentNetwork, ConsensusDB<CurrentNetwork>>::load(genesis, STORAGE_MODE).unwrap();
    info!("ledger loaded");
    std::thread::sleep(std::time::Duration::from_secs(120));
}
