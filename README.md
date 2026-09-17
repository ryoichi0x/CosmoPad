# CosmoPad

CosmoPad adalah prototype launchpad token bergaya Pump.fun untuk ekosistem Cosmos.

## Status

V0.1 hanya berisi fondasi:

- konfigurasi launch;
- state virtual reserve;
- constant-product bonding curve (`x * y = k`);
- estimasi harga, buy, dan sell;
- validasi input dan arithmetic safety;
- unit tests.

Belum ada transfer token/ATOM, trading sungguhan, deployment mainnet, migrasi liquidity, governance, atau fitur anti-bot. **Jangan gunakan contract ini untuk dana nyata.**

## Menjalankan test

Install Rust dan Cargo, lalu dari root repository jalankan:

```bash
cargo test
```

## Struktur

- `contracts/cosmopad-launcher/Cargo.toml`: dependency dan konfigurasi package Rust.
- `src/msg.rs`: bentuk pesan instantiate/execute/query.
- `src/state.rs`: konfigurasi dan state curve.
- `src/math.rs`: perhitungan integer-only.
- `src/error.rs`: error yang mungkin terjadi.
- `src/contract.rs`: entry point yang masih minimal; transfer belum diimplementasikan.
- `src/lib.rs`: mendaftarkan module Rust.
