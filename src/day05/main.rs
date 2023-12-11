use adventofcode2023::*;

main!();

#[derive(Debug, Clone)]
struct Component {
    type_nums: Vec<i64>,
    map: Vec<(i64, i64, i64)>,
}

impl Component {
    fn new() -> Self {
        Component {
            type_nums: Vec::new(),
            map: Vec::new(),
        }
    }
}

fn part1(input: Vec<String>) -> Result<()> {

    let mut components = vec![Component::new(); 7];
    // |-------|-------------|
    // | Index | Type        |
    // |-------|-------------|
    // | 0     | Seed        |
    // | 1     | Soil        |
    // | 2     | Fertilizer  |
    // | 3     | Water       |
    // | 4     | Light       |
    // | 5     | Temperature |
    // | 6     | Humidity    |
    // | 7     | Location    |
    // |-------|-------------|

    // Parse seed type numbers
    for seed in input[0]
        .split(":")
        .nth(1)
        .expect("Could not get split")
        .split_whitespace()
        .into_iter()
    {
        components[0]
            .type_nums
            .push(seed.parse().expect("Seed num couldn't be parsed"));
    }


    // Parse Maps
    let map_types = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    for (component_idx, &map_type) in map_types.iter().enumerate() {
        let start_idx = 1 + input
            .iter()
            .position(|line| line == map_type)
            .expect(&format!("Could not find '{}'", map_type));

        let end_idx = start_idx
            + input[start_idx..]
                .iter()
                .position(|line| line.trim().is_empty())
                .expect("Map end not found");

        for line in input[start_idx..end_idx].iter() {
            let dest = line.split_whitespace().nth(0).unwrap().parse::<i64>()?;
            let src = line.split_whitespace().nth(1).unwrap().parse::<i64>()?;
            let rng = line.split_whitespace().nth(2).unwrap().parse::<i64>()?;

            let low = src;
            let high = src + rng - 1;
            let offset = dest - src;


            components[component_idx].map.push((low, high, offset));
        }
    }

    // Calculate type numbers
    let mut current_types = Vec::new();
    for component in &mut components {
        if component.type_nums.len() == 0 {
            component.type_nums = current_types.clone();
        }
        let mut next_types = Vec::new();
        for &type_num in &component.type_nums {
            let mut in_map = false;
            for &(low, high, offset) in &component.map {
                if type_num >= low && type_num <= high {
                    next_types.push(type_num + offset);
                    in_map = true;
                    break;
                }
            }
            if !in_map {
                next_types.push(type_num);
            }
        }
        current_types = next_types;
    }

    // println!("{:?}", components);
    println!("{}", current_types.iter().min().unwrap());

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    let mut components = vec![Component::new(); 7];
    // |-------|-------------|
    // | Index | Type        |
    // |-------|-------------|
    // | 0     | Seed        |
    // | 1     | Soil        |
    // | 2     | Fertilizer  |
    // | 3     | Water       |
    // | 4     | Light       |
    // | 5     | Temperature |
    // | 6     | Humidity    |
    // | 7     | Location    |
    // |-------|-------------|

    // Parse seed type numbers
    let mut seeds = Vec::new();
    for num in input[0]
        .split(":")
        .nth(1)
        .expect("Could not get split")
        .split_whitespace()
        .into_iter()
    {
        seeds.push(num.parse::<i64>()?);
    }

    // Parse Maps
    let map_types = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    for (component_idx, &map_type) in map_types.iter().enumerate() {
        let start_idx = 1 + input
            .iter()
            .position(|line| line == map_type)
            .expect(&format!("Could not find '{}'", map_type));

        let end_idx = start_idx
            + input[start_idx..]
                .iter()
                .position(|line| line.trim().is_empty())
                .expect("Map end not found");

        for line in input[start_idx..end_idx].iter() {
            let dest = line.split_whitespace().nth(0).unwrap().parse::<i64>()?;
            let src = line.split_whitespace().nth(1).unwrap().parse::<i64>()?;
            let rng = line.split_whitespace().nth(2).unwrap().parse::<i64>()?;

            let low = src;
            let high = src + rng - 1;
            let offset = dest - src;

            components[component_idx].map.push((low, high, offset));
        }
    }

    let mut overall_min = i64::MAX;

    for chunk in seeds.chunks_exact(2) {
        let start = chunk[0];
        let end = start + chunk[1];

        components[0].type_nums = (start..end).collect();

        // Calculate type numbers
        let mut current_types = components[0].type_nums.clone();
        for component in &mut components {
            // if component.type_nums.len() == 0 {
                component.type_nums = current_types.clone();
            // }
            let mut next_types = Vec::new();
            for &type_num in &component.type_nums {
                let mut in_map = false;
                for &(low, high, offset) in &component.map {
                    if type_num >= low && type_num <= high {
                        next_types.push(type_num + offset);
                        in_map = true;
                        break;
                    }
                }
                if !in_map {
                    next_types.push(type_num);
                }
            }
            current_types = next_types;
        }
        let current_min = *current_types.iter().min().unwrap();
        overall_min = current_min.min(overall_min); 
    }
    println!("Result: {}", overall_min);

    Ok(())
}
