use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    zkpo::sp1::build(&["init"], &[], true, Some(&PathBuf::from("elf/")))
}
