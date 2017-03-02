use iron::prelude::{Request, Response};
use router::Router;
use iron::{IronResult, status};
use maud::PreEscaped;

use web_server::server::{CacheType, RequestTimer};
use web_server::view::layout;
use web_server::pages;
use metadata::{Broker, Partition};
use cache::MetricsCache;

use std::collections::HashMap;


fn format_broker_list(cluster_id: &str, brokers: &Vec<i32>) -> PreEscaped<String> {
    html! {
        @for (n, broker) in brokers.iter().enumerate() {
            a href=(format!("/clusters/{}/broker/{}/", cluster_id, broker)) (broker)
            @if n < brokers.len() - 1 { ", " }
        }
    }
}

fn topic_table_row(cluster_id: &str, partition: &Partition) -> PreEscaped<String> {
    let status = if partition.error.is_none() {
        html!{ i class="fa fa-check fa-fw" style="color: green" {} }
    } else {
        //html!{ i class="fa fa-exclamation-triangle fa-fw" style="color: yellow" {} }
        html!{ i class="fa fa-times fa-fw" style="color: red" {} (partition.error.clone().unwrap()) }
    };
    html! {
        tr {
            td (partition.id)
            td a href=(format!("/clusters/{}/broker/{}/", cluster_id, partition.leader)) (partition.leader)
            td (format_broker_list(cluster_id, &partition.replicas))
            td (format_broker_list(cluster_id, &partition.isr))
            td (status)
        }
    }
}

fn topic_table(cluster_id: &str, partitions: &Vec<Partition>) -> PreEscaped<String> {
    let table = layout::datatable (true, "topology",
        html! { tr { th "Id" th "Leader" th "Replicas" th "ISR" th "Status" } },
        html! { @for partition in partitions.iter() { (topic_table_row(cluster_id, partition)) }}
    );
    html!(
        span class="loader-parent-marker" {
            div class="table-loader-marker" style="text-align: center; padding: 0.3in;" { }
            (table)
        }
    )
}

pub fn topic_page(req: &mut Request) -> IronResult<Response> {
    let cache = req.extensions.get::<CacheType>().unwrap();
    let cluster_id = req.extensions.get::<Router>().unwrap().find("cluster_id").unwrap();
    let topic_name = req.extensions.get::<Router>().unwrap().find("topic_name").unwrap();

    let partitions = match cache.topics.get(&(cluster_id.to_owned(), topic_name.to_owned())) {
        Some(partitions) => partitions,
        None => {
            return pages::warning_page(req,
                &format!("Topic: {}", cluster_id),
                "The specified cluster doesn't exist.")
        }
    };

    let brokers = cache.brokers.get(&cluster_id.to_owned()).expect("Broker should exist");

    let metrics = pages::cluster::build_topic_metrics(&cluster_id, &brokers, 100, &cache.metrics)
        .get(topic_name).cloned();
    let content = html! {
        h3 style="margin-top: 0px" "General information"
        dl class="dl-horizontal" {
            dt "Cluster name " dd (cluster_id)
            dt "Topic name " dd (topic_name)
            dt "Number of partitions " dd (partitions.len())
            dt "Number of replicas " dd (partitions[0].replicas.len())
            @if metrics.is_some() {
                dt "Traffic last 15 minutes"
                dd (format!("{:.1} KB/s {:.0} msg/s", metrics.unwrap().0 / 1000f64, metrics.unwrap().1))
            } @else {
                dt "Traffic data" dd "Not available"
            }
        }
        h3 "Topology"
        (topic_table(cluster_id, &partitions))
        h3 "Active consumers"
        p "Coming soon."
    };

    let html = layout::page(req, &format!("Topic: {}", topic_name), content);

    Ok(Response::with((status::Ok, html)))
}

