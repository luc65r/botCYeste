use std::time::Duration;

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};

use crate::Uptime;

#[command]
pub async fn uptime(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let time: Duration = {
        let data = ctx.data.read().await;
        let start_time = data.get::<Uptime>()
            .ok_or("cannot get global data uptime")?;
        start_time.elapsed()?
    };

    msg.reply(ctx, format!("Le bot est actif depuis {}.", format_uptime(time)))
        .await?;

    Ok(())
}

fn format_uptime(time: Duration) -> String {
    let mut left = time.as_secs();
    left /= 60;
    let minutes = left % 60;
    left /= 60;
    let hours = left % 24;
    left /= 24;
    let days = left;

    match [
        (days, "jour"),
        (hours, "heure"),
        (minutes, "minute"),
    ].iter().filter_map(|(n, s)| if *n == 0 {
        None
    } else {
        Some(format!("{} {}{}", n, s, if *n == 1 { "" } else { "s" }))
    }).collect::<Vec<String>>().as_slice() {
        [] => format!("0 minutes"),
        [a] => format!("{}", a),
        [a, b] => format!("{} et {}", a, b),
        [a, b, c] => format!("{}, {} et {}", a, b, c),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_uptime() {
        [
            (0, 0, 0, "0 minutes"),
            (0, 0, 1, "1 minute"),
            (0, 0, 2, "2 minutes"),
            (0, 0, 55, "55 minutes"),
            (0, 1, 0, "1 heure"),
            (0, 1, 1, "1 heure et 1 minute"),
            (0, 1, 34, "1 heure et 34 minutes"),
            (0, 2, 1, "2 heures et 1 minute"),
            (0, 2, 25, "2 heures et 25 minutes"),
            (0, 7, 44, "7 heures et 44 minutes"),
            (0, 19, 0, "19 heures"),
            (1, 0, 0, "1 jour"),
            (1, 0, 19, "1 jour et 19 minutes"),
            (1, 1, 1, "1 jour, 1 heure et 1 minute"),
            (4, 3, 0, "4 jours et 3 heures"),
            (15, 16, 17, "15 jours, 16 heures et 17 minutes"),
        ].iter().for_each(|(d, h, m, s)| {
            assert_eq!(
                format_uptime(Duration::from_secs(
                    ((d * 24 + h) * 60 + m) * 60
                )), *s
            );
        });
    }
}
