extern crate helpers;

use std::fs;

fn process_intcode(opcodes: &Vec<usize>, noun: usize, verb: usize, target: usize) -> bool {
    let mut buffer = opcodes.clone();
    buffer[1] = noun;
    buffer[2] = verb;
    for chunk in opcodes.chunks(4) {
        match chunk[0] {
            1 => buffer[chunk[3]] = buffer[chunk[1]] + buffer[chunk[2]],
            2 => buffer[chunk[3]] = buffer[chunk[1]] * buffer[chunk[2]],
            99 => break,
            _ => panic!()
        }
    }
    buffer[0] == target
}

fn main() {
    let data = fs::read_to_string(helpers::args::path_from_args().unwrap()).unwrap();
    let opcodes: Vec<usize> = data.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

    for noun in 0..99 {
        for verb in 0..99 {
            if process_intcode(&opcodes, noun, verb, 19690720) {
                println!("result: {}", 100 * noun + verb)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_process_intcode() {
        assert!(process_intcode(&vec!{1,0,0,0,99}, 0,0,2));
        assert!(process_intcode(&vec!{2,3,0,3,99}, 3,0,2));
        assert!(process_intcode(&vec!{2,0,0,5,99,0}, 4,4,2));
    }
}