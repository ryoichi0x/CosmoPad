# CosmoPad

CosmoPad adalah prototype launchpad token bergaya Pump.fun untuk ekosistem Cosmos.

## Status saat ini

### Tahap A — CW20 token foundation

Sudah ditambahkan kontrak `contracts/cosmopad-token` berbasis `cw20-base` dengan:

- pembuatan token CW20;
- initial balances;
- transfer;
- allowance/transfer_from dari implementasi standar;
- mint/burn sesuai konfigurasi minter;
- query token info, balance, allowance, dan supply;
- unit tests dasar.

### Launcher

`contracts/cosmopad-launcher` masih berada pada tahap fondasi bonding curve dan query. Ia belum membuat token CW20 secara otomatis dan belum melakukan trading.

## Belum tersedia

- transfer ATOM untuk trading;
- real buy/sell;
- wallet/frontend;
- DEX integration;
- liquidity migration;
- mainnet deployment.

Jangan gunakan kontrak ini untuk dana nyata. Jalankan test terlebih dahulu:

```bash
cargo test
```
