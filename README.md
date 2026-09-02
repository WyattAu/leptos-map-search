# leptos-map-search

Map search/typeahead component for Leptos — filter and select countries with keyboard navigation.

## Features

- Text input with country search/typeahead
- Filters by name, ISO2, ISO3, or capital city
- Keyboard navigation (Arrow keys, Enter, Escape)
- Dropdown with match highlighting
- Supports SSR and hydration

## Installation

```bash
cargo add leptos-map-search
```

## Quick Start

```rust
use leptos::prelude::*;
use leptos_map_search::{MapSearchBar, CountryMeta};

#[component]
fn App() -> impl IntoView {
    let on_select = move |country: CountryMeta| {
        leptos::logging::log!("Selected: {} ({})", country.name, country.iso2);
    };

    view! {
        <MapSearchBar countries=country_list on_select=on_select/>
    }
}
```

## API Reference

### MapSearchBar

| Prop | Type | Description |
|------|------|-------------|
| `countries` | `Signal<Vec<CountryMeta>>` | List of countries to search |
| `on_select` | `Callback<CountryMeta>` | Fires when user selects a country |
| `placeholder` | `Option<String>` | Input placeholder text |
| `class` | `Option<String>` | Additional CSS class |

### CountryMeta

```rust
pub struct CountryMeta {
    pub iso2: String,        // ISO2 code (e.g., "US")
    pub iso3: String,        // ISO3 code (e.g., "USA")
    pub name: String,        // Common name (e.g., "United States")
    pub capital: String,     // Capital city name
    pub lat: f64,            // Capital latitude
    pub lng: f64,            // Capital longitude
    pub region: String,      // Region (e.g., "Americas")
    pub population: Option<i64>,
}
```

## Features

```toml
[dependencies]
leptos-map-search = { version = "0.1", features = ["hydrate"] }
```

- `hydrate` (default) — Client-side hydration support
- `ssr` — Server-side rendering support

## License

MIT
