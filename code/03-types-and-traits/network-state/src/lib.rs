pub enum ConnectionState {
    Disconnected,
    Connecting { attempts: u32 },
    Connected { peer: String },
    Failed(String),
}

impl ConnectionState {
    pub fn on_connect_success(self, peer: String) -> Self {
        Self::Connected { peer }
    }
    pub fn on_connect_attempt(self) -> Self {
        match self {
            Self::Connecting { attempts } => Self::Connecting {
                attempts: attempts + 1,
            },
            _ => Self::Connecting { attempts: 1 },
        }
    }
    pub fn on_connect_failure(self, message: String) -> Self {
        Self::Failed(message)
    }
}
