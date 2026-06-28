/// Размер сетки (квадрат GRID x GRID)
pub const GRID: usize = 5;

/// Количество входных нейронов (пикселей)
pub const INPUTS: usize = GRID * GRID;

/// Количество нейронов скрытого слоя
pub const HIDDEN: usize = 8;

/// Количество выходных нейронов (классов)
// TODO: Может это должно быть динамечски из размера LABELS?
pub const OUTPUTS: usize = 4;

/// Названия классов (для вывода)
pub const LABELS: [&str; OUTPUTS] = ["square", "rectangle", "circle", "triangle"];
