use std::io;
fn main() {
    /*
     * addition
     * substraction
     * multiplication
     * division
     * cpga calculator
     * pow
     * free calculator
     */
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
                "Enter the values you want to operate on and seperate the value using '{}' like so: 1{}2{}3{}4", operations[opt-1], operations[opt-1], operations[opt-1], operations[opt-1]
            );
            let mut space_sep_values = String::new();
            let _ = io::stdin()
                .read_line(&mut space_sep_values)
                .expect("Na rubbish you dey do");
            let val = handle_space_sep_values(space_sep_values);
            let res = handle_values(val);
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
        } 
        // else if opt == 2 {
        //     println!(
        //         "Enter the values you want to find the differences and seperate the value using ' - ' like so: 1 - 2 - 3 - 4"
        //     );
        //     let mut space_sep_values = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut space_sep_values)
        //         .expect("Na rubbish you dey do");
        //     let val = handle_operation_sep_values(space_sep_values, " - ");
        //     println!("The result is {}\n \n", subtraction(val));
        //     println!("Do you want to continue? (Y, N)");
        //     let mut choice = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut choice)
        //         .expect("Na rubbish you dey do");
        //     if choice.trim() == "N" {
        //         println!("\nThank you for using this CLI calculator\n");
        //         break;
        //     }
        // } else if opt == 3 {
        //     println!(
        //         "Enter the values you want to multiply and seperate the value using ' * ' like so: 1 * 2 * 3 * 4"
        //     );
        //     let mut space_sep_values = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut space_sep_values)
        //         .expect("Na rubbish you dey do");
        //     let val = handle_operation_sep_values(space_sep_values, " * ");
        //     println!("The result is {}\n \n", multiplication(val));
        //     println!("Do you want to continue? (Y, N)");
        //     let mut choice = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut choice)
        //         .expect("Na rubbish you dey do");
        //     if choice.trim() == "N" {
        //         println!("\nThank you for using this CLI calculator\n");
        //         break;
        //     }
        // } else if opt == 4 {
        //     println!(
        //         "Enter the values you want to divide and seperate the value using ' / ' like so: 1 / 2 / 3 / 4"
        //     );
        //     let mut space_sep_values = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut space_sep_values)
        //         .expect("Na rubbish you dey do");
        //     let val = handle_operation_sep_values(space_sep_values, " / ");
        //     println!("The result is {}\n \n", division(val));
        //     println!("Do you want to continue? (Y, N)");
        //     let mut choice = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut choice)
        //         .expect("Na rubbish you dey do");
        //     if choice.trim() == "N" {
        //         println!("\nThank you for using this CLI calculator\n");
        //         break;
        //     }
        // }
        else if opt == 7 {
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
        } 
        // else if opt == 5 {
        //     println!(
        //         "Enter the values you want to raise t and seperate the value using ' ** ' like so: 1 ** 2"
        //     );
        //     let mut space_sep_values = String::new();
        //     let _ = io::stdin()
        //         .read_line(&mut space_sep_values)
        //         .expect("Na rubbish you dey do");
        //     let val = handle_operation_sep_values(space_sep_values, " ** ");
        //     println!("The result is {}\n \n", pow(val));
        // }
         else if opt == 6 {
            println!(
                "Enter the values you want to divide and seperate the value using ' ' and the like so: 1 + 2 * 3 / 4"
            );
            let mut space_sep_values = String::new();
            let _ = io::stdin()
                .read_line(&mut space_sep_values)
                .expect("Na rubbish you dey do");
            let val = handle_space_sep_values(space_sep_values);
            handle_values(val);
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
// fn _handle_operation_sep_values(space_sep_values: String, operation: &str) -> Vec<f64> {
//     let values: Vec<f64> = space_sep_values.split(operation).map(|f| f.trim().parse::<f64>().expect("You entered an invalid input, please make sure that the values are integers and are seperated by the given operation")).collect();
//     return values;
// }

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

fn handle_values(mut values: Vec<String>) -> f64{
    let mut ptr: usize = 0;
    let operations = [ "**","*", "/", "+", "-",];
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
    // 'multiply: loop {
    //     if values.len() == 1 {
    //         break;
    //     } else if ptr == values.len() {
    //         break 'multiply;
    //     } else if values[ptr] == "*" {
    //         let new_val = multiplication(vec![values[ptr - 1].parse::<f64>().unwrap(), values[ptr + 1].parse::<f64>().unwrap()]);
    //         println!("{}", new_val);
    //         values.remove(ptr);
    //         values.remove(ptr);
    //         // values.remove(ptr + 1);
    //         values.remove(ptr - 1);
    //         println!("After Removal: {:?} {}", values.clone(), ptr);
    //         values.insert(ptr-1, new_val.to_string());
    //         println!("After insertion: {:?} {}", values.clone(), ptr);
    //         ptr = 0;
    //         if values.len() == 1 {
    //             break 'multiply;
    //         }
    //     }
    //     ptr += 1;
    // }
    // ptr = 0;
    // 'divide: loop {
    //     if values.len() == 1 {
    //         break;
    //     } else if ptr == values.len() {
    //         break 'divide;
    //     } else if values[ptr] == "/" {
    //         let new_val = division(vec![values[ptr - 1].parse::<f64>().unwrap(), values[ptr + 1].parse::<f64>().unwrap()]);
    //         println!("{}", new_val);
    //         values.remove(ptr);
    //         values.remove(ptr);
    //         // values.remove(ptr + 1);
    //         values.remove(ptr - 1);
    //         println!("After Removal: {:?} {}", values.clone(), ptr);
    //         values.insert(ptr-1, new_val.to_string());
    //         println!("After insertion: {:?} {}", values.clone(), ptr);
    //         ptr = 0;
    //         if values.len() == 1 {
    //             break 'divide;
    //         }
    //     }
    //     ptr += 1;
    // }
    // ptr = 0;
    // 'add: loop {
    //     if values.len() == 1 {
    //         break;
    //     } else if ptr == values.len() {
    //         break 'add;
    //     } else if values[ptr] == "+" {
    //         let new_val = addition(vec![values[ptr - 1].parse::<f64>().unwrap(), values[ptr + 1].parse::<f64>().unwrap()]);
    //         println!("{}", new_val);
    //         values.remove(ptr);
    //         values.remove(ptr);
    //         // values.remove(ptr + 1);
    //         values.remove(ptr - 1);
    //         println!("After Removal: {:?} {}", values.clone(), ptr);
    //         values.insert(ptr-1, new_val.to_string());
    //         println!("After insertion: {:?} {}", values.clone(), ptr);
    //         ptr = 0;
    //         if values.len() == 1 {
    //             break 'add;
    //         }
    //     }
    //     ptr += 1;
    // }
    // ptr = 0;
    // 'substract: loop {
    //     if values.len() == 1 {
    //         break;
    //     } else if ptr == values.len() {
    //         break 'substract;
    //     } else if values[ptr] == "-" {
    //         let new_val = subtraction(vec![values[ptr - 1].parse::<f64>().unwrap(), values[ptr + 1].parse::<f64>().unwrap()]);
    //         println!("{}", new_val);
    //         values.remove(ptr);
    //         values.remove(ptr);
    //         // values.remove(ptr + 1);
    //         values.remove(ptr - 1);
    //         println!("After Removal: {:?} {}", values.clone(), ptr);
    //         values.insert(ptr-1, new_val.to_string());
    //         println!("After insertion: {:?} {}", values.clone(), ptr);
    //         ptr = 0;
    //         if values.len() == 1 {
    //             break 'substract;
    //         }
    //     }
    //     ptr += 1;
    // }
    return values[0].parse::<f64>().unwrap();
}
