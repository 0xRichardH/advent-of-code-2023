use std::ops::Neg;

use z3::{
    ast::{Ast, Int, Real},
    Config, Context, Solver,
};

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn process_data(input: &str) -> Option<usize> {
    let hails = input
        .trim()
        .lines()
        .flat_map(Hailstone::try_from)
        .take(3) // Optional
        .collect::<Vec<_>>();
    let hails_len = hails.len();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");
    let z = Real::new_const(&ctx, "z");
    let vx = Real::new_const(&ctx, "vx");
    let vy = Real::new_const(&ctx, "vy");
    let vz = Real::new_const(&ctx, "vz");

    let t: Vec<Real> = (0..hails_len)
        .map(|i| Real::new_const(&ctx, format!("T{}", i)))
        .collect();
    for i in 0..hails_len {
        let stone = &hails[i];
        let zero = &Real::from_int(&Int::from_i64(&ctx, 0));
        // x + t * vx = stone.x + t * stone.vx
        solver.assert(
            &Real::add(
                &ctx,
                &[
                    &x,
                    &Real::mul(&ctx, &[&t[i], &vx]),
                    &Real::from_int(&Int::from_i64(&ctx, stone.x as i64)).neg(),
                    &Real::mul(
                        &ctx,
                        &[
                            &t[i],
                            &Real::from_int(&Int::from_i64(&ctx, stone.vx as i64)),
                        ],
                    )
                    .neg(),
                ],
            )
            ._eq(zero),
        );

        // y + t * vy = stone.y + t * stone.vy
        solver.assert(
            &Real::add(
                &ctx,
                &[
                    &y,
                    &Real::mul(&ctx, &[&t[i], &vy]),
                    &Real::from_int(&Int::from_i64(&ctx, stone.y as i64)).neg(),
                    &Real::mul(
                        &ctx,
                        &[
                            &t[i],
                            &Real::from_int(&Int::from_i64(&ctx, stone.vy as i64)),
                        ],
                    )
                    .neg(),
                ],
            )
            ._eq(zero),
        );

        // z + t * vz = stone.z + t * stone.vz
        solver.assert(
            &Real::add(
                &ctx,
                &[
                    &z,
                    &Real::mul(&ctx, &[&t[i], &vz]),
                    &Real::from_int(&Int::from_i64(&ctx, stone.z as i64)).neg(),
                    &Real::mul(
                        &ctx,
                        &[
                            &t[i],
                            &Real::from_int(&Int::from_i64(&ctx, stone.vz as i64)),
                        ],
                    )
                    .neg(),
                ],
            )
            ._eq(zero),
        );
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model()?;
            let result = model.eval(&Real::add(&ctx, &[&x, &y, &z]), true)?;
            result
                .to_string()
                .parse::<f64>()
                .map(|f| f.abs() as usize)
                .ok()
        }
        _ => None,
    }
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
            )
            .unwrap(),
            47
        );
    }
}
