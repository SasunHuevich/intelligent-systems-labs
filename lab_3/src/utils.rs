use crate::config::{GRID, OUTPUTS};

/// Сигмоида (логистическая функция)
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Производная сигмоиды (принимает значение y = sigmoid(x))
pub fn sigmoid_deriv(y: f64) -> f64 {
    y * (1.0 - y)
}

/// Среднеквадратичная ошибка (MSE) для векторов длины OUTPUTS
pub fn mse(output: &[f64; OUTPUTS], target: &[f64; OUTPUTS]) -> f64 {
    output
        .iter()
        .zip(target.iter())
        .map(|(o, t)| (o - t).powi(2))
        .sum::<f64>()
        / OUTPUTS as f64
}

/// Индекс максимального элемента в срезе
pub fn argmax(values: &[f64]) -> usize {
    values
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Печатает двумерный массив u8 как сетку из 0 и 1
pub fn print_grid(grid: &[[u8; GRID]; GRID]) {
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

/// Печатает примеры фигур
pub fn print_examples() {
    println!("Classes:");
    println!("  square    — equal width and height");
    println!("  rectangle — width != height");
    println!("  circle    — round shape");
    println!("  triangle  — triangle pointing up");
    println!();
    println!("Example square:");
    print_grid(&[
        [0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0],
    ]);
    println!("Example rectangle:");
    print_grid(&[
        [0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]);
    println!("Example circle:");
    print_grid(&[
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ]);
    println!("Example triangle:");
    print_grid(&[
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]);
}
