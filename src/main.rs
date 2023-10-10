use std::io;

fn main() {
    let mut game_state: [u8; 5] = [0b00000101, 0b00000100, 0b00000011, 0b00000010, 0b00000001];
    let mut error_message = "";
    let mut player_turn = 1;
    'game_loop: loop {
        // clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        // update game
        println!("Enter a column number, then enter the number of counters to take.");
        println!("Player {}'s turn", player_turn);
        display(&game_state);
        println!("{}", error_message);

        if total(&game_state) == 0 {
            break;
        }

        error_message = "";

        let mut user_input_column = String::new();
        let mut user_input_counters = String::new();

        // get user column input
        io::stdin()
            .read_line(&mut user_input_column)
            .expect("Failed to read line.");

        let user_column: usize = match user_input_column.trim().parse() {
            Ok(num) => {
                if 1 <= num && num <= 5 {
                    if game_state[5 - num] > 0 {
                        num
                    } else {
                        error_message = "Please enter a non-empty column";
                        continue 'game_loop;
                    }
                } else {
                    error_message = "Column number must be between 1 and 5";
                    continue 'game_loop;
                }
            }
            Err(_) => {
                error_message = "Please enter a valid number";
                continue 'game_loop;
            }
        };

        // get user counter input
        io::stdin()
            .read_line(&mut user_input_counters)
            .expect("Failed to read line.");

        let user_counters: u8 = match user_input_counters.trim().parse() {
            Ok(num) => {
                if num > 0 && num <= game_state[5 - user_column] {
                    player_turn = 3 - player_turn;
                    num
                } else if num <= 0 {
                    error_message = "Enter a positive number";
                    continue 'game_loop;
                } else {
                    error_message = "Enter a number less than the number of counters in the column";
                    continue 'game_loop;
                }
            }
            Err(_) => {
                error_message = "Please enter a valid number";
                continue 'game_loop;
            }
        };
        game_state = decrement_index(&mut game_state, 5 - user_column, user_counters);
    }
    print!("\x1B[2J\x1B[1;1H");
    println!("\n###############");
    println!("#             #");
    println!("#   WINNER:   #");
    println!("#  PLAYER {}!  #", player_turn);
    println!("#             #");
    println!("###############\n\n");
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

fn decrement_index(input_array: &mut [u8; 5], index: usize, decrement_amount: u8) -> [u8; 5] {
    input_array[index] -= decrement_amount;
    *input_array
}
