//! P2P network implementation using libp2p (TCP + Noise + Yamux + `GossipSub` 1.1).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::Duration;

use libp2p::futures::StreamExt;
use libp2p::gossipsub::{
    self, Behaviour as Gossipsub, ConfigBuilder as GossipsubConfigBuilder, IdentTopic as Topic,
    MessageAuthenticity, MessageId, ValidationMode,
};
use libp2p::swarm::SwarmEvent;
use libp2p::{Multiaddr, Swarm};
use tokio::sync::mpsc;

use crate::envelope::SignedEnvelope;
use crate::error::{NodeError, Result};

pub struct NetworkService {
    swarm: Swarm<Gossipsub>,
    consensus_topic: Topic,
    seed_peers: Vec<String>,
    inbound_tx: mpsc::Sender<SignedEnvelope>,
    outbound_rx: mpsc::Receiver<SignedEnvelope>,
}

impl NetworkService {
    ///
    /// # Errors
    ///
    /// Restituisce errore se l'indirizzo di ascolto non e' valido, se il trasporto non si costruisce, o se la sottoscrizione al topic fallisce.
    pub fn new(
        network_id: &str,
        listen_addr: &str,
        seed_peers: &[String],
        inbound_tx: mpsc::Sender<SignedEnvelope>,
        outbound_rx: mpsc::Receiver<SignedEnvelope>,
    ) -> Result<Self> {
        let id_keys = libp2p::identity::Keypair::generate_ed25519();

        // Message ID computation for GossipSub deduplication
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_millis(50))
            .mesh_n_low(1)
            .mesh_n(3)
            .mesh_n_high(4)
            .mesh_outbound_min(1)
            .flood_publish(true)
            .validation_mode(ValidationMode::Permissive)
            .message_id_fn(message_id_fn)
            .build()
            .map_err(|e| NodeError::Protocol(format!("failed to build gossipsub config: {e}")))?;

        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(id_keys.clone()),
            gossipsub_config,
        )
        .map_err(|e| NodeError::Protocol(format!("failed to initialize gossipsub: {e}")))?;

        let consensus_topic_str = format!("/coblox/{network_id}/consensus/0.1");
        let consensus_topic = Topic::new(&consensus_topic_str);
        gossipsub
            .subscribe(&consensus_topic)
            .map_err(|e| NodeError::Protocol(format!("failed to subscribe to topic: {e}")))?;

        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )
            .map_err(|e| NodeError::Protocol(format!("failed to build libp2p transport: {e}")))?
            .with_behaviour(|_| gossipsub)
            .map_err(|e| NodeError::Protocol(format!("failed to configure swarm behaviour: {e}")))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_mins(1)))
            .build();

        let listen_multiaddr = Multiaddr::from_str(listen_addr)
            .map_err(|e| NodeError::Protocol(format!("invalid listen multiaddr: {e}")))?;
        swarm
            .listen_on(listen_multiaddr)
            .map_err(|e| NodeError::Protocol(format!("failed to listen on multiaddr: {e}")))?;

        for peer in seed_peers {
            if peer != listen_addr
                && let Ok(addr) = Multiaddr::from_str(peer)
            {
                let _ = swarm.dial(addr);
            }
        }

        Ok(Self {
            swarm,
            consensus_topic,
            seed_peers: seed_peers.to_vec(),
            inbound_tx,
            outbound_rx,
        })
    }

    /// Runs the network event loop.
    pub async fn run(mut self) {
        let mut redial_interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            tokio::select! {
                _ = redial_interval.tick() => {
                    for peer in &self.seed_peers {
                        if let Ok(addr) = Multiaddr::from_str(peer) {
                            let _ = self.swarm.dial(addr);
                        }
                    }
                }
                outbound = self.outbound_rx.recv() => {
                    match outbound {
                        Some(envelope) => {
                            if let Ok(bytes) = envelope.to_jcs() {
                                let _ = self.swarm.behaviour_mut().publish(self.consensus_topic.clone(), bytes);
                            }
                        }
                        None => break,
                    }
                }
                event = self.swarm.next() => {
                    match event {
                        Some(SwarmEvent::Behaviour(gossipsub::Event::Message {
                            message,
                            ..
                        })) => {
                            if let Ok(envelope) = SignedEnvelope::from_slice(&message.data) {
                                let _ = self.inbound_tx.send(envelope).await;
                            }
                        }
                        Some(_) => {}
                        None => break,
                    }
                }
            }
        }
    }
}
