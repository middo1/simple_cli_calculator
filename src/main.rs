use std::io;
fn main() {
    let list_of_operations = [
        "Addition",
        "Subtraction",
        "Multiplication",
        "Division",
        "Exponent",
        "Free Calc",
        "CPGA Caculator",
    ];
    loop {
        let mut opt = String::new();
        println!("Welcome to my simple CLI calculator");
        println!("Select the operation that you want to perform, enter q to exit");
        let mut ctr = 0;
        while ctr < list_of_operations.len() {
            println!("{}. {}\n", ctr + 1, list_of_operations[ctr]);
            ctr += 1;
        }
        let _ = io::stdin()
            .read_line(&mut opt)
            .expect("Na rubbish you dey do");
        if opt.trim() == "q" {
            println!("\nThank you for using this CLI calculator\n");
            break;
        }
        let opt: usize = opt
            .trim()
            .parse::<usize>()
            .expect("Please enter a valid Option\n");
        let operations = [" + ", " - ", " * ", " / ", " ** "];
        if opt <= 5 && opt > 0 {
            println!(
                "Enter the values you want to operate on and seperate the value using '{}' like so: 1{}2{}3{}4",
                operations[opt - 1],
                operations[opt - 1],
                operations[opt - 1],
                operations[opt - 1]
            );
            let mut space_sep_values = String::new();
            let _ = io::stdin()
                .read_line(&mut space_sep_values)
                .expect("Na rubbish you dey do");
            let val = handle_space_sep_values(space_sep_values);
            let mut final_res = val.clone();
            let mut clean = 0;
            'get_final: loop {
                let mut p: usize = 0;
                while p < final_res.len() {
                    if final_res[p].contains("(") {
                        final_res = handle_bodmas(final_res);
                        println!("{:?}", final_res)
                    } else {
                        clean += 1;
                        println!("e enter");
                    }
                    p += 1;
                }
                if clean == final_res.len() {
                    break 'get_final;
                } else {
                    clean = 0;
                }
                // println!("final{:?}", final_res);
            }
            println!("Final: {:?}", final_res);
            let res = handle_values(final_res);
            println!("The result is {}\n \n", res);
            println!("Do you want to continue? (Y, N)");
            let mut choice = String::new();
            let _ = io::stdin()
                .read_line(&mut choice)
                .expect("Na rubbish you dey do");
            if choice.trim() == "N" {
                println!("\nThank you for using this CLI calculator\n");
                break;
            }
        } else if opt == 7 {
            println!("Enter the amount of courses you offer");
            let mut num_of_courses = String::new();
            let _ = io::stdin()
                .read_line(&mut num_of_courses)
                .expect("Na rubbish you dey do");
            let num_of_courses: u32 = num_of_courses
                .trim()
                .parse()
                .expect("Please enter a valid number of Courses");
            let mut count: u32 = 0;
            let mut list_grade_points: Vec<f64> = vec![];
            let mut list_credit_hours: Vec<f64> = vec![];
            while count < num_of_courses {
                println!("Enter the score for the number {} course", count + 1);
                let mut score = String::new();
                let _ = io::stdin()
                    .read_line(&mut score)
                    .expect("Enter a valid Score");
                let score: f64 = score.trim().parse().expect("Enter a valid score");
                println!(
                    "Enter the credit weight for the number {} course",
                    count + 1
                );
                let mut credit_weight = String::new();
                let _ = io::stdin()
                    .read_line(&mut credit_weight)
                    .expect("Enter a valid Score");
                let credit_weight: f64 = credit_weight.trim().parse().expect("Enter a valid score");
                list_credit_hours.push(credit_weight);
                list_grade_points.push(score * credit_weight);
                count += 1;
            }
            let total_grade_point = addition(list_grade_points);
            let total_credit_hours = addition(list_credit_hours);
            println!(
                "Your CPGA is {}\n \n",
                cpga_calculator(total_grade_point, total_credit_hours)
            );
            println!("Do you want to continue? (Y, N)");
            let mut choice = String::new();
            let _ = io::stdin()
                .read_line(&mut choice)
                .expect("Na rubbish you dey do");
            if choice.trim() == "N" {
                println!("\nThank you for using this CLI calculator\n");
                break;
            }
        } else if opt == 6 {
            println!(
                "Enter the values you want to perform several operations and seperate the value using ' ' and the like so: 1 + 2 * 3 / 4 \nAnd you can also use brakets like so: (2 + 2) - ((4 - 2) / (2 + 3)) \nPlease make sure the operations are seperated with a whitespace from the bracket and the numbers \naAnd make sure your brakets are well closed"
            );
            let mut space_sep_values = String::new();
            let _ = io::stdin()
                .read_line(&mut space_sep_values)
                .expect("Na rubbish you dey do");
            let val = handle_space_sep_values(space_sep_values);
            let mut final_res = val.clone();
            println!("{:?}", final_res);
            let mut clean = 0;
            if final_res[0] == "" {
                panic!("Empty or Invalid Expression");
            }
            'get_final: loop {
                let mut p: usize = 0;
                while p < final_res.len() {
                    if final_res[p].contains("(") {
                        final_res = handle_bodmas(final_res);
                        if final_res[0] == "" {
                            panic!("Empty or Invalid Expression");
                        }
                        println!("{:?}", final_res)
                    } else {
                        clean += 1;
                        // println!("e enter");
                    }
                    p += 1;
                }
                if clean == final_res.len() {
                    break 'get_final;
                } else {
                    clean = 0;
                }
                // println!("final{:?}", final_res);
            }
            // println!("for main: {:?}", final_res);
            let res = handle_values(final_res);
            println!("The result is {}\n \n", res);
            println!("Do you want to continue? (Y, N)");
            let mut choice = String::new();
            let _ = io::stdin()
                .read_line(&mut choice)
                .expect("Na rubbish you dey do");
            if choice.trim() == "N" {
                println!("\nThank you for using this CLI calculator\n");
                break;
            }
        } else {
            println!("Please enter one of the given options")
        }
    }
}

fn addition(values: Vec<f64>) -> f64 {
    let mut res = 0.0;
    for x in values {
        res += x;
    }
    return res;
}

fn subtraction(values: Vec<f64>) -> f64 {
    let mut res = values[0];
    let mut i = 0;
    while i < values.len() {
        if i != 0 {
            res -= values[i];
        }
        i += 1
    }
    return res;
}

fn multiplication(values: Vec<f64>) -> f64 {
    let mut res = 1.0;
    for x in values {
        res *= x;
    }
    return res;
}

fn division(values: Vec<f64>) -> f64 {
    let mut res = values[0];
    let mut i = 0;
    while i < values.len() {
        if i != 0 {
            res /= values[i];
        }
        i += 1
    }
    return res;
}

fn cpga_calculator(total_grade_point: f64, total_credit_hours: f64) -> f64 {
    total_grade_point / total_credit_hours
}

fn handle_space_sep_values(space_sep_values: String) -> Vec<String> {
    let values: Vec<String> = space_sep_values.split(" ").map(|f| f.trim().parse::<String>().expect("You entered an invalid input, please make sure that the values are integers and are seperated by a space")).collect();
    return values;
}

fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n < 0 && n % 2 == 0 {
        return my_pow(x * x, n / 2);
    } else if n < 0 && n % 2 == -1 {
        if n == -1 {
            return my_pow(1.0 / x * 1.0 / x, n / 2) * 1.0 / x;
        }
        return my_pow(x * x, n / 2) * 1.0 / x;
    }
    if n % 2 == 0 {
        return my_pow(x * x, n / 2);
    } else {
        return my_pow(x * x, n / 2) * x;
    }
}

fn pow(values: Vec<f64>) -> f64 {
    my_pow(values[0], values[1] as i32)
}

fn handle_values(mut values: Vec<String>) -> f64 {
    let mut ptr: usize = 0;
    let operations = ["**", "*", "/", "+", "-"];
    // println!("{:?}", values);
    for operation in operations {
        'operation: loop {
            if values.len() == 1 {
                break;
            } else if ptr == values.len() {
                break 'operation;
            } else if values[ptr] == operation {
                let new_val = match operation {
                    "**" => pow(vec![
                        values[ptr - 1].parse::<f64>().unwrap(),
                        values[ptr + 1].parse::<f64>().unwrap(),
                    ]),
                    "*" => multiplication(vec![
                        values[ptr - 1].parse::<f64>().unwrap(),
                        values[ptr + 1].parse::<f64>().unwrap(),
                    ]),
                    "/" => division(vec![
                        values[ptr - 1].parse::<f64>().unwrap(),
                        values[ptr + 1].parse::<f64>().unwrap(),
                    ]),
                    "+" => addition(vec![
                        values[ptr - 1].parse::<f64>().unwrap(),
                        values[ptr + 1].parse::<f64>().unwrap(),
                    ]),
                    "-" => subtraction(vec![
                        values[ptr - 1].parse::<f64>().unwrap(),
                        values[ptr + 1].parse::<f64>().unwrap(),
                    ]),
                    _ => 0.0,
                };
                // multiplication(vec![values[ptr - 1].parse::<f64>().unwrap(), values[ptr + 1].parse::<f64>().unwrap()]);
                // println!("{}", new_val);
                values.remove(ptr);
                values.remove(ptr);
                // values.remove(ptr + 1);
                values.remove(ptr - 1);
                // println!("After Removal: {:?} {}", values.clone(), ptr);
                values.insert(ptr - 1, new_val.to_string());
                // println!("After insertion: {:?} {}", values.clone(), ptr);
                ptr = 0;
                if values.len() == 1 {
                    break 'operation;
                }
            }
            ptr += 1;
        }
        ptr = 0;
    }
    // println!("{}", values[0]);
    return values[0].parse::<f64>().unwrap();
}
fn handle_bodmas(mut values: Vec<String>) -> Vec<String> {
    let mut start: usize = 0;
    let mut end: usize = 0;
    if values.len() == 1 {
        values.insert(
            0,
            values[0]
                .trim_start_matches("(")
                .trim_end_matches(")")
                .to_string(),
        );
        values.remove(1);
        // println!("{}", values[0]);
        return values;
    }
    while start < values.len() {
        if values[start].contains("(") {
            'find_end: loop {
                if end == values.len() {
                    return vec!["".to_string()];
                } else if values[end].contains("(") {
                    start = end;
                } else if values[end].contains(")") {
                    break 'find_end;
                }
                end += 1;
            }
            let (open_brackets, res_vec, close_brackets) =
                handle_brakets(values[start..=end].to_vec());
            let res = handle_values(res_vec);
            for _x in start..=end {
                values.remove(start);
            }
            // println!("After removal: {:?}", values);
            values.insert(
                start,
                open_brackets + res.to_string().trim() + close_brackets.trim(),
            );
            // start -= 1;
            // println!("{:?}", values);
            end = start;
        }
        end += 1;
        start += 1;
    }
    // println!("in handle bodmas: {:?}", values);
    values
}
fn handle_brakets(mut values: Vec<String>) -> (String, Vec<String>, String) {
    let mut open_brackets = "".to_string();
    let mut close_brackets = "".to_string();
    for ch in values[0].chars() {
        if ch == '(' {
            open_brackets.insert(0, ch);
        }
    }
    for ch in values[values.len() - 1].chars() {
        if ch == ')' {
            close_brackets.insert(0, ch);
        }
    }
    values.insert(0, values[0].trim_start_matches("(").to_string());
    values.insert(
        values.len() - 1,
        values[values.len() - 1].trim_end_matches(")").to_string(),
    );
    // println!("After{:?}", values);
    values.remove(1);
    if values.len() > 1 {
        values.remove(values.len() - 1);
    }

    (
        open_brackets[1..open_brackets.len()].to_string(),
        values,
        close_brackets[1..close_brackets.len()].to_string(),
    )
}
