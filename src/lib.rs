//! sigmaos-component-core — the shared lifecycle contract for SigmaOS
//! service components. Service daemons (power-manager, network-manager,
//! notifications, update-manager, snapshot-manager) implement
//! [`ComponentLifecycle`]; library/tool components stay functional and do
//! NOT use this trait (interface segregation).
//!
//! Dependency-inversion: components depend on this abstraction, not on each
//! other's concrete types.

#![forbid(unsafe_code)]

use std::fmt;

// ── Error contract ──────────────────────────────────────────────────────────

/// Unified error type every service component maps into.
#[derive(Debug)]
pub enum ComponentError {
    NotInitialized,
    AlreadyInitialized,
    NotReady,
    Runtime(String),
    Io(std::io::Error),
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "component not initialised"),
            Self::AlreadyInitialized => write!(f, "component already initialised"),
            Self::NotReady => write!(f, "component not ready"),
            Self::Runtime(m) => write!(f, "runtime error: {m}"),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}
impl std::error::Error for ComponentError {}
impl From<std::io::Error> for ComponentError {
    fn from(e: std::io::Error) -> Self { Self::Io(e) }
}

pub type ComponentResult<T> = Result<T, ComponentError>;

// ── Lifecycle contract ──────────────────────────────────────────────────────

/// The lifecycle every SigmaOS *service* component implements.
///
/// - `init`   — acquire resources, validate config, reach a ready state.
/// - `run`    — perform the component's work (may block; call on a worker).
/// - `shutdown`— release resources cleanly; must be idempotent-safe.
/// - `is_ready`— true once `init` succeeded and before `shutdown`.
pub trait ComponentLifecycle {
    fn init(&mut self) -> ComponentResult<()>;
    fn run(&self) -> ComponentResult<()>;
    fn shutdown(&mut self) -> ComponentResult<()>;
    fn is_ready(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reference implementation proving the trait is object-safe and usable
    /// polymorphically (dependency inversion).
    struct Dummy { ready: bool }
    impl Dummy {
        fn new() -> Self { Self { ready: false } }
    }
    impl ComponentLifecycle for Dummy {
        fn init(&mut self) -> ComponentResult<()> { self.ready = true; Ok(()) }
        fn run(&self) -> ComponentResult<()> {
            if self.ready { Ok(()) } else { Err(ComponentError::NotReady) }
        }
        fn shutdown(&mut self) -> ComponentResult<()> { self.ready = false; Ok(()) }
        fn is_ready(&self) -> bool { self.ready }
    }

    #[test]
    fn lifecycle_transitions() {
        let mut c = Dummy::new();
        assert!(!c.is_ready());
        c.init().unwrap();
        assert!(c.is_ready());
        c.run().unwrap();
        c.shutdown().unwrap();
        assert!(!c.is_ready());
    }

    #[test]
    fn run_before_init_fails() {
        let c = Dummy::new();
        assert!(matches!(c.run(), Err(ComponentError::NotReady)));
    }

    #[test]
    fn trait_is_object_safe() {
        let mut c: Box<dyn ComponentLifecycle> = Box::new(Dummy::new());
        c.init().unwrap();
        assert!(c.is_ready());
        c.shutdown().unwrap();
    }
}
