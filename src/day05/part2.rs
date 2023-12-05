use std::ops::Range;

use anyhow::Result;
use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

pub fn process_data(input: &str) -> Result<u64> {
    let (input, seeds) = parse_seeds(input).expect("Parse seeds failed");
    let (_, maps) = parse_maps(input).expect("Parse maps failed");

    let mut short_location = u64::MAX;
    seeds.into_iter().for_each(|seed| {
        short_location = find_location(&maps, seed).min(short_location);
    });

    Ok(short_location)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;

    let seeds = seeds
        .chunks_exact(2)
        .map(|item| {
            let start = item[0];
            let len = item[1];
            start..start + len
        })
        .collect::<Vec<Range<u64>>>();

    Ok((input, seeds))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Vec<[Range<u64>; 2]>>> {
    let (input, maps) = many1(
        take_until("map:").precedes(tag("map:")).precedes(many1(
            line_ending
                .precedes(separated_list1(space1, complete::u64))
                .map(|item| {
                    let dest = item[0];
                    let src = item[1];
                    let len = item[2];
                    let src_range = src..src + len;
                    let dest_range = dest..dest + len;
                    [src_range, dest_range]
                }),
        )),
    )
    .parse(input)?;

    Ok((input, maps))
}

fn find_location(maps: &[Vec<[Range<u64>; 2]>], seed_range: Range<u64>) -> u64 {
    let mut seeds = vec![seed_range];
    for map in maps {
        let mut new_seeds = Vec::new();
        for seed in seeds {
            let mut found = false;
            for [src, dest] in map {
                let os = seed.start.max(src.start);
                let oe = seed.end.min(src.end);
                if os < oe {
                    // dbg!("========================", &seed, &src, s, e, &dest);
                    let len = os - src.start;
                    new_seeds.push(dest.start + len..dest.start + len + oe - os);
                    if os > seed.start {
                        new_seeds.push(seed.start..os);
                    }
                    if oe < seed.end {
                        new_seeds.push(oe..seed.end);
                    }
                    found = true;
                    // dbg!(&new_seeds);
                    break;
                }
            }

            if !found {
                new_seeds.push(seed);
            }
        }
        seeds = new_seeds;
    }

    // dbg!(&seeds);
    let mut min_location = seeds[0].start;
    seeds.into_iter().for_each(|seed| {
        if seed.start < min_location {
            min_location = seed.start;
        }
    });
    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_process_data() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, process_data(input).unwrap());
    }
}
