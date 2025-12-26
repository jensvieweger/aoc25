fn parse_string(line: &String) -> Option<i32> {
    let dist_str = &line[1..];
    let dir = line.chars().next()?;
    match dir {
        'L' => Some(-dist_str.parse::<i32>().ok()?),
        'R' => Some(dist_str.parse::<i32>().ok()?),
        _ => None,
    }
}

fn parse_data(data: &Vec<String>) -> Option<Vec<i32>> {
    let mut result:Vec<i32> = Vec::new();

    for line in data {
        let parsed_line = parse_string(line);
        if parsed_line.is_none() {
            return None;
        }
        result.push(parsed_line.unwrap());
    }
    Some(result)
}

fn rotate(pos:u8, dist:i32) -> Option<u8> {
    let pos32 = pos as i32;
    let check = (100+pos32+(dist % 100)) % 100;
    print!("check: {}\n", check);
    let dist_from_zero:u8 = (check).try_into().unwrap();
    Some(dist_from_zero)
}

pub fn get_day01(data: &Vec<String>) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();
        
        data.push("L68".to_string());
        data.push("L30".to_string());
        data.push("R48".to_string());
        data.push("L5".to_string());
        data.push("R60".to_string());
        data.push("L55".to_string());
        data.push("L1".to_string());
        data.push("L99".to_string());
        data.push("R14".to_string());
        data.push("L82".to_string());

        data
    }

    #[test]
    fn test_parse_data() {
        {
        let data = fill_input();

        let parsed_data = parse_data(&data);

        assert_eq!(parsed_data.is_none(), false);

        let final_data=parsed_data.unwrap();

        let expected_data:Vec<i32> = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(final_data, expected_data);
        }
    }

    #[test]
    fn test_get_day() {
        let data = fill_input();

        let day = get_day01(&data);

        assert_eq!(day.is_none(), false);

        let solution = day.unwrap();

        assert_eq!(solution, 3);

    }

    #[test]
    fn test_rotate() {

        {
        let rot = rotate(50, 50);

        assert_eq!(rot.is_none(), false);
        let solution = rot.unwrap();
        assert_eq!(solution, 0);
        }
        {
        let rot = rotate(50, -50);

        assert_eq!(rot.is_none(), false);
        let solution = rot.unwrap();
        assert_eq!(solution, 0);
        }
                {
        let rot = rotate(0, 50);

        assert_eq!(rot.is_none(), false);
        let solution = rot.unwrap();
        assert_eq!(solution, 50);
        }
                {
        let rot = rotate(0, -50);

        assert_eq!(rot.is_none(), false);
        let solution = rot.unwrap();
        assert_eq!(solution, 50);
        }
    }
}