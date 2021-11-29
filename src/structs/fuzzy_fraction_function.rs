pub fn fuzzy_fraction(n: usize, d: usize) -> (usize, usize) {
    match (n, d) {
        (0, 0) => return (0, 0),
        (0, _) => return (0, 1),
        (_, 0) => return (1, 0),
        // (a, b)
        //     if ((a.max(b) - a.min(b)) as f32 / ((a + b) as f32 / 2.0) * 100.0) as usize <= 10 =>
        // {
        //     return (1, 1)
        // }
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
    let fraction = {
        if d < 47 {
            if d < 25 {
                if d < 16 {
                    if d < 12 {
                        if d < 11 {
                            (1, 10)
                        } else {
                            (1, 9)
                        }
                    } else if d < 14 {
                        (1, 8)
                    } else {
                        (1, 7)
                    }
                } else if d < 19 {
                    (1, 6)
                } else if d < 22 {
                    (1, 5)
                } else {
                    (2, 9)
                }
            } else if d < 37 {
                if d < 28 {
                    (1, 4)
                } else if d < 31 {
                    (2, 7)
                } else {
                    (1, 3)
                }
            } else if d < 42 {
                if d < 40 {
                    (3, 8)
                } else {
                    (2, 5)
                }
            } else if d < 44 {
                (3, 7)
            } else {
                (4, 9)
            }
        } else if d < 71 {
            if d < 60 {
                if d < 55 {
                    (1, 2)
                } else if d < 57 {
                    (5, 9)
                } else {
                    (4, 7)
                }
            } else if d < 62 {
                (3, 5)
            } else if d < 66 {
                (5, 8)
            } else {
                (2, 3)
            }
        } else if d < 80 {
            if d < 74 {
                (5, 7)
            } else if d < 77 {
                (3, 4)
            } else {
                (7, 9)
            }
        } else if d < 85 {
            if d < 83 {
                (4, 5)
            } else {
                (5, 6)
            }
        } else if d < 87 {
            (6, 7)
        } else if d < 88 {
            (7, 8)
        } else if d < 90 {
            (8, 9)
        } else {
            (9, 10)
        }
    };

    let answer = (whole as usize * fraction.1 + fraction.0, fraction.1);
    if switch {
        return (answer.1, answer.0);
    }
    answer
}
