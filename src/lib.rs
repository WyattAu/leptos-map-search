//! # leptos-map-search
//!
//! Map search/typeahead component for Leptos.
//!
//! Provides a text input that filters a list of countries and fires a callback
//! when a selection is made. Supports keyboard navigation (arrow keys, Enter, Escape).
//!
//! ## Usage
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_map_search::{MapSearchBar, CountryMeta};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let (countries, _set_countries) = signal(Vec::<CountryMeta>::new());
//!     let on_select = Callback::new(move |country: CountryMeta| {
//!         leptos::logging::log!("Selected: {} ({})", country.name, country.iso2);
//!     });
//!
//!     view! {
//!         <MapSearchBar countries=Signal::from(countries) on_select=on_select/>
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod types;
mod component;

pub use types::CountryMeta;
pub use component::MapSearchBar;
