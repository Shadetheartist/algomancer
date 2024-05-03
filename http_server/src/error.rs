#[derive(Debug, Responder)]
pub enum Error {

    #[response(status = 404)]
    LobbyNotFound(String),

    #[response(status = 500)]
    AgentDoesNotExist(String),

    #[response(status = 401)]
    AgentDoesNotExistWithKey(String),

    #[response(status = 400)]
    AgentNotInLobby(String),

    #[response(status = 400)]
    AgentAlreadyInLobby(String),

    #[response(status = 400)]
    AgentNotInCorrectLobby(String),

    #[response(status = 400)]
    DuplicateUsername(String),

    #[response(status = 500)]
    CannotRunServer(String),

    #[response(status = 400)]
    LobbyIsFull(String),
}


impl From<algomanserver::coordinator::Error> for Error {
    fn from(value: algomanserver::coordinator::Error) -> Self {
        match value {
            algomanserver::coordinator::Error::AgentDoesNotExist(_) => {
                Error::AgentDoesNotExist(value.to_string())
            }
            algomanserver::coordinator::Error::AgentDoesNotExistWithKey(_) => {
                Error::AgentDoesNotExistWithKey(value.to_string())
            }
            algomanserver::coordinator::Error::LobbyDoesNotExist(_) => {
                Error::LobbyNotFound(value.to_string())
            }
            algomanserver::coordinator::Error::AgentNotInAnyLobby(_) => {
                Error::AgentNotInLobby(value.to_string())
            }
            algomanserver::coordinator::Error::AgentAlreadyInLobby(_, _) => {
                Error::AgentAlreadyInLobby(value.to_string())
            }
            algomanserver::coordinator::Error::CannotRunError(_) => {
                Error::CannotRunServer(value.to_string())
            }
            algomanserver::coordinator::Error::AgentNotInCorrectLobby(_) => {
                Error::AgentNotInCorrectLobby(value.to_string())
            }
            algomanserver::coordinator::Error::NotListening(_) => {
                Error::AgentNotInCorrectLobby(value.to_string())
            }
            algomanserver::coordinator::Error::SendEventError(_) => {
                Error::AgentNotInCorrectLobby(value.to_string())
            }
            algomanserver::coordinator::Error::AgentAlreadyExistsWithUsername(_) => {
                Error::DuplicateUsername(value.to_string())
            }
            algomanserver::coordinator::Error::LobbyIsFull(_) => {
                Error::LobbyIsFull(value.to_string())
            }
        }
    }
}