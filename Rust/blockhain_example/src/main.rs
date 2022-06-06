use std::{io, time::Duration};

use blockhain_example::{
    models::App,
    p2p::{self, EventType, LocalChainRequest, CHAIN_TOPIC},
};
use libp2p::{
    core::{connection::Event, transport::upgrade},
    futures::StreamExt,
    mplex::MplexConfig,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::SwarmBuilder,
    tcp::TokioTcpConfig,
    Swarm, Transport,
};
use log::{error, info};
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::mpsc,
    time::sleep,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Peer ID: {}", p2p::PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    let (init_sender, mut init_rcv) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&p2p::KEYS)
        .expect("can't create auth key");

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(MplexConfig::new())
        .boxed();

    let app_behaviour =
        p2p::AppBehaviour::new(App::new(), response_sender, init_sender.clone()).await;

    let mut swarm = SwarmBuilder::new(transp, app_behaviour, p2p::PEER_ID.clone())
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
        .build();

    let mut stdin = BufReader::new(stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can't get a local socket"),
    )
    .expect("swarm can be started");

    spawn(async move {
        std::thread::sleep(Duration::from_secs(1));
        info!("sending init event");
        init_sender.send(true).expect("can't send init event");
    });

    loop {
        let event = select! {
          cli_command = stdin.next_line() => cli_command.handle(),
          response = response_rcv.recv() =>
            response.map(EventType::LocalChainResponse),
          _init_command = init_rcv.recv() => Some(EventType::Init),

          event = swarm.select_next_some() => {

                    info!("Unhandled Swarm Event: {:?}", event);
                    None
                }
        };

        event.map(|e| match e {
            EventType::Init => {
                let peers = p2p::get_list_peers(&swarm);
                swarm.behaviour_mut().app.genesis();
                info!("connected modes: {}", peers.len());

                if !peers.is_empty() {
                    let req = LocalChainRequest {
                        from_peer_id: peers.last().expect("at least one peer").to_string(),
                    };
                    serde_json::to_string(&req)
                        .map(|json| {
                            swarm
                                .behaviour_mut()
                                .floodsub
                                .publish(CHAIN_TOPIC.clone(), json.as_bytes())
                        })
                        .unwrap_or_else(|e| error!("failed to init: {e}"))
                }
            }
            EventType::LocalChainResponse(resp) => serde_json::to_string(&resp)
                .map(|json| {
                    swarm
                        .behaviour_mut()
                        .floodsub
                        .publish(CHAIN_TOPIC.clone(), json.as_bytes())
                })
                .unwrap_or_else(|e| info!("failed to deser response: {e}")),
            EventType::Input(input) => match input.as_str() {
                "ls p" => p2p::handle_print_peers(&swarm),
                cmd if cmd.starts_with("ls c") => p2p::handle_print_chain(&swarm),
                cmd if cmd.starts_with("create b") => p2p::handle_create_block(cmd, &mut swarm),
                _ => {
                    error!("unknown command: {input}")
                }
            },
        });
    }
}

pub trait SelectHandler<HandledType>
where
    Self: Sized,
    HandledType: Sized,
{
    type Output;
    fn handle(self) -> Option<Self::Output>;
}

impl SelectHandler<EventType> for io::Result<Option<String>> {
    type Output = EventType;

    fn handle(self) -> Option<EventType> {
        self.unwrap_or_else(|er| {
            info!("cli command err: {er}");
            None
        })
        .map(EventType::Input)
    }
}
