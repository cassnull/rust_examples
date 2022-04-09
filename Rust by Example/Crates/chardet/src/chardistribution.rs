use super::big5freq;

const ENOUGH_DATA_THRESHOLD: usize = 1024;
const SURE_YES: f64 = 0.99;
const SURE_NO: f64 = 0.01;
const MINIMUM_DATA_THRESHOLD: usize = 3;

pub struct Big5DistributionAnalysis {
    /// Таблица отображения для получения частотного порядка из char order (получить из get_order())
    char_to_freq_order: [i32; big5freq::BIG5_TABLE_SIZE],
    /// Размер таблицы char_to_freq_order
    table_size: usize,
    /// Это постоянная величина, которая варьируется от языка к языку, используемые при расчете уверенности.
    /// См. http://www.mozilla.org/projects/intl/UniversalCharsetDetection.html для более подробной информации.
    typical_distribution_ratio: f64,
    done: bool,
    total_chars: usize,
    freq_chars: usize,
}

impl Big5DistributionAnalysis {
    pub fn get_order(&self, byte_str: &[u8]) -> Option<usize> {
        // для кодирования big5 нас интересуют
        //   диапазон первых байтов: 0xa4 -- 0xfe
        //   диапазон второго байта: 0x40 -- 0x7e , 0xa1 -- 0xfe
        // здесь не нужна валидация. Стейт-машина уже сделала это
        let (first_char, second_char) = (byte_str[0], byte_str[1]);
        if first_char >= 0xA4 {
            if second_char >= 0xA1 {
                Some((157 * (first_char - 0xA4) + second_char - 0xA1 + 63) as usize)
            } else {
                Some((157 * (first_char - 0xA4) + second_char - 0x40) as usize)
            }
        } else {
            None
        }
    }

    /// Сброс анализатора, очистка любого состояния
    pub fn reset(&mut self) {
        // Если этот флаг установлен в true, обнаружение выполнено и вывод сделан
        self.done = false;
        // Всего встреченных символов
        self.total_chars = 0;
        // Количество символов, частотный порядок которых меньше 512
        self.freq_chars = 0;
    }

    /// Подача символа известной длины
    pub fn feed(&mut self, char: &[u8], char_len: usize) {
        let order = if char_len == 2 {
            // в нашем анализе распределения нам важен только 2-байтовый символ
            self.get_order(char)
        } else {
            None
        };
        if let Some(order) = order {
            self.total_chars += 1;
            // order действителен
            if order < self.table_size && 512 > self.char_to_freq_order[order] {
                self.freq_chars += 1;
            }
        }
    }

    /// Возвращение уверенности на основе существующих данных
    pub fn get_confidence(&self) -> f64 {
        // Если мы не получили ни одного символа в нашем диапазоне рассмотрения, возвращаем отрицательный ответ
        if self.total_chars == 0 || self.freq_chars <= MINIMUM_DATA_THRESHOLD {
            return SURE_NO;
        }

        if self.total_chars != self.freq_chars {
            let r = self.freq_chars as f64
                / ((self.total_chars - self.freq_chars) as f64 * self.typical_distribution_ratio);
            if r < SURE_YES {
                return r;
            }
        }

        // Нормализовать уверенность (мы не хотим быть уверенными на 100%)
        SURE_YES
    }

    /// Не обязательно получать все данные, чтобы сделать вывод.
    /// Для определения кодовой таблицы достаточно определенного количества данных.
    pub fn got_enough_data(&self) -> bool {
        self.total_chars > ENOUGH_DATA_THRESHOLD
    }
}

impl Default for Big5DistributionAnalysis {
    fn default() -> Self {
        Self {
            char_to_freq_order: big5freq::BIG5_CHAR_TO_FREQ_ORDER,
            table_size: big5freq::BIG5_TABLE_SIZE,
            typical_distribution_ratio: big5freq::BIG5_TYPICAL_DISTRIBUTION_RATIO,
            done: false,
            total_chars: 0,
            freq_chars: 0,
        }
    }
}
