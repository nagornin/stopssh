use crate::event::{Event, EventContainer};
use russh::server::{self, Auth};
use uuid::Uuid;

type Sender = tokio::sync::mpsc::UnboundedSender<EventContainer>;

pub struct Handler {
    session_id: Uuid,
    event_tx: Sender,
}

impl Handler {
    pub fn new(session_id: Uuid, event_tx: Sender) -> Self {
        Self {
            session_id,
            event_tx,
        }
    }

    pub fn send_event(&self, event: Event) {
        let _ = self.event_tx.send(EventContainer {
            session_id: self.session_id,
            time: std::time::SystemTime::now(),
            event,
        });
    }
}

#[async_trait::async_trait]
impl server::Handler for Handler {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn auth_password(
        &mut self,
        user: &str,
        password: &str,
    ) -> Result<Auth, Self::Error> {
        self.send_event(Event::PasswordAuth {
            user: user.into(),
            password: password.into(),
        });

        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn auth_publickey_offered(
        &mut self,
        user: &str,
        public_key: &russh::keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        self.send_event(Event::PublicKeyAuth {
            user: user.into(),
            key: public_key.clone(),
        });

        Ok(Auth::Reject {
            proceed_with_methods: Some(russh::MethodSet::PASSWORD),
        })
    }

    async fn received_sshid(&mut self, sshid: &[u8]) {
        self.send_event(Event::Version {
            version: sshid.into(),
        });
    }

    async fn received_kex_init_packet(
        &mut self,
        packet: &russh::server::KexInitPacket,
    ) {
        self.send_event(Event::KexInit {
            packet: packet.clone(),
        });
    }
}
