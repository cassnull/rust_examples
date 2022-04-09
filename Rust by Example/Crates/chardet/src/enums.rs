//! Все Enums, которые используются во всем пакете chardet.

/// Это перечисление представляет различные состояния, в которых может находиться универсальный детектор.
pub enum InputState {
    PureAscii,
    EscAscii,
    HighByte,
}

/// Это перечисление представляет различные языковые фильтры, которые мы можем применить к `UniversalDetector`.
pub enum LanguageFilter {
    ChineseSimplified = 0x01,
    ChineseTraditional = 0x02,
    Japanese = 0x04,
    Korean = 0x08,
    NonCjk = 0x10,
    All = 0x1F,
    Chinese =
        LanguageFilter::ChineseSimplified as isize | LanguageFilter::ChineseTraditional as isize,
    Cjk = LanguageFilter::Chinese as isize
        | LanguageFilter::Japanese as isize
        | LanguageFilter::Korean as isize,
}

/// Это перечисление представляет различные состояния, в которых может находиться проберри.
pub enum ProbingState {
    Detecting,
    FoundIt,
    NotMe,
}

/// Это перечисление представляет различные состояния, в которых может находиться машина состояний.
#[derive(PartialEq, Copy, Clone)]
pub enum MachineState {
    None = 3,
    Start = 0,
    Error = 1,
    ItsMe = 2,
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState::None
    }
}

/// Это перечисление представляет вероятность того, что символ следует за предыдущим.
pub enum SequenceLikelihood {
    Negative,
    Unlikely,
    Likely,
    Positive,
}

impl SequenceLikelihood {
    /// Количество категорий вероятности в перечислении.
    pub fn get_num_categories() -> usize {
        4
    }
}

/// Это перечисление представляет различные категории языковых моделей для
/// `SingleByteCharsetProber`, в которые помещаются символы.
/// Все, что меньше CONTROL, считается буквой.
pub enum CharacterCategory {
    Undefined = 255,
    Linebreak = 254,
    Symbol = 253,
    Digit = 252,
    Control = 251,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_filter_chinese() {
        assert_eq!(LanguageFilter::Chinese as isize, 3);
    }

    #[test]
    fn test_language_filter_cjk() {
        assert_eq!(LanguageFilter::Cjk as isize, 15);
    }

    #[test]
    fn test_sequence_likelihood_get_num_categories() {
        assert_eq!(SequenceLikelihood::get_num_categories(), 4);
    }
}
