use super::enums;

pub const BIG5_CLS_SIZE: usize = 256;

#[rustfmt::skip]
pub const BIG5_CLS: [i32; BIG5_CLS_SIZE] = [
    1, 1, 1, 1, 1, 1, 1, 1, // 00 - 07    // разрешить 0x00 в качестве законного значения
    1, 1, 1, 1, 1, 1, 0, 0, // 08 - 0f
    1, 1, 1, 1, 1, 1, 1, 1, // 10 - 17
    1, 1, 1, 0, 1, 1, 1, 1, // 18 - 1f
    1, 1, 1, 1, 1, 1, 1, 1, // 20 - 27
    1, 1, 1, 1, 1, 1, 1, 1, // 28 - 2f
    1, 1, 1, 1, 1, 1, 1, 1, // 30 - 37
    1, 1, 1, 1, 1, 1, 1, 1, // 38 - 3f
    2, 2, 2, 2, 2, 2, 2, 2, // 40 - 47
    2, 2, 2, 2, 2, 2, 2, 2, // 48 - 4f
    2, 2, 2, 2, 2, 2, 2, 2, // 50 - 57
    2, 2, 2, 2, 2, 2, 2, 2, // 58 - 5f
    2, 2, 2, 2, 2, 2, 2, 2, // 60 - 67
    2, 2, 2, 2, 2, 2, 2, 2, // 68 - 6f
    2, 2, 2, 2, 2, 2, 2, 2, // 70 - 77
    2, 2, 2, 2, 2, 2, 2, 1, // 78 - 7f
    4, 4, 4, 4, 4, 4, 4, 4, // 80 - 87
    4, 4, 4, 4, 4, 4, 4, 4, // 88 - 8f
    4, 4, 4, 4, 4, 4, 4, 4, // 90 - 97
    4, 4, 4, 4, 4, 4, 4, 4, // 98 - 9f
    4, 3, 3, 3, 3, 3, 3, 3, // a0 - a7
    3, 3, 3, 3, 3, 3, 3, 3, // a8 - af
    3, 3, 3, 3, 3, 3, 3, 3, // b0 - b7
    3, 3, 3, 3, 3, 3, 3, 3, // b8 - bf
    3, 3, 3, 3, 3, 3, 3, 3, // c0 - c7
    3, 3, 3, 3, 3, 3, 3, 3, // c8 - cf
    3, 3, 3, 3, 3, 3, 3, 3, // d0 - d7
    3, 3, 3, 3, 3, 3, 3, 3, // d8 - df
    3, 3, 3, 3, 3, 3, 3, 3, // e0 - e7
    3, 3, 3, 3, 3, 3, 3, 3, // e8 - ef
    3, 3, 3, 3, 3, 3, 3, 3, // f0 - f7
    3, 3, 3, 3, 3, 3, 3, 0, // f8 - ff
];

pub const BIG5_ST_SIZE: usize = 24;

#[rustfmt::skip]
pub const BIG5_ST: [enums::MachineState; BIG5_ST_SIZE] = [
    enums::MachineState::Error, enums::MachineState::Start, enums::MachineState::Start,  enums::MachineState::None, // 00-03
    enums::MachineState::Error, enums::MachineState::Error, enums::MachineState::Error, enums::MachineState::Error, // 04-07
    enums::MachineState::Error, enums::MachineState::Error, enums::MachineState::ItsMe, enums::MachineState::ItsMe, // 08-0b
    enums::MachineState::ItsMe, enums::MachineState::ItsMe, enums::MachineState::ItsMe, enums::MachineState::Error, // 0c-0f
    enums::MachineState::Error, enums::MachineState::Start, enums::MachineState::Start, enums::MachineState::Start, // 0f-13
    enums::MachineState::Start, enums::MachineState::Start, enums::MachineState::Start, enums::MachineState::Start, // 14-17
];

pub const BIG5_CHAR_LEN_TABLE_SIZE: usize = 5;

pub const BIG5_CHAR_LEN_TABLE: [i32; BIG5_CHAR_LEN_TABLE_SIZE] = [0, 1, 1, 2, 0];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big5_cls_len() {
        assert_eq!(BIG5_CLS.len(), BIG5_CLS_SIZE);
    }

    #[test]
    fn test_big5_st_len() {
        assert_eq!(BIG5_ST.len(), BIG5_ST_SIZE);
    }

    #[test]
    fn test_big5_char_len_table_len() {
        assert_eq!(BIG5_CHAR_LEN_TABLE.len(), BIG5_CHAR_LEN_TABLE_SIZE);
    }
}
