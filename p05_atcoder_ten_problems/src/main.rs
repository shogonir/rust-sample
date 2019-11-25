//
// [Problem 1]
//

fn execute_problem_1() {
    println!("[Problem 1]");

    check_product_is_even(1, 1);
    check_product_is_even(2, 1);
    check_product_is_even(1, 2);
    check_product_is_even(2, 2);
    check_product_is_even(0, 3);
    check_product_is_even(3, 0);
    check_product_is_even(-1, 2);
    check_product_is_even(-1, 1);
    check_product_is_even(-1, -3);

    println!("");
}

fn check_product_is_even(a: i32, b: i32) {
    println!("{} x {} = {}, isEven: {}", a, b, a * b, is_product_even(a, b))
}

fn is_product_even(a: i32, b: i32) -> bool {
    if a % 2 == 0 {
        true
    } else if b % 2 == 0 {
        true
    } else {
        false
    }
}



//
// [Problem 2]
//

fn execute_problem_2() {
    println!("[Problem 2]");
    check_count_letter_1("12:10");
    check_count_letter_1("My name is shogonir.");
    check_count_letter_1("111111");
    println!("");
}

fn check_count_letter_1(string: &str) {
    println!("'{}' -> contains {} x '1'", string, count_pattern_in_string(string, "1"));
}

fn count_pattern_in_string(string: &str, pattern: &str) -> usize {
    string.match_indices(pattern).count()
}



//
// [Problem 3]
//

fn execute_problem_3() {
    println!("[Problem 3]");
    let numbers: Vec<i32> = vec![12, 12, 12, 4];
    let min: usize = minimum_divisible_2(&numbers);
    print_numbers(&numbers);
    println!(" -> {}", min);
    println!("");
}

fn print_numbers(numbers: &Vec<i32>) {
    print!("[");
    for index in 0..numbers.len() {
        if index != 0 {
            print!(", ");
        }
        print!("{}", numbers[index]);
    }
    print!("]");
}

fn minimum_divisible_2(numbers: &Vec<i32>) -> usize {
    let mut min: usize = count_divisor(numbers[0], 2);
    for number in numbers {
        let count_divisor_2 = count_divisor(*number, 2);
        if count_divisor_2 < min {
            min = count_divisor_2;
        }
    }
    min
}

fn count_divisor(mut number: i32, divisor: i32) -> usize {
    let mut count = 0;
    loop {
        if number % divisor == 0 {
            count += 1;
            number /= divisor;
        } else {
            break;
        }
    }
    count
}



//
// [Problem 4]
//

fn execute_problem_4() {
    println!("[Problem 4]");
    check_sum_coins(7, 30, 50, 4950);
    check_sum_coins(10, 50, 100, 4950);
    println!("");
}

fn check_sum_coins(max_500: u32, max_100: u32, max_50: u32, target_sum: u32) {
    let count = count_sum_coins(max_500, max_100, max_50, target_sum);
    println!("500: {}, 100: {}, 50: {}, Total: {} -> Count: {}", max_500, max_100, max_50, target_sum, count);
}

fn count_sum_coins(max_500: u32, max_100: u32, max_50: u32, target_sum: u32) -> usize {
    let mut count = 0;
    for num_500 in 0..(target_sum / 500 + 1) {
        if num_500 > max_500 { break; }
        let target_left = target_sum - (num_500 * 500);
        for num_100 in 0..(target_left / 100 + 1) {
            if num_100 > max_100 { break; }
            let target_left = target_left - (num_100 * 100);
            if target_left % 50 == 0 {
                let num_50 = target_left / 50;
                if num_50 > max_50 { break; }
                count += 1;
            }
        }
    }
    count
}



//
// [Problem 6]
//

fn execute_problem_6() {
    println!("[Problem 6]");
    check_diff_sum_odd_even_element(vec![4, 2, 1, 5, 3]);
    check_diff_sum_odd_even_element(vec![2, 3, 4, 5, 6, 7]);
    println!("");
}

fn check_diff_sum_odd_even_element(vector: Vec<i32>) {
    print_numbers(&vector);
    print!(" -> ");
    let diff = diff_sum_odd_even_element(vector);
    println!(" -> {}", diff);
}

fn diff_sum_odd_even_element(mut vector: Vec<i32>) -> i32 {
    &vector.sort();
    print_numbers(&vector);
    let mut sum_odd: i32 = 0;
    let mut sum_even: i32 = 0;
    for (index, value) in (0..).zip(vector) {
        if index % 2 == 0 {
            sum_odd += value;
        } else {
            sum_even += value;
        }
    }
    (sum_odd - sum_even).abs()
}

//
// main
//

fn main() {
    execute_problem_1();
    execute_problem_2();
    execute_problem_3();
    execute_problem_4();
    execute_problem_6();
}
