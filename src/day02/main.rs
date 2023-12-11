use adventofcode2023::*;

main!();

#[derive(Debug)]
struct Game {
    id: i32,
    blue: i32,
    red: i32,
    green: i32,
}

impl Game {
    fn new(line: String) -> Self {
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        let id = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        line.split(":")
            .last()
            .unwrap()
            .replace(";", ",")
            .split(",")
            .for_each(|l| {
                let num = l.trim().split(" ").nth(0).unwrap().parse::<i32>().unwrap();

                match l.split(" ").last().unwrap() {
                    "red" => {
                        if max_red < num {
                            max_red = num
                        }
                    }
                    "green" => {
                        if max_green < num {
                            max_green = num
                        }
                    }
                    "blue" => {
                        if max_blue < num {
                            max_blue = num
                        }
                    }
                    _ => (),
                }
            });

        Game {
            id,
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
}

fn part1(input: Vec<String>) -> Result<()> {
    let mut sum = 0;
    for line in input {
        let game = Game::new(line);
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            sum += game.id;
        } else {
            println!("{:?}", game);
        }
    }

    println!("Sum: {sum}");
    Ok(())
}

fn part2(input: Vec<String>) -> Result<()>{
    let mut sum = 0;
    for line in input {
        let game = Game::new(line);
        sum += game.red * game.green * game.blue;
    }

    println!("Sum: {sum}");
    Ok(())
}
