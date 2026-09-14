fn main() {
    let connection = ConnectionState::Disconnected;
    let connection = connection.on_connect_attempt();
    let connection = connection.on_connect_attempt();

    let connection = connection.on_connect_success(String::from("server.example"));
    let failed_connection = ConnectionState::Disconnected;
    let failed_connection = failed_connection.on_connect_attempt();
    let failed_connection = failed_connection.on_connect_failure(String::from("timed out"));
    match connection {
        ConnectionState::Connected { peer } => {
            println!("Connected to {peer}");
        }
        ConnectionState::Failed(message) => {
            println!("Connection failed: {message}");
        }
        _ => {
            println!("Not connected");
        }
    }
    match failed_connection {
        ConnectionState::Failed(message) => {
            println!("Connection failed: {message}");
        }
        _ => {
            println!("Expected a failed connection");
        }
    }
}

enum ConnectionState {
    Disconnected,
    Connecting { attempts: u32 },
    Connected { peer: String },
    Failed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_attempts_increment_count() {
        let connection = ConnectionState::Disconnected;
        let connection = connection.on_connect_attempt();
        let connection = connection.on_connect_attempt();
        match connection {
            ConnectionState::Connecting { attempts } => {
                assert_eq!(attempts, 2);
            }
            _ => {
                panic!("expectd the Connecting state");
            }
        }
    }
    #[test]
    fn successful_connection_stores_peer() {
        let connection = ConnectionState::Disconnected;
        let connection = connection.on_connect_success(String::from("server.example"));
        match connection {
            ConnectionState::Connected { peer } => {
                assert_eq!(peer, "server.example")
            }
            _ => {
                panic!("expected Connected")
            }
        }
    }
    #[test]
    fn failed_connection_stores_message() {
        let connection = ConnectionState::Disconnected;
        let connection = connection.on_connect_failure(String::from("timed out"));
        match connection {
            ConnectionState::Failed(message) => {
                assert_eq!(message, "timed out")
            }
            _ => {
                panic!("expected Failed")
            }
        }
    }
}
impl ConnectionState {
    fn on_connect_success(self, peer: String) -> Self {
        Self::Connected { peer }
    }
    fn on_connect_attempt(self) -> Self {
        match self {
            Self::Connecting { attempts } => Self::Connecting {
                attempts: attempts + 1,
            },
            _ => Self::Connecting { attempts: 1 },
        }
    }
    fn on_connect_failure(self, message: String) -> Self {
        Self::Failed(message)
    }
}
