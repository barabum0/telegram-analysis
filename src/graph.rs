use std::collections::HashMap;

use chrono::Datelike;

use crate::tg_result::{Actor, TgExportMessage};

pub(crate) fn split_by_people<'a>(messages: impl Into<&'a Vec<TgExportMessage>>) -> HashMap<Actor, Vec<TgExportMessage>> {
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

pub(crate) fn count_by_days<'a>(messages_by_actor: impl Into<&'a HashMap<Actor, Vec<TgExportMessage>>>) -> HashMap<Actor, Vec<[f64; 2]>> {
    let mut result: HashMap<Actor, Vec<[f64; 2]>> = HashMap::new();

    for (actor, messages) in messages_by_actor.into() {
        let mut day_counts: HashMap<i32, u32> = HashMap::new();
        let mut min_day = i32::MAX;
        let mut max_day = i32::MIN;

        for message in messages {
            let date = message.get_date();
            let day_number = date.naive_utc().date().num_days_from_ce();

            min_day = min_day.min(day_number);
            max_day = max_day.max(day_number);

            *day_counts.entry(day_number).or_insert(0) += 1;
        }

        let mut points = Vec::new();
        for day in min_day..=max_day {
            let count = *day_counts.get(&day).unwrap_or(&0);
            points.push([day as f64, count as f64]);
        }

        result.insert(actor.clone(), points);
    }

    result
}