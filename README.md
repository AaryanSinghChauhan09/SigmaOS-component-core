# sigmaos-component-core

Shared lifecycle contract for SigmaOS **service** components. Implements
dependency inversion: daemons (`power-manager`, `network-manager`,
`notifications`, `update-manager`, `snapshot-manager`) depend on this
abstraction rather than duplicating trait definitions.

Library/tool components (file-writer, renamer, …) do **not** use this —
interface segregation.

```rust
use sigmaos_component_core::{ComponentLifecycle, ComponentResult};

struct PowerManager { ready: bool }
impl ComponentLifecycle for PowerManager {
    fn init(&mut self) -> ComponentResult<()> { self.ready = true; Ok(()) }
    fn run(&self) -> ComponentResult<()> { /* monitor loop */ Ok(()) }
    fn shutdown(&mut self) -> ComponentResult<()> { self.ready = false; Ok(()) }
    fn is_ready(&self) -> bool { self.ready }
}
```

`cargo test` — 3 tests (lifecycle transitions, error paths, object safety).
