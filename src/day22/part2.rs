use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
};

#[derive(Debug, Clone)]
struct Coord3D {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone)]
struct Brick {
    start: Coord3D,
    end: Coord3D,
}

type Bricks = Vec<Brick>;

pub fn process_data(input: &str) -> usize {
    let mut bricks = parse_bricks(input);
    falling(&mut bricks);

    let (a_supports_b, b_supports_a) = get_support_relationships(&bricks);

    let mut total = 0;
    for i in 0..bricks.len() {
        total += simulate_chain_reaction(&a_supports_b, &b_supports_a, i);
    }

    total
}

fn parse_bricks(input: &str) -> Bricks {
    input
        .trim()
        .lines()
        .flat_map(|l| {
            let mut parts = l.trim().split('~');
            let Some(start_position) = parts.next() else {
                return None;
            };
            let Some(end_position) = parts.next() else {
                return None;
            };
            let start = Coord3D::try_from(start_position).ok()?;
            let end = Coord3D::try_from(end_position).ok()?;

            Some(Brick { start, end })
        })
        .collect()
}

fn falling(bricks: &mut Bricks) {
    sort_bricks(bricks);
    // dbg!(&bricks);

    // Adjusts each brick's position to simulate falling and settling.
    for (idx, brick) in bricks.clone().iter().enumerate() {
        let mut z = 1;
        for other in &bricks[..idx] {
            if brick.is_overlap(other) {
                z = z.max(other.end.z + 1);
            }
        }
        let Some(brick) = bricks.get_mut(idx) else {
            continue;
        };
        brick.end.z -= brick.start.z - z;
        brick.start.z = z;
    }
    sort_bricks(bricks);
    // dbg!(&bricks);
}

fn get_support_relationships(
    bricks: &Bricks,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    // Track Support Relationships
    let mut a_supports_b = (0..bricks.len())
        .map(|i| (i, HashSet::new()))
        .collect::<HashMap<usize, HashSet<usize>>>();
    let mut b_supports_a = a_supports_b.clone();
    for (j, upper) in bricks.clone().iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if upper.is_overlap(lower) && upper.start.z == lower.end.z + 1 {
                a_supports_b.entry(i).and_modify(|v| {
                    v.insert(j);
                });
                b_supports_a.entry(j).and_modify(|v| {
                    v.insert(i);
                });
            }
        }
    }
    (a_supports_b, b_supports_a)
}

fn sort_bricks(bricks: &mut Bricks) {
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
}

fn simulate_chain_reaction(
    a_support_b: &HashMap<usize, HashSet<usize>>,
    b_support_a: &HashMap<usize, HashSet<usize>>,
    idx: usize,
) -> usize {
    let mut q = VecDeque::new();
    let mut falling = HashSet::new();

    if let Some(supported_bricks) = a_support_b.get(&idx) {
        for j in supported_bricks {
            if b_support_a.get(j).map_or(false, |v| v.len() < 2) {
                q.push_back(*j);
                falling.insert(*j);
            }
        }
    }
    falling.insert(idx);

    while let Some(j) = q.pop_front() {
        let Some(supported_bricks) = a_support_b.get(&j) else {
            continue;
        };

        for k in supported_bricks - &falling {
            if b_support_a.get(&k).map_or(false, |v| v.is_subset(&falling)) {
                q.push_back(k);
                falling.insert(k);
            }
        }
    }

    falling.len() - 1
}

impl TryFrom<&str> for Coord3D {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let r: Option<Vec<usize>> = input.split(',').map(|s| s.parse::<usize>().ok()).collect();
        let Some(r) = r else {
            return Err(anyhow::anyhow!("Invalid input"));
        };
        let mut r = r.into_iter();
        let x = r.next().ok_or(anyhow::anyhow!("Invalid input"))?;
        let y = r.next().ok_or(anyhow::anyhow!("Invalid input"))?;
        let z = r.next().ok_or(anyhow::anyhow!("Invalid input"))?;

        Ok(Self { x, y, z })
    }
}

impl Display for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl Brick {
    /// Determines if two bricks overlap in the x and y dimensions
    fn is_overlap(&self, other: &Brick) -> bool {
        self.start.x.max(other.start.x) <= self.end.x.min(other.end.x)
            && self.start.y.max(other.start.y) <= self.end.y.min(other.end.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(7, process_data(input));
    }
}
