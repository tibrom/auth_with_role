use std::future;

use crate::domain::proxy::proxy_process::service::{TransportEndpoint, CommandListener, ContextSynchro, ServerConnector};
use crate::domain::proxy::state::ConnectionState;
use crate::domain::proxy::message::ProcessMessage;
use crate::domain::proxy::model::ConnectionContext;
use tracing::{error, info, debug};

pub struct ForwardingUseCase<A, B, CL, CS, SC> {
    connection_context: ConnectionContext,
    transport_endpoint_client: A,
    transport_endpoint_server: Option<B>,
    command_listener: CL,
    context_synchro: CS,
    server_connector: SC,
}

impl<A, B, CL, CS, SC> ForwardingUseCase<A, B, CL, CS, SC>
where
    A: TransportEndpoint,
    B: TransportEndpoint<Message = A::Message>,
    CL: CommandListener,
    CS: ContextSynchro,
    SC: ServerConnector<Message = A::Message, Error = A::Error, Endpoint = B>,
    A::Message: Clone + Send + 'static,
{
    pub fn new(
        transport_endpoint_client: A,
        connection_context: ConnectionContext,
        command_listener: CL,
        context_synchro: CS,
        server_connector: SC,
    ) -> Self {
        Self {
            connection_context,
            transport_endpoint_client,
            transport_endpoint_server: None,
            command_listener,
            context_synchro,
            server_connector,
        }
    }

    pub async fn execute(mut self) {
        if let Err(e) = self.run_forwarding_loop().await {
            error!("Forwarding loop terminated with error: {:?}", e);
        }
    }

    async fn run_forwarding_loop(&mut self) -> Result<(), ()> {
        info!("Starting forwarding loop...");

        loop {
            let client = &mut self.transport_endpoint_client;
            let server_opt = self.transport_endpoint_server.as_mut();
            let command_listener = &mut self.command_listener;

            let server_future = async {
                match server_opt {
                    Some(server) => server.receive().await,
                    None => future::pending().await
                }
            };

            tokio::select! {
                cmd = command_listener.receive() => {
                    debug!("Received command: {:?}", cmd);
                    if let Some(m) = cmd {
                        self.message_handler(m).await;
                    }
                    
                }

                msg = client.receive() => {
                    if !self.should_forward() {
                        info!("Forwarding stopped: current state does not allow forwarding.");
                        return Ok(());
                    }

                    match msg {
                        Some(Ok(m)) => {
                            debug!("Received message from client");
                            if let Some(server) = self.transport_endpoint_server.as_mut() {
                                if let Err(e) = server.send(m.clone()).await {
                                    error!("Failed to forward message to server: {:?}", e);
                                    return Err(());
                                }
                            }
                        }
                        _ => {
                            error!("Client connection closed or returned error");
                            return Err(());
                        },
                    }
                }

                msg = server_future => {
                    if !self.should_forward() {
                        info!("Forwarding stopped: current state does not allow forwarding.");
                        return Ok(());
                    }

                    match msg {
                        Some(Ok(m)) => {
                            debug!("Received message from server");
                            if let Err(e) = self.transport_endpoint_client.send(m.clone()).await {
                                error!("Failed to forward message to client: {:?}", e);
                                return Err(());
                            }
                        }
                        _ => {
                            error!("Server connection closed or returned error");
                            return Err(());
                        },
                    }
                }
            }
        }
    }

    async fn message_handler(&mut self, msg: ProcessMessage) {
        debug!("Handling command: {:?}", msg);
        let ProcessMessage::Context(context) = msg else {
            return ;
        };

        let old_state = self.connection_context.state().clone();
        let new_state = context.state().clone();

        if old_state == new_state {
            self.new_state_handler(&new_state).await;
        }

        self.connection_context = *context;

        self.context_synchro
            .push_context(self.connection_context.clone())
            .await;
    }

    async fn new_state_handler(&mut self, state: &ConnectionState) {
        if let ConnectionState::ServerConnection = state {
            info!("State changed to ServerConnection — connecting to server...");
            self.connect_to_server().await;
        }
    }

    async fn connect_to_server(&mut self) {
        let Some(access_token) = self.connection_context.access_token() else {
            error!("Missing access token; cannot connect to server.");
            self.connection_context.set_state(ConnectionState::Error);
            return;
        };

        match self.server_connector.connect(access_token.clone()).await {
            Ok(endpoint) => {
                info!("Successfully connected to server.");
                self.transport_endpoint_server = Some(endpoint);
                let Some(state) = self.connection_context.state().next() else {
                    return ;
                };
                self.connection_context.set_state(state);
            }
            Err(err) => {
                error!("Failed to connect to server: {:?}", err);
                self.connection_context.set_state(ConnectionState::Error);
            }
        }
    }

    fn should_forward(&self) -> bool {
        self.transport_endpoint_server.is_some()
            && matches!(self.connection_context.state(), ConnectionState::Active)
    }
}
