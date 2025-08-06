use zkpo::prelude::*;

/// take some data and prove it's initiailized to 0

pub struct SecretData {
    data: [u8; 32],
    exe: Box<dyn ZKExe>,
}
