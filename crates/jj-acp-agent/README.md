# jj-acp-agent

Declarative ACP runner catalogs and trusted launch environments.

Part of [jj-acp](https://github.com/stevejuma/jj-acp), an Agent Client Protocol
harness. Rust 1.98.1 or newer is required.

```toml
[dependencies]
jj-acp-agent = "0.1.0"
```

See the [API reference](https://docs.rs/jj-acp-agent) and the
[canonical specification](https://stevejuma.github.io/jj-acp/).

Control version 3 is experimental; its live-adapter qualification gaps remain
recorded in the specification ledger. The controlling application owns trust,
credentials, prompt policy and durable state.

Apache-2.0. This project derives from `block/buzz`'s `buzz-acp`; see NOTICE.md.
