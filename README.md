# Blockchain Tech Map

A naive and simple blockchain ecosystem comparator. Pick any two ecosystems and instantly see how hard it would be to migrate between them — scored across language, VM, transaction model, EVM compatibility, deployment model, tooling, docs, L2 maturity, and funding.

**[Live demo →](https://al3mart.github.io/bc-techmap/)**

## Views

- **Grid** — all ecosystems laid out as cards. Click one to select it as source, click another to see the migration analysis.
- **Ring** — select an ecosystem to center it, and all others arrange themselves on concentric difficulty rings (Trivial → Easy → Moderate → Hard → Extreme). Click any ring node to open the migration panel.

## Tech

Built with [Leptos](https://leptos.dev/) (Rust → WebAssembly), styled with vanilla CSS, zero JS dependencies. Ecosystem data is parsed at compile time from TOML — no runtime parser in the binary.

## Contributing

Ecosystem data lives in [`data/ecosystems.toml`](data/ecosystems.toml). Contributions are welcome — whether that's adding new ecosystems, updating scores, fixing inaccuracies, or improving tooling lists. Open a PR and keep the same TOML structure.

## Building

```bash
# Dev
trunk serve

# Release
trunk build --release
```

Requires [Trunk](https://trunkrs.dev/) and the `wasm32-unknown-unknown` target:

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## License

[GPL-3.0](LICENSE)
