use std::fs::read_to_string;

fn main() {
    let input: String = read_to_string("input").unwrap();

    let mut total_valid_game_id = 0;
    let mut power_of_sets = 0;

    for line in input.lines() {
        let (game_idx, record_info) = line.split_once(':').unwrap();
        let game_idx: u32 = game_idx.split_once(' ').unwrap().1.parse().unwrap();

        /* part 1 */
        let mut ok = true;
        /* part 2 */
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let records: Vec<&str> = record_info.split(';').collect();
        for record in records {
            let cubes: Vec<&str> = record.split(',').collect();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for cube in cubes {
                let (qty, color) = cube.trim().split_once(' ').unwrap();
                let qty: u32 = qty.parse().unwrap();
                match color {
                    "red" => {
                        red += qty;
                    }
                    "green" => {
                        green += qty;
                    }
                    "blue" => {
                        blue += qty;
                    }
                    _ => {}
                }
            }
            /* part 1 */
            if red > 12 || green > 13 || blue > 14 {
                ok = false;
            }
            /* part 2 */
            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }
        if ok {
            total_valid_game_id += game_idx;
        }
        power_of_sets += max_red * max_green * max_blue;
    }
    println!("{}", total_valid_game_id);
    println!("{}", power_of_sets);
}
