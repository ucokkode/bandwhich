use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct GetInterfaceError(#[from] GetInterfaceErrorKind);

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum GetInterfaceErrorKind {
    #[error("{0}")]
    PermissionError(String),
    #[error("{0}")]
    OtherError(String),
}
