use anyhow::Result;
use smithay_client_toolkit::{
    delegate_registry,
    output::{OutputHandler, OutputState},
    reexports::client::{self, Connection, EventQueue, globals::registry_queue_init},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};
use tokio::sync::mpsc::{self, UnboundedSender};

pub struct Watcher {
    output_state: OutputState,
    registry_state: RegistryState,
    tx: UnboundedSender<WaylandEvent>,
}

delegate_registry!(Watcher);

impl ProvidesRegistryState for Watcher {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState];
}

smithay_client_toolkit::delegate_dispatch2!(Watcher);

impl OutputHandler for Watcher {
    fn output_state(&mut self) -> &mut smithay_client_toolkit::output::OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        conn: &client::Connection,
        qh: &client::QueueHandle<Self>,
        output: client::protocol::wl_output::WlOutput,
    ) {
        let Some(info) = self.output_state.info(&output) else {
            return;
        };
        dbg!(&info);
    }

    fn update_output(
        &mut self,
        conn: &client::Connection,
        qh: &client::QueueHandle<Self>,
        output: client::protocol::wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        conn: &client::Connection,
        qh: &client::QueueHandle<Self>,
        output: client::protocol::wl_output::WlOutput,
    ) {
    }
}

pub enum WaylandEvent {}

impl Watcher {
    pub fn try_new(
        conn: &Connection,
        tx: UnboundedSender<WaylandEvent>,
    ) -> Result<(Self, EventQueue<Self>)> {
        let (globals, event_queue) = registry_queue_init::<Self>(conn)?;
        let qh = event_queue.handle();

        let watcher = Self {
            output_state: OutputState::new(&globals, &qh),
            registry_state: RegistryState::new(&globals),
            tx,
        };
        Ok((watcher, event_queue))
    }

    pub fn spawn_watcher(conn: &Connection) -> Result<()> {
        let (tx, rx) = mpsc::unbounded_channel::<WaylandEvent>();

        let (watcher, mut event_queue) = Self::try_new(conn, tx)?;

        std::thread::spawn(move || {});

        Ok(())
    }
}
