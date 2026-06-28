mod config;
mod data;
mod network;
mod utils;

use crate::config::{GRID, LABELS};
use crate::data::{grid_to_input, training_data};
use crate::network::Network;
use crate::utils::{print_examples, print_grid};

use std::io::{self, Write};

fn main() {
    println!("Lab 3: shape recognition (neural network)");
    println!("Grid size: {GRID}x{GRID}\n");

    print_examples();

    let samples = training_data();

    let mut net = Network::random();

    // Обучаем
    println!("Training...");
    net.train(&samples, 10_000, 0.5);

    // Проверяем на обучающей выборке
    println!("\nTraining set check:");
    for (i, sample) in samples.iter().enumerate() {
        let predicted = net.predict(&sample.input);
        let expected = crate::utils::argmax(&sample.target); // можно и так
        println!(
            "sample {}: expected {}, got {} ({})",
            i + 1,
            LABELS[expected],
            LABELS[predicted],
            if predicted == expected { "ok" } else { "fail" }
        );
    }

    println!("\nEnter shapes to recognize. Type 'q' on first line to quit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut first = String::new();
        if io::stdin().read_line(&mut first).is_err() {
            break;
        }
        if first.trim().eq_ignore_ascii_case("q") {
            break;
        }

        let mut grid = [[0u8; GRID]; GRID];
        let first_row: Vec<u8> = first
            .trim()
            .chars()
            .filter_map(|ch| match ch {
                '0' => Some(0),
                '1' => Some(1),
                _ => None,
            })
            .collect();

        if first_row.len() != GRID {
            println!("need {GRID} digits, try again\n");
            continue;
        }
        grid[0] = first_row.try_into().unwrap();

        let mut valid = true;
        for row in 1..GRID {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let digits: Vec<u8> = line
                .trim()
                .chars()
                .filter_map(|ch| match ch {
                    '0' => Some(0),
                    '1' => Some(1),
                    _ => None,
                })
                .collect();
            if digits.len() != GRID {
                println!("need {GRID} digits, cancelled\n");
                valid = false;
                break;
            }
            grid[row] = digits.try_into().unwrap();
        }
        if !valid {
            continue;
        }

        println!("\nYour figure:");
        print_grid(&grid);

        let input = grid_to_input(&grid);
        let (_, output) = net.forward(&input);
        let class = net.predict(&input);

        println!("Result: {}", LABELS[class]);
        println!(
            "Scores: {}",
            output
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{}={:.3}", LABELS[i], v))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();
    }
}
