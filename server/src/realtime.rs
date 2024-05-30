use rocket::response::stream::{Event, EventStream};
use rocket::tokio::sync::broadcast;
use rocket::{Shutdown, State};
use rocket::response::status::NoContent;
use rocket::tokio::select;

pub struct ConnectedClients {
    message_queue : broadcast::Sender<()>,
}

impl ConnectedClients {
    pub fn new() -> Self {
        let (sender, _rcv1) = broadcast::channel(20);
        ConnectedClients {
            message_queue: sender,
        }
    }
}

struct UserConnection {
    id: u32,
    channel: broadcast::Receiver<()>,
}

impl PartialEq for UserConnection {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for UserConnection {}

impl PartialOrd for UserConnection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UserConnection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl UserConnection {
    fn new(server: &ConnectedClients, id : u32) -> Self {
        UserConnection { id, channel: server.message_queue.subscribe() }
    }
}

#[get("/events")]
pub fn stream<'server>(mut shutdown: Shutdown, state : &'server State<ConnectedClients>) -> EventStream![Event + 'server] {
    EventStream! {
        let mut connection = UserConnection::new(state, 3);
        loop {
            select! {
                _ = connection.channel.recv() => yield Event::data("ping!"),
                _ = &mut shutdown => {
                    break;
                },
            }
        }
    }
}

#[post("/events")]
pub fn new_event(state: &State<ConnectedClients>) -> NoContent {
    state.message_queue.send(()).unwrap_or(0); // ignore errors - doesn't matter if no one reads the message
    NoContent
}
