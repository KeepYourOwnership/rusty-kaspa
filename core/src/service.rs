use crate::core::Core;
use std::{sync::Arc, thread::JoinHandle};

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
use intertrait::CastFromSync;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub trait CastFromSync: Send + Sync {}

pub trait Service: CastFromSync {
    fn ident(self: Arc<Self>) -> &'static str;
    fn start(self: Arc<Self>, core: Arc<Core>) -> Vec<JoinHandle<()>>;
    fn stop(self: Arc<Self>);
}
