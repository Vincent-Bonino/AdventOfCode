const START_POSITION: i32 = 50;

pub fn solve_both_parts(rotations: &[i32]) -> (String, String) {
    let (part1, part2, _final): (i32, i32, i32) =
        rotations
            .iter()
            .fold((0, 0, START_POSITION), |(part1, part2, curr), rot| {
                let new: i32 = (curr + rot + 100) % 100;
                let mut part1 = part1;
                let mut part2 = part2;

                // Part 1
                if new == 0 {
                    part1 += 1;
                }

                // Part 2

                // Count full rotations
                part2 += (rot / 100).abs();
                let rot = rot % 100;

                part2 += match (curr.signum(), rot.signum()) {
                    (0, _) => 0, // No additional rotation possible
                    (1, 1) => {
                        if curr + rot >= 100 {
                            1
                        } else {
                            0
                        }
                    }
                    (1, -1) => {
                        if curr + rot <= 0 {
                            1
                        } else {
                            0
                        }
                    }
                    (-1, 1) => {
                        if curr + rot >= 0 {
                            1
                        } else {
                            0
                        }
                    }
                    (-1, -1) => {
                        if curr + rot <= -100 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };

                (part1, part2, new)
            });

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_part_one() {
        assert_eq!(7 % 100, 7);
        assert_eq!(-7 % 100, -7); // !!
        assert_eq!((-7 + 100) % 100, 93);
    }

    #[test]
    fn test_modulo_part_two() {
        assert_eq!((-99 / 100i16).abs(), 0);
        assert_eq!((123 / 100i16).abs(), 1);
        assert_eq!((-654 / 100i16).abs(), 6);
        assert_eq!(-654 % 100i16, -54);
    }
}
