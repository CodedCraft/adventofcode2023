use adventofcode2023::*;

fn main() -> Result<()> {
    let file;
    match input() {
        Input::Sample => file = "sample.txt",
        Input::Input => file = "input.txt",
    }

    let input = parse(file)?;

    match run() {
        Part::Both => {part1(input.clone())?; part2(input)?;},
        Part::Part1 => part1(input)?,
        Part::Part2 => part2(input)?,
    };

    Ok(())
}

fn part1(input: Vec<String>) -> Result<()> {

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {

    Ok(())
}
