use thiserror::Error;

#[derive(Error, Debug)]
pub enum Dx12Error {
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),

    #[error("No compatible DirectX 12 hardware GPU adapter found")]
    NoAdapterFound,

    #[error("DirectX 12 feature level {0} not supported")]
    FeatureLevelNotSupported(String),

    #[error("Shared NT handle creation failed")]
    SharedHandleFailed,

    #[error("DirectComposition device initialization failed")]
    DirectCompositionFailed,

    #[error("DispatcherQueueController initialization failed")]
    DispatcherQueueFailed,

    #[error("GPU fence wait timeout ({0} ms)")]
    FenceTimeout(u32),
}

pub type Result<T> = std::result::Result<T, Dx12Error>;
