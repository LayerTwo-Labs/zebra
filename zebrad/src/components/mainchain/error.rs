use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitMainchainError {
    #[error("unable to initialize enforcer")]
    InitEnforcer(#[from] color_eyre::Report),
}

#[derive(Error, Debug)]
pub enum GetSidechainsError {
    #[error("unable to get sidechains")]
    GetSidechains(#[from] tonic::Status),
}
