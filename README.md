<div align="center">

# 🇵🇱 pizza-analysis-stempel

**Polish algorithmic stemming (Stempel) for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--stempel-blue)](https://github.com/pizza-rs/analysis-stempel)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Algorithmic stemmer for Polish using the Stempel algorithm — a table-driven
stemmer that applies learned suffix-removal patterns from a compiled automaton.
Compared to Morfologik's dictionary lookup, Stempel is lighter weight and can
handle out-of-vocabulary words, but may produce non-dictionary stems.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `stempel_stem` | Polish Stempel algorithmic stemmer |
| TokenFilter | `polish_stop` | Polish stop words |

### Stempel vs Morfologik

| Feature | Stempel | Morfologik |
|:--------|:--------|:-----------|
| Approach | Algorithmic (learned rules) | Dictionary (FSA lookup) |
| OOV handling | Applies rules to unknown words | Falls through unchanged |
| Output | Stems (may not be real words) | Lemmas (dictionary forms) |
| Size | Smaller model | Larger dictionary |

Choose Stempel when you need broader coverage; choose Morfologik when accuracy matters more.

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_stempel::register_all(&mut factory);

let stem = factory.get_token_filter("stempel_stem").unwrap();
```

## Installation

```toml
[dependencies]
pizza-analysis-stempel = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["stempel"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
