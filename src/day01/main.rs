use adventofcode2023::*;

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let mut sum = 0;

    for string in input {
        let mut first_num = None;
        let mut last_num = None;
        for i in 0..string.len() {
            let slice = string.get(i..=i).expect("string slice out of bounds");
            if let Ok(number) = slice.parse::<i32>() {
                if first_num == None {
                    first_num = Some(number);
                    last_num = Some(number);
                } else {
                    last_num = Some(number);
                }
            }
        }
        let num =
            first_num.expect("first_num not found") * 10 + last_num.expect("last_num not found");
        sum += num;
    }
    println!("Sum: {sum}");
    Ok(())
}


fn part2(input: Vec<String>) -> Result<()> {
    let num_words = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut sum = 0;
    let mut string_vector = Vec::new();

    for line in input {
        let mut l = line.clone();
        for (word, num) in num_words {
            let occurences: Vec<_> = line.match_indices(word).collect();
            for (pos, _) in occurences {
                l.insert(pos, num);
                l.remove(pos + 1);
            }
        }
        string_vector.push(l);
    }

    for string in string_vector {
        let mut first_num = None;
        let mut last_num = None;
        for i in 0..string.len() {
            let slice = string.get(i..=i).expect("string slice out of bounds");
            if let Ok(number) = slice.parse::<i32>() {
                if first_num == None {
                    first_num = Some(number);
                    last_num = Some(number);
                } else {
                    last_num = Some(number);
                }
            }
        }
        let num =
            first_num.expect("first_num not found") * 10 + last_num.expect("last_num not found");
        sum += num;
    }
    println!("Sum: {sum}");
    Ok(())
}
