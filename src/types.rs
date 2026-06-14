//! Country metadata types.

use serde::Deserialize;

/// Metadata for a country, used for search and map positioning.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CountryMeta {
    /// ISO 3166-1 alpha-2 code (e.g., "US")
    pub iso2: String,
    /// ISO 3166-1 alpha-3 code (e.g., "USA")
    pub iso3: String,
    /// Common name (e.g., "United States")
    pub name: String,
    /// Capital city name
    pub capital: String,
    /// Latitude of capital
    pub lat: f64,
    /// Longitude of capital
    pub lng: f64,
    /// Region (e.g., "Americas")
    pub region: String,
    /// Population
    pub population: Option<i64>,
}

/// Search result with match score.
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// The matched country
    pub country: CountryMeta,
    /// Match score (lower is better)
    pub score: u32,
}
