#[cfg(not(target_os = "zkvm"))]
pub mod zk_programs;

#[cfg(not(target_os = "zkvm"))]
pub mod prelude {
    use super::zk_programs;
    pub use zk_programs::init::ZKInitProgram;
}
