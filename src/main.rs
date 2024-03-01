use std::fs;
use std::io::{self, Read, Write};

fn output_byte(byte: u8) {
    print!("{}", byte as char);
    io::stdout().flush().unwrap();
}

fn read_byte_from_user() -> u8 {
    let mut buf = [0; 1];
    io::stdin().read_exact(&mut buf).unwrap();

    buf[0]
}

fn main() {
    let file = std::env::args().nth(1).expect("Provide file to run");
    let data = fs::read_to_string(file).unwrap();
    let code = data.as_bytes();

    let mut mem = [0u8; 30000];
    let mut instruction_ptr = 0;
    let mut data_ptr = 0;
    let mut jump_list = Vec::new();

    while instruction_ptr < code.len() {
        let byte = code[instruction_ptr];
        match byte {
            b'>' => data_ptr += 1,
            b'<' => data_ptr -= 1,
            b'+' => mem[data_ptr] = mem[data_ptr].wrapping_add(1),
            b'-' => mem[data_ptr] = mem[data_ptr].wrapping_sub(1),
            b'.' => output_byte(mem[data_ptr]),
            b',' => mem[data_ptr] = read_byte_from_user(),
            b'[' => {
                if mem[data_ptr] == 0 {
                    let mut jump_balance = 1;
                    while jump_balance != 0 {
                        instruction_ptr += 1;
                        let byte = code[instruction_ptr];
                        match byte {
                            b'[' => jump_balance += 1,
                            b']' => jump_balance -= 1,
                            _ => (),
                        }
                    }
                } else {
                    jump_list.push(instruction_ptr);
                }
            },
            b']' => {
                if mem[data_ptr] != 0 {
                    instruction_ptr = *jump_list.last().unwrap();
                } else {
                    jump_list.pop();
                }
            },
            _ => (),
        }

        instruction_ptr += 1;
    }
}
