use std::net::SocketAddr;

use russh::server;

pub struct Server {
    pub event_tx:
        tokio::sync::mpsc::UnboundedSender<crate::event::EventContainer>,
}

impl server::Server for Server {
    type Handler = crate::handler::Handler;

    fn new_client(&mut self, addr: SocketAddr) -> Self::Handler {
        let handler =
            Self::Handler::new(uuid::Uuid::new_v4(), self.event_tx.clone());

        handler.send_event(crate::Event::TcpConnection { addr });

        handler
    }
}
