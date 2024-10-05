use std::collections::HashMap;
use chrono::Datelike;
use egui_plot::PlotPoint;
use crate::tg_result::{Actor, TgExportMessage};

pub(crate) fn split_by_people(messages: impl Into<&Vec<TgExportMessage>>) -> HashMap<Actor, Vec<TgExportMessage>> {
    let mut data: HashMap<Actor, Vec<TgExportMessage>> = HashMap::new();
    for message in messages.into() {
        let key = Actor::new(message.get_actor_id(), message.get_actor());
        if data.contains_key(&key) {
            data.get_mut(&key).unwrap().push(message.clone());
        } else {
            data.insert(key, vec![message.clone()]);
        }
    };

    data
}

pub(crate) fn count_by_days(messages_by_actor: impl Into<&HashMap<Actor, Vec<TgExportMessage>>>) -> HashMap<Actor, Vec<PlotPoint>> {
    let mut data: HashMap<Actor, Vec<PlotPoint>> = HashMap::new();
    for (actor, mut messages) in messages_by_actor.into() {
        messages.sort_by_key(|m| { m.get_date().num_days_from_ce() });
        data.insert(
            actor.into(),
            messages.chunk_by(
                |a, b| {
                    a.get_date().num_days_from_ce() == b.get_date().num_days_from_ce()
                }
            ).collect(),
        )
    };
    data
}