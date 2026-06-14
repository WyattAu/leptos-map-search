//! MapSearchBar Leptos component.

use leptos::prelude::*;
use web_sys::KeyboardEvent;

use crate::types::CountryMeta;

/// A map search/typeahead component.
///
/// Provides a text input that filters countries and fires a callback on selection.
///
/// # Props
///
/// - `countries`: Signal<Vec<CountryMeta>> — list of countries to search
/// - `on_select`: Callback<CountryMeta> — fires when user selects a country
/// - `placeholder`: Option<String> — input placeholder text
/// - `class`: Option<String> — additional CSS class
#[component]
pub fn MapSearchBar(
    countries: Signal<Vec<CountryMeta>>,
    on_select: Callback<CountryMeta>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal(Vec::<CountryMeta>::new());
    let (show_dropdown, set_show_dropdown) = signal(false);
    let (selected_idx, set_selected_idx) = signal(0usize);

    let placeholder = placeholder.unwrap_or_else(|| "SEARCH COUNTRY...".to_string());
    let class = class.unwrap_or_else(|| "map-search".to_string());

    // Filter countries on query change
    Effect::new(move |_| {
        let q = query.get().to_lowercase();
        let all = countries.get();

        if q.len() < 2 {
            set_results.set(vec![]);
            set_show_dropdown.set(false);
            return;
        }

        let mut matched: Vec<CountryMeta> = all
            .into_iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.iso2.to_lowercase().contains(&q)
                    || c.iso3.to_lowercase().contains(&q)
                    || c.capital.to_lowercase().contains(&q)
            })
            .take(8)
            .collect();

        matched.sort_by(|a, b| a.name.cmp(&b.name));

        set_results.set(matched);
        set_show_dropdown.set(true);
        set_selected_idx.set(0);
    });

    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        set_query.set(val);
    };

    let on_keydown = move |ev: KeyboardEvent| {
        let res = results.get();
        let len = res.len();
        if len == 0 { return; }

        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                set_selected_idx.update(|i| *i = (*i + 1).min(len - 1));
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_idx.update(|i| *i = i.saturating_sub(1));
            }
            "Enter" => {
                ev.prevent_default();
                let idx = selected_idx.get();
                if let Some(country) = res.get(idx) {
                    on_select.run(country.clone());
                    set_query.set(String::new());
                    set_results.set(vec![]);
                    set_show_dropdown.set(false);
                }
            }
            "Escape" => {
                set_show_dropdown.set(false);
            }
            _ => {}
        }
    };

    let on_blur = move |_| {
        let set_show = set_show_dropdown.clone();
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(150).await;
            set_show.set(false);
        });
    };

    let on_focus = move |_| {
        if !results.get().is_empty() {
            set_show_dropdown.set(true);
        }
    };

    let select_country = move |country: CountryMeta| {
        on_select.run(country);
        set_query.set(String::new());
        set_results.set(vec![]);
        set_show_dropdown.set(false);
    };

    view! {
        <div class=class>
            <input
                type="text"
                class="map-search-input"
                placeholder=placeholder
                prop:value=query
                on:input=on_input
                on:keydown=on_keydown
                on:blur=on_blur
                on:focus=on_focus
                autocomplete="off"
                spellcheck="false"
                aria-label="Search country"
                aria-autocomplete="list"
                aria-expanded=move || show_dropdown.get().to_string()
            />
            {move || if show_dropdown.get() && !results.get().is_empty() {
                let items = results.get();
                let idx = selected_idx.get();
                view! {
                    <div class="map-search-dropdown" role="listbox">
                        {items.into_iter().enumerate().map(|(i, country)| {
                            let is_selected = i == idx;
                            let name = country.name.clone();
                            let iso2 = country.iso2.clone();
                            let country_clone = country.clone();
                            let select = select_country.clone();
                            view! {
                                <div
                                    class=move || if is_selected { "map-search-item is-selected" } else { "map-search-item" }
                                    role="option"
                                    aria-selected=is_selected.to_string()
                                    on:mousedown=move |_| select(country_clone.clone())
                                >
                                    <span class="map-search-iso">{iso2}</span>
                                    <span class="map-search-name">{name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
