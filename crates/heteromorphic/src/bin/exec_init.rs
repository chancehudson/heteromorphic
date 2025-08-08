use heteromorphic_zk::prelude::*;
use zkpo::prelude::*;

fn main() -> anyhow::Result<()> {
    let key = [0u8; 32];
    let mem_len = 32;
    let nonce = [0u8; 12];

    let input = (key, mem_len, nonce);
    println!("Initializing {mem_len} bytes in chacha in zk...");
    let exe = ZKInitProgram.execute(&bincode::serialize(&input)?, None)?;
    println!("Executed.");
    println!("Verifying....");

    let output = ZKInitProgram.agent().verify(&*exe);
    println!("verified init data: {output:?}");
    Ok(())
}
