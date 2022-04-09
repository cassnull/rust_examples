use super::enums;
use super::mbcssm;

/// Машина состояний для проверки последовательности байтов для определенной кодировки.
/// Для каждого байта, полученного детектором, он подает этот байт в каждую доступную активную стейт-машину, по одному байту за раз.
/// Стейт-машина изменяет свое состояние на основе своего предыдущего состояния и полученного байта.
/// В стейт-машине есть 3 состояния, которые представляют интерес для автодетектора:
/// - START: Это состояние, с которого следует начинать, или была определена законная последовательность байтов
///          (т.е. допустимая точка кода) для символа.
/// - ME:    Это состояние указывает на то, что стейт-машина идентифицировала последовательность байтов, специфичную для кодовой таблицы,
///          для которой она предназначена, и что не существует другой возможной кодировки, которая может содержать эту последовательность байтов.
///          Это приведет к немедленному положительному ответу детектора.
/// - ERROR: Это состояние указывает на то, что стейт-машина определила недопустимую последовательность байтов для данной кодировки.
///          Это приведет к немедленному отрицательному ответу для данной кодировки.
///          В дальнейшем детектор будет исключать эту кодировку из рассмотрения.       
pub struct CodingStateMachine {
    model: mbcssm::Big5SmModel,
    curr_byte_pos: usize,
    curr_char_len: usize,
    curr_state: enums::MachineState,
}

impl CodingStateMachine {
    pub fn new(model: mbcssm::Big5SmModel) -> Self {
        Self {
            model,
            curr_byte_pos: 0,
            curr_char_len: 0,
            curr_state: enums::MachineState::Start,
        }
    }

    pub fn reset(&mut self) {
        self.curr_state = enums::MachineState::Start
    }

    pub fn next_state(&mut self, class_table_index: usize) -> enums::MachineState {
        // для каждого байта мы получаем его класс
        // если это первый байт, мы также получаем длину байта
        let byte_class = self.model.class_table[class_table_index];
        if self.curr_state == enums::MachineState::Start {
            self.curr_byte_pos = 0;
            self.curr_char_len = self.model.char_len_table[byte_class];
        }
        // из byte_class и таблицы state_table мы получаем его следующее состояние
        let curr_state = self.curr_state as usize * self.model.class_factor + byte_class;
        self.curr_state = self.model.state_table[curr_state];
        self.curr_byte_pos += 1;
        self.curr_state
    }

    pub fn get_current_charlen(&self) -> usize {
        self.curr_char_len
    }

    pub fn get_coding_state_machine(&self) -> &String {
        &self.model.name
    }
}
