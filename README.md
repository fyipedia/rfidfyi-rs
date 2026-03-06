# rfidfyi

[![crates.io](https://img.shields.io/crates/v/rfidfyi)](https://crates.io/crates/rfidfyi)
[![docs.rs](https://docs.rs/rfidfyi/badge.svg)](https://docs.rs/rfidfyi)

Async Rust client for the [RFIDFYI](https://rfidfyi.com) API. Look up RFID tags, readers, EPC standards, frequency bands, and industry use cases.

## Install

```toml
[dependencies]
rfidfyi = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use rfidfyi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let results = client.search("uhf").await?;
    println!("Found {} results", results.total);
    Ok(())
}
```

## API Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Search tags, readers, and glossary |
| `tag(slug)` | Get RFID tag details |
| `reader(slug)` | Get RFID reader details |
| `family(slug)` | Get tag family details |
| `frequency(slug)` | Get frequency band details |
| `standard(slug)` | Get standard details |
| `epc(slug)` | Get EPC standard details |
| `use_case(slug)` | Get use case details |
| `glossary_term(slug)` | Get glossary term definition |
| `compare(slug_a, slug_b)` | Compare two RFID tags |
| `random()` | Get a random RFID tag |

All methods are async and return `Result<T, RfidFyiError>`.

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| Python | [rfidfyi](https://pypi.org/project/rfidfyi/) | `pip install rfidfyi` |
| TypeScript | [rfidfyi](https://www.npmjs.com/package/rfidfyi) | `npm install rfidfyi` |
| Go | [rfidfyi-go](https://pkg.go.dev/github.com/fyipedia/rfidfyi-go) | `go get github.com/fyipedia/rfidfyi-go` |
| Rust | [rfidfyi](https://crates.io/crates/rfidfyi) | `cargo add rfidfyi` |
| Ruby | [rfidfyi](https://rubygems.org/gems/rfidfyi) | `gem install rfidfyi` |

## Code FYI Family

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode symbologies & standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types & encoding |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips & protocols |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags & readers |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart card platforms |

## License

MIT
