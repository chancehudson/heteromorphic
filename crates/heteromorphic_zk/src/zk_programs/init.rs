use std::sync::OnceLock;

use sp1_sdk::HashableKey;
use sp1_sdk::ProverClient;
use zkpo::prelude::*;

pub struct ZKInitProgram;
impl ZKProgram for ZKInitProgram {
    fn id(&self) -> &[u8; 32] {
        static HASH: OnceLock<[u8; 32]> = OnceLock::new();
        HASH.get_or_init(|| {
            let client = ProverClient::from_env();
            let (_pk, vk) = client.setup(self.elf());
            vk.hash_bytes()
        })
    }

    fn elf(&self) -> &[u8] {
        include_bytes!("../../elf/init")
    }

    fn name(&self) -> Option<&str> {
        Some("init data")
    }

    fn agent(&self) -> &dyn ZKAgent {
        ZKSPOneAgent::singleton()
    }
}
