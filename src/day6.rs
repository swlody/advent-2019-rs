use std::io::{Error, ErrorKind};

struct System<'a> {
    orbits: std::collections::HashMap<&'a str, &'a str>,
}

impl<'a> System<'a> {
    fn from_string(string: &'a str) -> std::io::Result<Self> {
        Ok(Self {
            orbits: string
                .trim()
                .split('\n')
                .map(|orbit| {
                    let pair: Vec<&str> = orbit.split(')').collect();
                    if pair.len() == 2 {
                        Ok((pair[1], pair[0]))
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Invalid planet pair format",
                        ))
                    }
                })
                .collect::<std::io::Result<_>>()?,
        })
    }

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

    fn transfer_distance(&self, source: &str, destination: &str) -> usize {
        let source_to_com = self.find_path(source, "COM").unwrap();
        let destination_to_com = self.find_path(destination, "COM").unwrap();
        let lengths = source_to_com.len() + destination_to_com.len();

        let intersection = source_to_com
            .into_iter()
            .rev()
            .zip(destination_to_com.into_iter().rev())
            .take_while(|(source_planet, destination_planet)| source_planet == destination_planet)
            .count();

        lengths - (intersection * 2) - 2
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> std::io::Result<usize> {
    let system = System::from_string(&input)?;

    Ok(system.count_orbits())
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> std::io::Result<usize> {
    let system = System::from_string(&input)?;

    Ok(system.transfer_distance("YOU", "SAN"))
}
