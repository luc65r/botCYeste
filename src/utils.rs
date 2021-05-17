use std::time::Duration;

pub fn format_duration(time: Duration) -> String {
    let mut left = time.as_secs();
    let seconds = left % 60;
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
        (seconds, "seconde"),
    ].iter().filter_map(|(n, s)| if *n == 0 {
        None
    } else {
        Some(format!("{} {}{}", n, s, if *n == 1 { "" } else { "s" }))
    }).collect::<Vec<String>>().as_slice() {
        [] => format!("0 secondes"),
        [a] => format!("{}", a),
        [a, b] => format!("{} et {}", a, b),
        [a, b, c] => format!("{}, {} et {}", a, b, c),
        [a, b, c, d] => format!("{}, {}, {} et {}", a, b, c, d),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        [
            (0, 0, 0, 0, "0 secondes"),
            (0, 0, 0, 1, "1 seconde"),
            (0, 0, 0, 5, "5 secondes"),
            (0, 0, 1, 0, "1 minute"),
            (0, 0, 2, 0, "2 minutes"),
            (0, 0, 55, 12, "55 minutes et 12 secondes"),
            (0, 1, 0, 0, "1 heure"),
            (0, 1, 1, 0, "1 heure et 1 minute"),
            (0, 1, 34, 0, "1 heure et 34 minutes"),
            (0, 2, 1, 1, "2 heures, 1 minute et 1 seconde"),
            (0, 2, 25, 0, "2 heures et 25 minutes"),
            (0, 7, 0, 44, "7 heures et 44 secondes"),
            (0, 19, 0, 0, "19 heures"),
            (1, 0, 0, 0, "1 jour"),
            (1, 0, 19, 0, "1 jour et 19 minutes"),
            (1, 1, 1, 0, "1 jour, 1 heure et 1 minute"),
            (4, 3, 0, 0, "4 jours et 3 heures"),
            (15, 16, 17, 18, "15 jours, 16 heures, 17 minutes et 18 secondes"),
        ].iter().for_each(|(d, h, m, s, st)| {
            assert_eq!(
                format_duration(Duration::from_secs(
                    ((d * 24 + h) * 60 + m) * 60 + s
                )), *st
            );
        });
    }
}
