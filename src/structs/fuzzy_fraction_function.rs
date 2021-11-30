/// Approximate fraction. Fast and dirty.
/// ```
/// # use fuzzy_fraction::fuzzy_fraction;
/// assert_eq!(fuzzy_fraction(13, 26), (1, 2));
/// assert_eq!(fuzzy_fraction(1, 3), fuzzy_fraction(2, 6));
/// assert_eq!(fuzzy_fraction(3, 1), fuzzy_fraction(6, 2));
/// assert_eq!(fuzzy_fraction(4, 3), fuzzy_fraction(16, 12));
/// ```
pub fn fuzzy_fraction(n: usize, d: usize) -> (usize, usize) {
    match (n, d) {
        (0, 0) => return (0, 0),
        (0, _) => return (0, 1),
        (_, 0) => return (1, 0),
        (a, b)
            if ((a.max(b) - a.min(b)) as f32 / ((a + b) as f32 / 2.0) * 100.0) as usize <= 10 =>
        {
            return (1, 1)
        }

        (a, b) if a as f32 / b as f32 >= 100.0 => return (100, 1),
        (a, b) if b as f32 / a as f32 >= 100.0 => return (1, 100),
        (_, _) => (),
    };
    let switch = n < d;
    let max = n.max(d);
    let min = n.min(d);
    let float = max as f32 / min as f32;
    let whole = float.floor();
    let d = ((float - whole) * 100.0) as isize;
    if d < 10 || float.round() >= 10.0 {
        match switch {
            true => return (1, float.round() as usize),
            false => return (float.round() as usize, 1),
        };
    }

    let fraction = match d {
        1..=10 => (1, 10),
        11..=11 => (1, 9),
        12..=13 => (1, 8),
        14..=15 => (1, 7),
        16..=18 => (1, 6),
        19..=21 => (1, 5),
        22..=23 => (2, 9),
        24..=26 => (1, 4),
        27..=29 => (2, 7),
        30..=31 => (3, 10),
        32..=35 => (1, 3),
        36..=38 => (3, 8),
        39..=41 => (2, 5),
        42..=43 => (3, 7),
        44..=47 => (4, 9),
        48..=52 => (1, 2),
        53..=56 => (5, 9),
        57..=58 => (4, 7),
        59..=61 => (3, 5),
        62..=64 => (5, 8),
        65..=68 => (2, 3),
        69..=70 => (7, 10),
        71..=73 => (5, 7),
        74..=76 => (3, 4),
        77..=78 => (7, 9),
        79..=81 => (4, 5),
        82..=84 => (5, 6),
        85..=86 => (6, 7),
        87..=88 => (7, 8),
        89..=89 => (8, 9),
        _ => (9, 10),
    };

    let answer = (whole as usize * fraction.1 + fraction.0, fraction.1);
    if switch {
        return (answer.1, answer.0);
    }
    answer
}
