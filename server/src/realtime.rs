use rocket::response::stream::{Event, EventStream};
use rocket::tokio::sync::broadcast;
use rocket::{Shutdown, State};
use rocket::response::status::NoContent;
use rocket::tokio::select;

pub struct ConnectedClients {
    message_queue : broadcast::Sender<String>,
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
    channel: broadcast::Receiver<String>,
}

impl UserConnection {
    fn new(server: &ConnectedClients) -> Self {
        UserConnection { channel: server.message_queue.subscribe() }
    }
}

#[get("/events")]
pub fn stream<'server>(mut shutdown: Shutdown, state : &'server State<ConnectedClients>) -> EventStream![Event + 'server] {
    EventStream! {
        let mut connection = UserConnection::new(state);
        loop {
            select! {
                message = connection.channel.recv() => match message {
                    Ok(message) => yield Event::data(message),
                    Err(err) => yield Event::comment(format!("Error: {}", err)),
                },
                _ = &mut shutdown => {
                    break;
                },
            }
        }
    }
}

#[post("/events", data="<message>")]
pub fn new_event(state: &State<ConnectedClients>, message : String) -> NoContent {
    state.message_queue.send(message).unwrap_or(0); // ignore errors - doesn't matter if no one reads the message
    NoContent
}
