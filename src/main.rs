#![allow(warnings)]
use std::process::Child;
use rand::{RngExt, random, random_range};

fn main() {
    // Defined
    let TARGET: String = "The quick brown fox jumps over the lazy dog".to_string();
    let TARGET_BYTES = TARGET.as_bytes();
    let TARGET_LENGTH = TARGET_BYTES.len();
    let CHARSET: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ".to_string();
    let CHARSET_BYTES = CHARSET.as_bytes();
    let CHARSET_LENGTH: usize = CHARSET_BYTES.len();
    let POPULATION_SIZE: u64 = 10000;
    let THRESHOLD: usize = 3;
    let mut GENERATION: usize = 0;

    // Containers
    let mut POPULATION: Vec<Vec<u8>> = Vec::new();
    let mut NEW_POPULATION: Vec<Vec<u8>> = Vec::new();
    let mut SELECTED: Vec<Vec<u8>> = Vec::new();
    let mut INDIVIDUAL: Vec<u8> = Vec::new();

    // Loop that handles the First Generation
    for _ in 0..POPULATION_SIZE {
            while INDIVIDUAL.len() < TARGET_LENGTH {
                let CHAR = fill_string(CHARSET_BYTES, CHARSET_LENGTH);
                INDIVIDUAL.push(CHAR);
            }
            if INDIVIDUAL.len() == TARGET_LENGTH {
                POPULATION.push(INDIVIDUAL.clone());
                INDIVIDUAL.clear();
            }
        }

    // Main loop (Handles the cycle of Calculating Fitness -> Mutating until we get satisfactory results)
    loop {
        println!("Generation {}", GENERATION);
        GENERATION += 1;
        SELECTED.clear();

        for INDIVIDUAL in &POPULATION {
            let FITNESS = calculate_fitness(INDIVIDUAL, TARGET_BYTES, TARGET_LENGTH);

            if FITNESS >= THRESHOLD {
                SELECTED.push(INDIVIDUAL.clone());
            }

            if FITNESS == TARGET_LENGTH {
                println!("Found {} At Generation {}", String::from_utf8_lossy(INDIVIDUAL), GENERATION);
                return
            }
        }

        for _ in 0..POPULATION_SIZE {
            let mut CHILD = recombination(TARGET_LENGTH, TARGET_BYTES, &SELECTED, CHARSET_BYTES, CHARSET_LENGTH);
            mutate(&mut CHILD, CHARSET_BYTES, CHARSET_LENGTH, TARGET_BYTES);
            NEW_POPULATION.push(CHILD);
        }

        POPULATION.clear();
        POPULATION.append(&mut NEW_POPULATION);
    }    
}

fn fill_string(CHARSET_BYTES: &[u8], CHARSET_LENGTH: usize) -> u8 {
    let mut rng = rand::rng();
    let random_number = rng.random_range(0..CHARSET_LENGTH);
    return CHARSET_BYTES[random_number];
}

fn calculate_fitness(INDIVIDUAL: &[u8], TARGET_BYTES: &[u8], TARGET_LENGTH: usize) -> usize {
    let mut FITNESS: usize = 0;

    for position in 0..TARGET_LENGTH {
        if INDIVIDUAL[position] == TARGET_BYTES[position] {
            FITNESS += 1;
        }
    }

    return FITNESS;
}

fn random_selected(SELECTED: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut rng = rand::rng();
    let random_number = rng.random_range(0..SELECTED.len());
    return SELECTED[random_number].clone();
}

fn mutate(PARENT: &mut Vec<u8>, CHARSET_BYTES: &[u8], CHARSET_LENGTH: usize, TARGET_BYTES: &[u8]) {
    let mut rng = rand::rng();
    let PARENT_LENGTH = PARENT.len();
    let mut MUTATION_COUNT = 0;

    while MUTATION_COUNT < 35 {
        let MUTATION_CHANCE = rng.random_range(0..100);

        if MUTATION_CHANCE > 10 {
            let mut random_number = rng.random_range(0..PARENT_LENGTH);
            let mut CURRENT_POS_CHAR = PARENT[random_number];
            let mut TARGET_POS_CHAR = TARGET_BYTES[random_number];

            if calculate_fitness(PARENT, TARGET_BYTES, PARENT_LENGTH) == PARENT_LENGTH {
                break;
            }

            while CURRENT_POS_CHAR == TARGET_POS_CHAR {
                random_number = rng.random_range(0..PARENT_LENGTH);
                CURRENT_POS_CHAR = PARENT[random_number];
                TARGET_POS_CHAR = TARGET_BYTES[random_number];
            }

            let mut CHAR = fill_string(CHARSET_BYTES, CHARSET_LENGTH);

            while CHAR == CURRENT_POS_CHAR {
                CHAR = fill_string(CHARSET_BYTES, CHARSET_LENGTH);
            }

            PARENT[random_number] = CHAR;
            MUTATION_COUNT += 1;

        } else {
            break;
        }
    }
}

fn recombination(TARGET_LENGTH: usize, TARGET_BYTES: &[u8], SELECTED: &Vec<Vec<u8>>, CHARSET_BYTES: &[u8], CHARSET_LENGTH: usize) -> Vec<u8> {
    let PARENT_A = random_selected(&SELECTED);
    let PARENT_B = random_selected(&SELECTED);
    let mut CHILD: Vec<u8> = Vec::new();

    for position in 0..TARGET_LENGTH {
        if PARENT_A[position] == TARGET_BYTES[position] {
            CHILD.push(PARENT_A[position]);
        } else if PARENT_B[position] == TARGET_BYTES[position] {
            CHILD.push(PARENT_B[position]);
        } else {
            let random_char = fill_string(CHARSET_BYTES, CHARSET_LENGTH);
            CHILD.push(random_char);
        }
    }

    CHILD
}