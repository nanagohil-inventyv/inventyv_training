use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
struct Session {
    token: Option<String>, // one-time consumable token for authentication
    last_active: SystemTime,
}

#[derive(Debug)]
struct SessionManager {
    sessions: HashMap<u64, Session>, // user_id -> Session
}

impl SessionManager {
    /// create manager with capacity for n users
    fn new(expected_users: usize) -> Self {
        let mut sessions = HashMap::new();
        sessions
            .try_reserve(expected_users)
            .expect("Failed to reserve capacity for sessions");
        Self { sessions }
    }

    /// Add session in bulk (e.g. from DB or config)

    fn load_sessions(&mut self, data: Vec<(u64, Session)>) {
        // extent -> merge sessions
        self.sessions.extend(data);
    }

    /// Take a snapshot (rollback)
    fn snapshot(&self) -> HashMap<u64, Session> {
        self.sessions.clone()
    }

    /// cleanup inactive sessions

    fn cleanup_inactive(&mut self, max_idle: Duration) {
        let now = SystemTime::now();

        self.sessions.retain(|_, session| {
            now.duration_since(session.last_active)
                .map(|d| d <= max_idle)
                .unwrap_or(false)
        });
    }

    /// Consume a one-time token for a user

    fn consume_token(&mut self, user_id: u64) -> Option<String> {
        if let Some(session) = self.sessions.get_mut(&user_id) {
            session.token.take()
        } else {
            None
        }
    }
}
fn main() {
    let mut manager = SessionManager::new(100);
    manager.load_sessions(vec![
        (
            1,
            Session {
                token: Some("OPT234".into()),
                last_active: SystemTime::now(),
            },
        ),
        (
            2,
            Session {
                token: Some("OPT567".into()),
                last_active: SystemTime::now() - Duration::from_secs(3600),
            },
        ),
    ]);

    let token = manager.consume_token(1);
    println!("Consumed token for user 1: {:#?}", token);

    let backup = manager.snapshot();

    // cleanup sessions inactive for more than 30 minutes
    manager.cleanup_inactive(Duration::from_secs(1800));

    println!("Active sessions after cleanup: {:#?}", manager.sessions);
    println!("Snapshot of sessions: {:#?}", backup);
}
