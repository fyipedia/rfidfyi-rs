# rfidfyi

[![crates.io](https://img.shields.io/crates/v/rfidfyi.svg)](https://crates.io/crates/rfidfyi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust client for the [RFIDFYI](https://rfidfyi.com) REST API. RFID tags. Uses `reqwest` for HTTP.

> **Explore at [rfidfyi.com](https://rfidfyi.com)** — interactive tools and comprehensive reference.

## Install

```toml
[dependencies]
rfidfyi = "0.1"
```

## Quick Start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rfidfyi::Client::new();
    let result = client.search("query")?;
    println!("{:?}", result);
    Ok(())
}
```

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **Python** | `pip install rfidfyi` | [PyPI](https://pypi.org/project/rfidfyi/) |
| **npm** | `npm install rfidfyi` | [npm](https://www.npmjs.com/package/rfidfyi) |
| **Go** | `go get github.com/fyipedia/rfidfyi-go` | [pkg.go.dev](https://pkg.go.dev/github.com/fyipedia/rfidfyi-go) |
| **Rust** | `cargo add rfidfyi` | [crates.io](https://crates.io/crates/rfidfyi) |
| **Ruby** | `gem install rfidfyi` | [rubygems](https://rubygems.org/gems/rfidfyi) |


## Links

- **Site**: [rfidfyi.com](https://rfidfyi.com)
- **API**: [rfidfyi.com/api/v1/](https://rfidfyi.com/api/v1/)
- **OpenAPI**: [rfidfyi.com/api/v1/schema/](https://rfidfyi.com/api/v1/schema/)
- **Glossary**: [rfidfyi.com/glossary/](https://rfidfyi.com/glossary/)
- **Guides**: [rfidfyi.com/guide/](https://rfidfyi.com/guide/)
- **Tools**: [rfidfyi.com/tools/](https://rfidfyi.com/tools/)
Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## Tag FYI Family

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode formats, EAN, UPC, ISBN standards |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy, GATT, beacons |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips, tag types, NDEF, standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types, versions, encoding modes |
| **RFIDFYI** | [rfidfyi.com](https://rfidfyi.com) | RFID tags, frequency bands, standards |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart cards, EMV, APDU, Java Card |

## Embed Widget

Embed [RFIDFYI](https://rfidfyi.com) widgets on any website with [rfidfyi-embed](https://widget.rfidfyi.com):

```html
<script src="https://cdn.jsdelivr.net/npm/rfidfyi-embed@1/dist/embed.min.js"></script>
<div data-rfidfyi="entity" data-slug="example"></div>
```

Zero dependencies · Shadow DOM · 4 themes (light/dark/sepia/auto) · [Widget docs](https://widget.rfidfyi.com)

## License

MIT
