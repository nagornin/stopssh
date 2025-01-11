use clap::Parser;
use event::{Event, EventContainer};
use russh::{
    keys::key::KeyPair,
    server::{Config, Server},
    MethodSet,
};
use tokio::{fs, io::AsyncWriteExt, sync::mpsc};

mod config;
mod event;
mod handler;
mod logger;
mod server;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg = config::Config::parse();
    logger::setup_logger()?;
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let mut server = server::Server { event_tx };

    let server_cfg = Config {
        server_id: russh::SshId::Standard(
            "SSH-2.0-OpenSSH_9.2p1 Debian-2+deb12u3".into(),
        ),
        methods: MethodSet::PASSWORD | MethodSet::PUBLICKEY,
        keys: vec![KeyPair::generate_ed25519().unwrap()],
        ..Default::default()
    };

    let mut output_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .truncate(false)
        .open(cfg.output_file)
        .await?;

    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            process_event(&mut output_file, event).await;
        }
    });

    log::info!("Binding to {}", cfg.listen_addr);

    server
        .run_on_address(server_cfg.into(), cfg.listen_addr)
        .await?;

    Ok(())
}

async fn process_event(out: &mut fs::File, event: EventContainer) {
    let mut data = serde_json::to_string(&event).unwrap();
    data.push('\n');
    out.write_all(data.as_ref()).await.unwrap();
    out.flush().await.unwrap();

    if let Event::TcpConnection { addr } = event.event {
        log::info!("New session {} from {}", event.session_id, addr);
    }
}
