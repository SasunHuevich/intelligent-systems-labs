use crate::config::{HIDDEN, INPUTS, OUTPUTS};
use crate::data::Sample;
use crate::utils::{argmax, mse, sigmoid, sigmoid_deriv};

/// Структура сети с весами и смещениями
pub struct Network {
    // Веса от входов к скрытому слою: [входной_нейрон][скрытый_нейрон]
    w_ih: [[f64; HIDDEN]; INPUTS],
    // Смещения скрытого слоя
    b_h: [f64; HIDDEN],
    // Веса от скрытого к выходному слою: [скрытый_нейрон][выходной_нейрон]
    w_ho: [[f64; OUTPUTS]; HIDDEN],
    // Смещения выходного слоя
    b_o: [f64; OUTPUTS],
}

impl Network {
    /// Создаёт сеть со случайными весами в диапазоне [-0.2, 0.2] для весов
    /// и [-0.1, 0.1] для смещений.
    pub fn random() -> Self {
        let mut seed = 42u64;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            (seed >> 33) as f64 / u32::MAX as f64 * 2.0 - 1.0
        };

        Self {
            w_ih: std::array::from_fn(|_| std::array::from_fn(|_| next() * 0.2)),
            b_h: std::array::from_fn(|_| next() * 0.1),
            w_ho: std::array::from_fn(|_| std::array::from_fn(|_| next() * 0.2)),
            b_o: std::array::from_fn(|_| next() * 0.1),
        }
    }

    /// Прямой проход: возвращает (активации скрытого слоя, выходные активации)
    pub fn forward(&self, input: &[f64; INPUTS]) -> ([f64; HIDDEN], [f64; OUTPUTS]) {
        // Вычисляем скрытый слой
        let mut hidden = [0.0; HIDDEN];
        for j in 0..HIDDEN {
            let mut sum = self.b_h[j];
            for i in 0..INPUTS {
                sum += input[i] * self.w_ih[i][j];
            }
            hidden[j] = sigmoid(sum);
        }

        // Вычисляем выходной слой
        let mut output = [0.0; OUTPUTS];
        for k in 0..OUTPUTS {
            let mut sum = self.b_o[k];
            for j in 0..HIDDEN {
                sum += hidden[j] * self.w_ho[j][k];
            }
            output[k] = sigmoid(sum);
        }

        (hidden, output)
    }

    /// Предсказывает класс (индекс) для данного входа
    pub fn predict(&self, input: &[f64; INPUTS]) -> usize {
        let (_, output) = self.forward(input);
        argmax(&output)
    }

    /// Обучение сети методом градиентного спуска (backpropagation)
    /// - `samples` – обучающая выборка
    /// - `epochs` – количество эпох
    /// - `lr` – скорость обучения (learning rate)
    pub fn train(&mut self, samples: &[Sample], epochs: usize, lr: f64) {
        for epoch in 0..epochs {
            let mut total_error = 0.0;

            for sample in samples {
                // Прямой проход
                let (hidden, output) = self.forward(&sample.input);
                total_error += mse(&output, &sample.target);

                // ----- Обратное распространение -----

                // Вычисляем дельты для выходного слоя
                let mut out_delta = [0.0; OUTPUTS];
                for k in 0..OUTPUTS {
                    out_delta[k] = (output[k] - sample.target[k]) * sigmoid_deriv(output[k]);
                }

                // Вычисляем дельты для скрытого слоя
                let mut hidden_delta = [0.0; HIDDEN];
                for j in 0..HIDDEN {
                    let mut err = 0.0;
                    for k in 0..OUTPUTS {
                        err += out_delta[k] * self.w_ho[j][k];
                    }
                    hidden_delta[j] = err * sigmoid_deriv(hidden[j]);
                }

                // Обновляем веса и смещения выходного слоя
                for j in 0..HIDDEN {
                    for k in 0..OUTPUTS {
                        self.w_ho[j][k] -= lr * out_delta[k] * hidden[j];
                    }
                }
                for k in 0..OUTPUTS {
                    self.b_o[k] -= lr * out_delta[k];
                }

                // Обновляем веса и смещения скрытого слоя
                for i in 0..INPUTS {
                    for j in 0..HIDDEN {
                        self.w_ih[i][j] -= lr * hidden_delta[j] * sample.input[i];
                    }
                }
                for j in 0..HIDDEN {
                    self.b_h[j] -= lr * hidden_delta[j];
                }
            }

            // Каждые 2000 эпох или в конце выводим среднюю ошибку
            if epoch % 2000 == 0 || epoch + 1 == epochs {
                println!(
                    "epoch {}: avg error = {:.6}",
                    epoch + 1,
                    total_error / samples.len() as f64
                );
            }
        }
    }
}
