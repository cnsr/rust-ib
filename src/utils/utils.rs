use sqlx::types::chrono::Utc;
use regex::Regex;

pub fn get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}

pub async fn parse_text(text: Option<String>) -> Option<Vec<i32>> {
    // this macro ensures that the regex is built only once per app runtime (i guess?)
    lazy_static! {
        // two capture groups - `arrows` and `number`
        static ref RE_REPLY: Regex = Regex::new(r"(?x)((?P<arrows>\>\>)(?P<number>\d+))").unwrap();
    }

    match text {
        Some(text) => {
            let static_text = Box::leak(text.into_boxed_str());

            // only `number` capture group is needed
            let maybe_replies: Result<Vec<i32>, _> = RE_REPLY.captures_iter(static_text)
                .map(|capture| capture.name("number").unwrap().as_str().parse::<i32>())
                .collect();
            // replies could be an error - shouldnt be, but still.
            match maybe_replies {
                Ok(replies) => {
                    if !replies.is_empty() {
                        Some(replies)
                    } else {None}
                },
                _ => None
            }
        },
        None => {None}
    }
}