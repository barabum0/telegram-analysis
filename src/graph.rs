use std::collections::HashMap;
use crate::tg_result::TgExportMessage;

pub(crate) fn split_by_people(messages: impl Into<Vec<TgExportMessage>>) -> HashMap<(u64, String), Vec<TgExportMessage>> {
    let mut data: HashMap<(u64, String), Vec<TgExportMessage>> = HashMap::new();
    for message in messages.into() {
        let key = (message.get_actor_id(), message.get_actor());
        if data.contains_key(&key) {
            data.get_mut(&key).unwrap().push(message);
        } else {
            data.insert(key, vec![message]);
        }
    };

    data
}