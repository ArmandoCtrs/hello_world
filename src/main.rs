use core::num;

//LinkedIn Learning practice from Lab Assignment 3 Task 7
fn main() {
    println!("Hello, World!");

    //chapter 2
    data_type_practice();
    math_practice();
    bitwise_practice();
    boolean_practice();
    comparison_practice();
    char_practice();
    average();

    //chapter 3
    array_practice();
    two_d_array_practice();
    tuple_practice();

    //chapter 4
    say_hello();
    say_a_number(13);

    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
    let result = square(13);
    println!("result is {:?}", result);

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");

    //chapter 5
    conditional_practice();
    multiple_conditions();
    conditional_assignment();
    loop_practice();
    while_loop_practice();
    for_loop_practice();
    nested_loops_practice();

    max_min_mean();
}

//chapter 2
fn data_type_practice() {
    let mut x: i32 = 255;
    println!("x is {}", x);

    x = x + 1;
    println!("x is {}", x);

    x = 20;
    println!("x is {}", x);

    let y: f64 = 10.1561515;
    println!("y is {}", y);
}

fn math_practice() {
    let a = 10;
    let b = 3.0;
    let c = a as f64 / (b + 1.0);

    print!("c is {0:08.3}\na is {1}\nc is {0}\n", c, a);
}

fn bitwise_practice() {
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value;
    println!("value is {:08b}", value);

    value = value & 0b1111_0111;
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("value is {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value);

    value = value << 4;
    println!("value is {:08b}", value);

    value = value >> 2;
    println!("value is {:08b}", value);
}

fn boolean_practice() {
    let a = true;
    let b = false;

    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c =  (a ^ b) || panic!();
    println!("c is {}", c);
}

fn comparison_practice() {
    let a = true;
    let b = false;

    println!("a is {} and b is {}", a, b);
    println!("a EQUAL to b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO {}", a <= b);
}

fn char_practice() {
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';

    println!("{}\n{}\n{}", letter, number, finger);
}

fn average() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}


//chapter 3
fn array_practice() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];

    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index = numbers.len();

    println!("last number is {}", numbers[index - 1]);
}

fn two_d_array_practice() {
    let parking_lot = [[1, 2, 3],
                                [4, 5, 6]];

    let number = parking_lot[1][2];
    println!("number is {}", number);

    let garage = [[[0; 100]; 20]; 5];
    println!("garage at end is {}", garage[4][19][99]);
}

fn tuple_practice() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;

    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);
}


//chapter 4
fn say_hello() {
    println!("Hello!");
}

fn say_a_number(number: i32) {
    println!("{}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("{}", sum);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    println!("End of function");
}

fn celsius_to_fahrenheit (celsius_temp: f64) -> f64{
    celsius_temp * 1.8 + 32.0
}


//chapter 5
fn conditional_practice() {
    let mut x = 3;

    if x == 3 {
        println!("x is 3!");
    }

    if x == 4 {
        println!("x is 4!");
    }

    x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }
}

fn multiple_conditions() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else if x < y {
            println!("x is less than y");
    } else {
            println!("x is equal to y");
    }
}

fn conditional_assignment() {
    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};
    
    /*
    if make_x_odd {
        x = 1;
    } else {
        x = 2;
    }
    */
    
    println!("x is {}", x);    
}

fn loop_practice() {
    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
}

fn while_loop_practice() {
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}

fn for_loop_practice() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);

        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }
}

fn nested_loops_practice() {
    let mut matrix = [[1, 2, 3],
                                [4, 5, 6],
                                [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }

        println!();
    }
}

fn max_min_mean() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    //max
    max = numbers[0];
    for num in numbers {
        if num > max {
            max = num;
        }
    }

    //min
    min = numbers[0];
    for num in numbers {
        if num < min {
            min = num;
        }
    }

    //mean
    mean = 0.0;
    for num in numbers {
        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}