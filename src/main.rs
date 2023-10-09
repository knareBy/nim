use std::io;
use rand::Rng;

fn main() {
    let mut game_state: [u8; 5] = [0b00000101, 0b00000100, 0b00000011, 0b00000010, 0b00000001];
    let mut error_message = "";
    'game_loop: loop {
        print!("\x1B[2J\x1B[1;1H");
        display(&game_state);
        println!("{}", error_message);
        error_message = "";
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        let user_column: usize = match user_input.trim().parse() {
            Ok(num) => {
                if 1 <= num && num <=5 {
                    num
                } else {
                    error_message = "Number must be between 1 and 5";
                    continue 'game_loop;
                }
            },
            Err(_) => {
                error_message = "Please enter a valid number";
                continue 'game_loop;
            }
        };
        if game_state[5 - user_column] > 0 {
            game_state = decrement_index(&mut game_state, 5 - user_column);
        } else {
            error_message = "Please enter a non-empty column";
        }
        
        if total(&game_state) == 0 {
            break;
        }
    }
}

fn display(state: &[u8; 5]) {
    let total_rows_columns = 5;
    let mut row_count = 0;
    loop {
        if row_count < total_rows_columns {
            let mut column_count = 0;
            let mut row = String::new();
            loop {
                if column_count < total_rows_columns {
                    if usize::from(state[column_count]) >= (total_rows_columns - row_count) {
                        row.push_str(" ⬤ ");
                    } else {
                        row.push_str("   ");
                    }
                } else {
                    break;
                }
                column_count += 1;
            }
            println!("{}", row);
        } else {
            break;
        }
        row_count += 1;
    }
    println!(" 5  4  3  2  1 ");
}

fn total(input_array: &[u8; 5]) -> u8 {
    let mut counter: usize = 0;
    let mut total: u8 = 0;
    loop {
        if counter < 5 {
            total += input_array[counter];
        } else {
            return total;
        }
        counter += 1;
    }
}

fn decrement_random(input_array: &mut [u8; 5]) -> [u8; 5] {
    let random_column = rand::thread_rng().gen_range(0..5);
    input_array[random_column] -= 1;
    *input_array
}

fn decrement_index(input_array: &mut [u8; 5], index: usize) -> [u8; 5] {
    input_array[index] -= 1;
    *input_array
}