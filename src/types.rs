//! Country metadata types.

use serde::Deserialize;

/// Metadata for a country, used for search and map positioning.
#[derive(Clone, Debug, Default, Deserialize, serde::Serialize)]
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

/// Filter countries by a query string (case-insensitive).
///
/// Matches against name, iso2, iso3, and capital fields.
pub fn filter_countries<'a>(countries: &'a [CountryMeta], query: &str) -> Vec<&'a CountryMeta> {
    let q = query.to_lowercase();
    if q.len() < 2 {
        return vec![];
    }
    let mut results: Vec<&CountryMeta> = countries
        .iter()
        .filter(|c| {
            c.name.to_lowercase().contains(&q)
                || c.iso2.to_lowercase().contains(&q)
                || c.iso3.to_lowercase().contains(&q)
                || c.capital.to_lowercase().contains(&q)
        })
        .collect();
    results.sort_by(|a, b| a.name.cmp(&b.name));
    results
}

/// Score a country match against a query (lower is better).
pub fn score_match(country: &CountryMeta, query: &str) -> u32 {
    let q = query.to_lowercase();
    let name_lower = country.name.to_lowercase();
    let iso2_lower = country.iso2.to_lowercase();
    let iso3_lower = country.iso3.to_lowercase();
    let capital_lower = country.capital.to_lowercase();

    if name_lower == q {
        return 0;
    }
    if iso2_lower == q || iso3_lower == q {
        return 1;
    }
    if name_lower.starts_with(&q) {
        return 2;
    }
    if name_lower.contains(&q) {
        return 3;
    }
    if capital_lower.contains(&q) {
        return 4;
    }
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_country(name: &str, iso2: &str, iso3: &str, capital: &str) -> CountryMeta {
        CountryMeta {
            iso2: iso2.into(),
            iso3: iso3.into(),
            name: name.into(),
            capital: capital.into(),
            lat: 0.0,
            lng: 0.0,
            region: "Test".into(),
            population: Some(1_000_000),
        }
    }

    fn sample_countries() -> Vec<CountryMeta> {
        vec![
            make_country("United States", "US", "USA", "Washington D.C."),
            make_country("United Kingdom", "GB", "GBR", "London"),
            make_country("Japan", "JP", "JPN", "Tokyo"),
            make_country("Germany", "DE", "DEU", "Berlin"),
            make_country("France", "FR", "FRA", "Paris"),
            make_country("Brazil", "BR", "BRA", "Brasilia"),
            make_country("Australia", "AU", "AUS", "Canberra"),
            make_country("Canada", "CA", "CAN", "Ottawa"),
        ]
    }

    #[test]
    fn filter_by_name() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "united");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|c| c.name.contains("United")));
    }

    #[test]
    fn filter_by_iso2() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "JP");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Japan");
    }

    #[test]
    fn filter_by_iso3() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "DEU");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Germany");
    }

    #[test]
    fn filter_by_capital() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "tokyo");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Japan");
    }

    #[test]
    fn filter_case_insensitive() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "JAPAN");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Japan");
    }

    #[test]
    fn filter_no_match() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "xyz");
        assert!(results.is_empty());
    }

    #[test]
    fn filter_too_short_query() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "J");
        assert!(results.is_empty());
    }

    #[test]
    fn filter_empty_query() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "");
        assert!(results.is_empty());
    }

    #[test]
    fn filter_sorted_by_name() {
        let countries = sample_countries();
        let results = filter_countries(&countries, "an");
        let names: Vec<&str> = results.iter().map(|c| c.name.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn filter_limit_results() {
        let countries = sample_countries();
        // "a" matches many but query is too short
        let results = filter_countries(&countries, "a");
        assert!(results.is_empty()); // < 2 chars
    }

    #[test]
    fn score_exact_name() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "Japan"), 0);
    }

    #[test]
    fn score_exact_iso() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "JP"), 1);
        assert_eq!(score_match(&country, "JPN"), 1);
    }

    #[test]
    fn score_starts_with() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "Jap"), 2);
    }

    #[test]
    fn score_contains_in_name() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "pan"), 3);
    }

    #[test]
    fn score_in_capital() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "Tokyo"), 4);
    }

    #[test]
    fn score_no_match() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "xyz"), 5);
    }

    #[test]
    fn score_case_insensitive() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        assert_eq!(score_match(&country, "japan"), 0);
    }

    #[test]
    fn default_country_meta() {
        let c = CountryMeta::default();
        assert!(c.name.is_empty());
        assert!(c.iso2.is_empty());
        assert!(c.population.is_none());
    }

    #[test]
    fn serde_roundtrip() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        let json = serde_json::to_string(&country).unwrap();
        let deserialized: CountryMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "Japan");
        assert_eq!(deserialized.iso2, "JP");
    }

    #[test]
    fn search_result_cloning() {
        let country = make_country("Japan", "JP", "JPN", "Tokyo");
        let result = SearchResult {
            country: country.clone(),
            score: 0,
        };
        let cloned = result.clone();
        assert_eq!(cloned.country.name, "Japan");
        assert_eq!(cloned.score, 0);
    }
}
