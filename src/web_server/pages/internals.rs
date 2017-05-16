use iron::prelude::{Request, Response};
use iron::{IronResult, status};
use maud::{Markup, PreEscaped};
use router::Router;
use rocket::State;

use web_server::pages;
use web_server::server::{CacheType, ConfigArc};
use web_server::view::layout;
use metadata::{Broker, ClusterId};
use cache::Cache;

fn broker_table() -> PreEscaped<String> {
    layout::datatable_ajax("internals-cache-brokers-ajax", "/api/internals/cache/brokers", "",
        html! { tr { th "Cluster id" th "Broker ids" } }
    )
}

fn metrics_table() -> PreEscaped<String> {
    layout::datatable_ajax("internals-cache-metrics-ajax", "/api/internals/cache/metrics", "",
        html! { tr { th "Cluster id" th "Broker id" th "Topics" } }
    )
}

fn cache_description_table(name: &str, key: &str, value: &str) -> PreEscaped<String> {
    html! {
        table style="margin-top: 10px; margin-bottom: 10px" {
            tr {
                td style="font-weight: bold" "Name:"
                td style="font-family: monospace; padding-left: 20px" (name)
            }
            tr {
                td style="font-weight: bold" "Key:"
                td style="font-family: monospace; padding-left: 20px" (key)
            }
            tr {
                td style="font-weight: bold" "Value:"
                td style="font-family: monospace; padding-left: 20px" (value)
            }
        }
    }
}

#[get("/internals/caches")]
pub fn caches_page(cache: State<Cache>) -> Markup {
    let content = html! {
        h3 style="margin-top: 0px" "Information"
        h3 "Brokers"
        (cache_description_table("BrokerCache", "ClusterId", "Vec<Broker>"))
        div (broker_table())
        h3 "Metrics"
        (cache_description_table("MetricsCache", "(ClusterId, BrokerId)", "BrokerMetrics"))
        div (metrics_table())
    };
    layout::page("Caches", content)
}
