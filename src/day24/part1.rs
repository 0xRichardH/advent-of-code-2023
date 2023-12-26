#![allow(dead_code)]

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn process_data(input: &str, min: usize, max: usize) -> usize {
    let hails = input
        .trim()
        .lines()
        .flat_map(Hailstone::try_from)
        .collect::<Vec<_>>();

    let mut counter = 0;

    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            if hails[i].in_area(&hails[j], min as f64, max as f64) {
                counter += 1;
            }
        }
    }

    counter
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
            .parse::<f64>()
            .map_err(|_| "Invalid x position".to_string())?;
        let y = position
            .next()
            .ok_or_else(|| "Invalid y position".to_string())?
            .parse::<f64>()
            .map_err(|_| "Invalid y position".to_string())?;
        let z = position
            .next()
            .ok_or_else(|| "Invalid z position".to_string())?
            .parse::<f64>()
            .map_err(|_| "Invalid z position".to_string())?;
        let vx = velocity
            .next()
            .ok_or_else(|| "Invalid x velocity".to_string())?
            .parse::<f64>()
            .map_err(|_| "Invalid x velocity".to_string())?;
        let vy = velocity
            .next()
            .ok_or_else(|| "Invalid y velocity".to_string())?
            .parse::<f64>()
            .map_err(|_| "Invalid y velocity".to_string())?;
        let vz = velocity
            .next()
            .ok_or_else(|| "Invalid z velocity".to_string())?
            .parse::<f64>()
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
    fn in_area(&self, stone: &Hailstone, min: f64, max: f64) -> bool {
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
        if determinant == 0f64 {
            return false;
        }

        let x = (b2 * c1 - b1 * c2) / determinant;
        let y = (a1 * c2 - a2 * c1) / determinant;

        let area = min..=max;

        let valid_a = (x > x1) == (x2 > x1);
        let valid_b = (x > x3) == (x4 > x3);

        area.contains(&x) && area.contains(&y) && valid_a && valid_b
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
