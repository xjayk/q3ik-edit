pub enum ConnectionIdentifier {
    Setup(u64),
    Workspace(i64),
}

impl ConnectionIdentifier {
    pub fn setup() -> Self {
        Self::Setup(0)
    }
}

pub use crate::Interactive;
