use std::convert::TryFrom;
use std::io::{Error, ErrorKind};

struct System<'a> {
    orbits: std::collections::HashMap<&'a str, &'a str>,
}

impl<'a> System<'a> {
    fn find_path(&self, source: &'a str, destination: &'a str) -> Option<Vec<&'a str>> {
        let mut path = vec![source];

        let mut current = self.orbits.get(source)?;
        while *current != destination {
            path.push(*current);
            current = self.orbits.get(current)?;
        }

        Some(path)
    }

    fn count_orbits(&self) -> usize {
        self.orbits.keys().fold(0, |acc, planet| {
            acc + self.find_path(planet, "COM").unwrap().len()
        })
    }

    fn transfer_distance(&self, source: &str, destination: &str) -> Option<usize> {
        let source_to_com = self.find_path(source, "COM")?;
        let destination_to_com = self.find_path(destination, "COM")?;
        let lengths = source_to_com.len() + destination_to_com.len();

        let intersection = source_to_com
            .into_iter()
            .rev()
            .zip(destination_to_com.into_iter().rev())
            .take_while(|(source_planet, destination_planet)| source_planet == destination_planet)
            .count();

        Some(lengths - (intersection * 2) - 2)
    }
}

impl<'a> TryFrom<&'a str> for System<'a> {
    type Error = std::io::Error;

    fn try_from(system: &'a str) -> Result<Self, Error> {
        Ok(Self {
            orbits: system
                .trim()
                .split('\n')
                .map(|orbit| {
                    let pair = orbit.split(')').collect::<Vec<_>>();
                    if pair.len() == 2 {
                        Ok((pair[1], pair[0]))
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Invalid planet pair format",
                        ))
                    }
                })
                .flatten()
                .collect(),
        })
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> std::io::Result<usize> {
    let system = System::try_from(input)?;

    Ok(system.count_orbits())
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> std::io::Result<usize> {
    let system = System::try_from(input)?;

    system
        .transfer_distance("YOU", "SAN")
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "No path from YOU to SAN"))
}
