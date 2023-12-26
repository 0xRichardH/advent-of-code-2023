#![allow(dead_code)]

use itertools::iproduct;

#[derive(Debug)]
struct Hailstone {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

pub fn process_data(input: &str, min: isize, max: isize) -> usize {
    let hails = input
        .trim()
        .lines()
        .flat_map(Hailstone::try_from)
        .collect::<Vec<_>>();

    println!("{:?}", hails);

    iproduct!(hails.iter(), hails.iter().skip(1)).fold(0, |acc, (a, b)| {
        println!("{:?} {:?}", a, b);
        if a.in_area(b, min, max).unwrap_or(false) {
            acc + 1
        } else {
            acc
        }
    })
}

impl TryFrom<&str> for Hailstone {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut parts = line
            .trim()
            .split('@')
            .map(|part| part.trim().split(',').map(|s| s.trim()));
        let mut position = parts.next().ok_or_else(|| "Invalid position".to_string())?;
        let mut velocity = parts.next().ok_or_else(|| "Invalid velocity".to_string())?;

        let x = position
            .next()
            .ok_or_else(|| "Invalid x position".to_string())?
            .parse::<isize>()
            .map_err(|_| "Invalid x position".to_string())?;
        let y = position
            .next()
            .ok_or_else(|| "Invalid y position".to_string())?
            .parse::<isize>()
            .map_err(|_| "Invalid y position".to_string())?;
        let z = position
            .next()
            .ok_or_else(|| "Invalid z position".to_string())?
            .parse::<isize>()
            .map_err(|_| "Invalid z position".to_string())?;
        let vx = velocity
            .next()
            .ok_or_else(|| "Invalid x velocity".to_string())?
            .parse::<isize>()
            .map_err(|_| "Invalid x velocity".to_string())?;
        let vy = velocity
            .next()
            .ok_or_else(|| "Invalid y velocity".to_string())?
            .parse::<isize>()
            .map_err(|_| "Invalid y velocity".to_string())?;
        let vz = velocity
            .next()
            .ok_or_else(|| "Invalid z velocity".to_string())?
            .parse::<isize>()
            .map_err(|_| "Invalid z velocity".to_string())?;

        Ok(Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        })
    }
}

impl Hailstone {
    fn in_area(&self, stone: &Hailstone, min: isize, max: isize) -> Option<bool> {
        let x1 = self.x;
        let x2 = self.x + self.vx;
        let x3 = stone.x;
        let x4 = stone.x + stone.vx;
        let y1 = self.y;
        let y2 = self.y + self.vy;
        let y3 = stone.y;
        let y4 = stone.y + stone.vy;

        // https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
        let a1 = y2 - y1;
        let b1 = x1 - x2;
        let c1 = a1 * x1 + b1 * y1;

        let a2 = y4 - y3;
        let b2 = x3 - x4;
        let c2 = a2 * x3 + b2 * y3;

        let determinant = a1 * b2 - a2 * b1;
        let x = (b2 * c1 - b1 * c2).checked_div(determinant)?;
        let y = (a1 * c2 - a2 * c1).checked_div(determinant)?;

        let area = min..=max;

        let valid_a = (x > x1) == (x2 > x1);
        let valid_b = (x > x3) == (x4 > x3);

        if area.contains(&x) && area.contains(&y) && valid_a && valid_b {
            println!("{} {}", x, y);
        }

        Some(area.contains(&x) && area.contains(&y) && valid_a && valid_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        assert_eq!(
            process_data(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
                7,
                27
            ),
            2
        );
    }
}
