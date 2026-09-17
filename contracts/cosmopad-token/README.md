# CosmoPad CW20 token contract

Tahap A menambahkan kontrak token standar CW20 untuk prototype CosmoPad.

## Yang didukung

- nama, symbol, dan decimals;
- initial balances;
- transfer token;
- allowance dan transfer_from;
- mint dan burn sesuai permission minter;
- query balance, token info, allowance, dan supply.

Kontrak ini menggunakan implementasi standar `cw20-base`. Kita tidak menambahkan aturan khusus yang belum diuji.

## Batasan penting

Kontrak ini belum otomatis dibuat oleh `cosmopad-launcher`. Launchpad masih belum melakukan buy/sell, belum memindahkan ATOM, dan belum mengelola liquidity.

Jangan gunakan untuk dana nyata sebelum testnet, review keamanan, dan audit.
