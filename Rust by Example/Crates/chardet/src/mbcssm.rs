use super::enums;

pub const BIG5_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// BIG5
pub const BIG5_CLS: [usize; BIG5_CLS_SIZE] = [
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
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, enums::MachineState::State3, // 00-03
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 04-07
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 08-0b
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::Error, // 0c-0f
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 10-13
    enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 14-17
];

pub const BIG5_CHAR_LEN_TABLE_SIZE: usize = 5;

pub const BIG5_CHAR_LEN_TABLE: [usize; BIG5_CHAR_LEN_TABLE_SIZE] = [0, 1, 1, 2, 0];

pub struct Big5SmModel {
    pub class_table: [usize; BIG5_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; BIG5_ST_SIZE],
    pub char_len_table: [usize; BIG5_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for Big5SmModel {
    fn default() -> Self {
        Self {
            class_table: BIG5_CLS,
            class_factor: BIG5_CHAR_LEN_TABLE_SIZE,
            state_table: BIG5_ST,
            char_len_table: BIG5_CHAR_LEN_TABLE,
            name: "Big5".into(),
        }
    }
}

pub const CP949_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// CP949
pub const CP949_CLS: [usize; CP949_CLS_SIZE] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, // 00 - 0f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, // 10 - 1f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 20 - 2f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 30 - 3f
    1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // 40 - 4f
    4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, // 50 - 5f
    1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, // 60 - 6f
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, // 70 - 7f
    0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, // 80 - 8f
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, // 90 - 9f
    6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, // a0 - af
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, // b0 - bf
    7, 7, 7, 7, 7, 7, 9, 2, 2, 3, 2, 2, 2, 2, 2, 2, // c0 - cf
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // d0 - df
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // e0 - ef
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, // f0 - ff
];

pub const CP949_ST_SIZE: usize = 70;

#[rustfmt::skip]
pub const CP949_ST: [enums::MachineState; CP949_ST_SIZE] = [
    // cls=                  0                            1                            2                            3                            4  // previous state
    //                       5                            6                            7                            8                            9  // previous state
    enums::MachineState::Error,  enums::MachineState::Start, enums::MachineState::State3,  enums::MachineState::Error,  enums::MachineState::Start, // enums::MachineState::Start
    enums::MachineState::Start, enums::MachineState::State4, enums::MachineState::State5,  enums::MachineState::Error, enums::MachineState::State6, // enums::MachineState::Start
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // enums::MachineState::Error
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // enums::MachineState::Error
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // enums::MachineState::ItsMe
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // enums::MachineState::ItsMe
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Error, // enums::MachineState::State3
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State3
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State4
    enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State4
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State5
    enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State5
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State6
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State6
];

pub const CP949_CHAR_LEN_TABLE_SIZE: usize = 10;

pub const CP949_CHAR_LEN_TABLE: [usize; CP949_CHAR_LEN_TABLE_SIZE] = [0, 1, 2, 0, 1, 1, 2, 2, 0, 2];

pub struct Cp949SmModel {
    pub class_table: [usize; CP949_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; CP949_ST_SIZE],
    pub char_len_table: [usize; CP949_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for Cp949SmModel {
    fn default() -> Self {
        Self {
            class_table: CP949_CLS,
            class_factor: CP949_CHAR_LEN_TABLE_SIZE,
            state_table: CP949_ST,
            char_len_table: CP949_CHAR_LEN_TABLE,
            name: "CP949".into(),
        }
    }
}

pub const EUCJP_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// EUC-JP
pub const EUCJP_CLS: [usize; EUCJP_CLS_SIZE] = [
    4, 4, 4, 4, 4, 4, 4, 4, // 00 - 07
    4, 4, 4, 4, 4, 4, 5, 5, // 08 - 0f
    4, 4, 4, 4, 4, 4, 4, 4, // 10 - 17
    4, 4, 4, 5, 4, 4, 4, 4, // 18 - 1f
    4, 4, 4, 4, 4, 4, 4, 4, // 20 - 27
    4, 4, 4, 4, 4, 4, 4, 4, // 28 - 2f
    4, 4, 4, 4, 4, 4, 4, 4, // 30 - 37
    4, 4, 4, 4, 4, 4, 4, 4, // 38 - 3f
    4, 4, 4, 4, 4, 4, 4, 4, // 40 - 47
    4, 4, 4, 4, 4, 4, 4, 4, // 48 - 4f
    4, 4, 4, 4, 4, 4, 4, 4, // 50 - 57
    4, 4, 4, 4, 4, 4, 4, 4, // 58 - 5f
    4, 4, 4, 4, 4, 4, 4, 4, // 60 - 67
    4, 4, 4, 4, 4, 4, 4, 4, // 68 - 6f
    4, 4, 4, 4, 4, 4, 4, 4, // 70 - 77
    4, 4, 4, 4, 4, 4, 4, 4, // 78 - 7f
    5, 5, 5, 5, 5, 5, 5, 5, // 80 - 87
    5, 5, 5, 5, 5, 5, 1, 3, // 88 - 8f
    5, 5, 5, 5, 5, 5, 5, 5, // 90 - 97
    5, 5, 5, 5, 5, 5, 5, 5, // 98 - 9f
    5, 2, 2, 2, 2, 2, 2, 2, // a0 - a7
    2, 2, 2, 2, 2, 2, 2, 2, // a8 - af
    2, 2, 2, 2, 2, 2, 2, 2, // b0 - b7
    2, 2, 2, 2, 2, 2, 2, 2, // b8 - bf
    2, 2, 2, 2, 2, 2, 2, 2, // c0 - c7
    2, 2, 2, 2, 2, 2, 2, 2, // c8 - cf
    2, 2, 2, 2, 2, 2, 2, 2, // d0 - d7
    2, 2, 2, 2, 2, 2, 2, 2, // d8 - df
    0, 0, 0, 0, 0, 0, 0, 0, // e0 - e7
    0, 0, 0, 0, 0, 0, 0, 0, // e8 - ef
    0, 0, 0, 0, 0, 0, 0, 0, // f0 - f7
    0, 0, 0, 0, 0, 0, 0, 5, // f8 - ff
];

pub const EUCJP_ST_SIZE: usize = 40;

#[rustfmt::skip]
pub const EUCJP_ST: [enums::MachineState; EUCJP_ST_SIZE] = [
    enums::MachineState::State3, enums::MachineState::State4, enums::MachineState::State3, enums::MachineState::State5, // 00-03
     enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 04-07
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 08-0b
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 0c-0f
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::Start,  enums::MachineState::Error, // 10-13
     enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 14-17
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Error, // 18-1b
     enums::MachineState::Error,  enums::MachineState::Error, enums::MachineState::State3,  enums::MachineState::Error, // 1c-1f
    enums::MachineState::State3,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 20-23
     enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 24-27
];

pub const EUCJP_CHAR_LEN_TABLE_SIZE: usize = 6;

pub const EUCJP_CHAR_LEN_TABLE: [usize; EUCJP_CHAR_LEN_TABLE_SIZE] = [2, 2, 2, 3, 1, 0];

pub struct EucjpSmModel {
    pub class_table: [usize; EUCJP_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; EUCJP_ST_SIZE],
    pub char_len_table: [usize; EUCJP_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for EucjpSmModel {
    fn default() -> Self {
        Self {
            class_table: EUCJP_CLS,
            class_factor: EUCJP_CHAR_LEN_TABLE_SIZE,
            state_table: EUCJP_ST,
            char_len_table: EUCJP_CHAR_LEN_TABLE,
            name: "EUC-JP".into(),
        }
    }
}

pub const EUCKR_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// EUC-KR
pub const EUCKR_CLS: [usize; EUCKR_CLS_SIZE] = [
    1, 1, 1, 1, 1, 1, 1, 1, // 00 - 07
    1, 1, 1, 1, 1, 1, 0, 0, // 08 - 0f
    1, 1, 1, 1, 1, 1, 1, 1, // 10 - 17
    1, 1, 1, 0, 1, 1, 1, 1, // 18 - 1f
    1, 1, 1, 1, 1, 1, 1, 1, // 20 - 27
    1, 1, 1, 1, 1, 1, 1, 1, // 28 - 2f
    1, 1, 1, 1, 1, 1, 1, 1, // 30 - 37
    1, 1, 1, 1, 1, 1, 1, 1, // 38 - 3f
    1, 1, 1, 1, 1, 1, 1, 1, // 40 - 47
    1, 1, 1, 1, 1, 1, 1, 1, // 48 - 4f
    1, 1, 1, 1, 1, 1, 1, 1, // 50 - 57
    1, 1, 1, 1, 1, 1, 1, 1, // 58 - 5f
    1, 1, 1, 1, 1, 1, 1, 1, // 60 - 67
    1, 1, 1, 1, 1, 1, 1, 1, // 68 - 6f
    1, 1, 1, 1, 1, 1, 1, 1, // 70 - 77
    1, 1, 1, 1, 1, 1, 1, 1, // 78 - 7f
    0, 0, 0, 0, 0, 0, 0, 0, // 80 - 87
    0, 0, 0, 0, 0, 0, 0, 0, // 88 - 8f
    0, 0, 0, 0, 0, 0, 0, 0, // 90 - 97
    0, 0, 0, 0, 0, 0, 0, 0, // 98 - 9f
    0, 2, 2, 2, 2, 2, 2, 2, // a0 - a7
    2, 2, 2, 2, 2, 3, 3, 3, // a8 - af
    2, 2, 2, 2, 2, 2, 2, 2, // b0 - b7
    2, 2, 2, 2, 2, 2, 2, 2, // b8 - bf
    2, 2, 2, 2, 2, 2, 2, 2, // c0 - c7
    2, 3, 2, 2, 2, 2, 2, 2, // c8 - cf
    2, 2, 2, 2, 2, 2, 2, 2, // d0 - d7
    2, 2, 2, 2, 2, 2, 2, 2, // d8 - df
    2, 2, 2, 2, 2, 2, 2, 2, // e0 - e7
    2, 2, 2, 2, 2, 2, 2, 2, // e8 - ef
    2, 2, 2, 2, 2, 2, 2, 2, // f0 - f7
    2, 2, 2, 2, 2, 2, 2, 0, // f8 - ff
];

pub const EUCKR_ST_SIZE: usize = 16;

#[rustfmt::skip]
pub const EUCKR_ST: [enums::MachineState; EUCKR_ST_SIZE] = [
    enums::MachineState::Error,  enums::MachineState::Start, enums::MachineState::State3,  enums::MachineState::Error, // 00-03
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 04-07
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 08-0b
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 0c-0f
];

pub const EUCKR_CHAR_LEN_TABLE_SIZE: usize = 4;

pub const EUCKR_CHAR_LEN_TABLE: [usize; EUCKR_CHAR_LEN_TABLE_SIZE] = [0, 1, 2, 0];

pub struct EuckrSmModel {
    pub class_table: [usize; EUCKR_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; EUCKR_ST_SIZE],
    pub char_len_table: [usize; EUCKR_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for EuckrSmModel {
    fn default() -> Self {
        Self {
            class_table: EUCKR_CLS,
            class_factor: EUCKR_CHAR_LEN_TABLE_SIZE,
            state_table: EUCKR_ST,
            char_len_table: EUCKR_CHAR_LEN_TABLE,
            name: "EUC-KR".into(),
        }
    }
}

pub const JOHAB_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// JOHAB
pub const JOHAB_CLS: [usize; JOHAB_CLS_SIZE] = [
    4,4,4,4,4,4,4,4, // 00 - 07
    4,4,4,4,4,4,0,0, // 08 - 0f
    4,4,4,4,4,4,4,4, // 10 - 17
    4,4,4,0,4,4,4,4, // 18 - 1f
    4,4,4,4,4,4,4,4, // 20 - 27
    4,4,4,4,4,4,4,4, // 28 - 2f
    4,3,3,3,3,3,3,3, // 30 - 37
    3,3,3,3,3,3,3,3, // 38 - 3f
    3,1,1,1,1,1,1,1, // 40 - 47
    1,1,1,1,1,1,1,1, // 48 - 4f
    1,1,1,1,1,1,1,1, // 50 - 57
    1,1,1,1,1,1,1,1, // 58 - 5f
    1,1,1,1,1,1,1,1, // 60 - 67
    1,1,1,1,1,1,1,1, // 68 - 6f
    1,1,1,1,1,1,1,1, // 70 - 77
    1,1,1,1,1,1,1,2, // 78 - 7f
    6,6,6,6,8,8,8,8, // 80 - 87
    8,8,8,8,8,8,8,8, // 88 - 8f
    8,7,7,7,7,7,7,7, // 90 - 97
    7,7,7,7,7,7,7,7, // 98 - 9f
    7,7,7,7,7,7,7,7, // a0 - a7
    7,7,7,7,7,7,7,7, // a8 - af
    7,7,7,7,7,7,7,7, // b0 - b7
    7,7,7,7,7,7,7,7, // b8 - bf
    7,7,7,7,7,7,7,7, // c0 - c7
    7,7,7,7,7,7,7,7, // c8 - cf
    7,7,7,7,5,5,5,5, // d0 - d7
    5,9,9,9,9,9,9,5, // d8 - df
    9,9,9,9,9,9,9,9, // e0 - e7
    9,9,9,9,9,9,9,9, // e8 - ef
    9,9,9,9,9,9,9,9, // f0 - f7
    9,9,5,5,5,5,5,0, // f8 - ff
];

pub const JOHAB_ST_SIZE: usize = 50;

#[rustfmt::skip]
pub const JOHAB_ST: [enums::MachineState; JOHAB_ST_SIZE] = [
    // cls=                  0                            1                            2                            3                            4  // previous state
    //                       5                            6                            7                            8                            9  // previous state
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::Start
    enums::MachineState::Error,  enums::MachineState::Error, enums::MachineState::State3, enums::MachineState::State3, enums::MachineState::State4, // enums::MachineState::Start
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // enums::MachineState::ItsMe
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // enums::MachineState::ItsMe
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // enums::MachineState::Error
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // enums::MachineState::Error
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Error, // enums::MachineState::State3
    enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // enums::MachineState::State3
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Error, // enums::MachineState::State4
    enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Start, // enums::MachineState::State4
];

pub const JOHAB_CHAR_LEN_TABLE_SIZE: usize = 10;

pub const JOHAB_CHAR_LEN_TABLE: [usize; JOHAB_CHAR_LEN_TABLE_SIZE] = [0, 1, 1, 1, 1, 0, 0, 2, 2, 2];

pub struct JohabSmModel {
    pub class_table: [usize; JOHAB_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; JOHAB_ST_SIZE],
    pub char_len_table: [usize; JOHAB_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for JohabSmModel {
    fn default() -> Self {
        Self {
            class_table: JOHAB_CLS,
            class_factor: JOHAB_CHAR_LEN_TABLE_SIZE,
            state_table: JOHAB_ST,
            char_len_table: JOHAB_CHAR_LEN_TABLE,
            name: "Johab".into(),
        }
    }
}

pub const EUCTW_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// EUC-TW
pub const EUCTW_CLS: [usize; EUCTW_CLS_SIZE] = [
    2, 2, 2, 2, 2, 2, 2, 2, // 00 - 07
    2, 2, 2, 2, 2, 2, 0, 0, // 08 - 0f
    2, 2, 2, 2, 2, 2, 2, 2, // 10 - 17
    2, 2, 2, 0, 2, 2, 2, 2, // 18 - 1f
    2, 2, 2, 2, 2, 2, 2, 2, // 20 - 27
    2, 2, 2, 2, 2, 2, 2, 2, // 28 - 2f
    2, 2, 2, 2, 2, 2, 2, 2, // 30 - 37
    2, 2, 2, 2, 2, 2, 2, 2, // 38 - 3f
    2, 2, 2, 2, 2, 2, 2, 2, // 40 - 47
    2, 2, 2, 2, 2, 2, 2, 2, // 48 - 4f
    2, 2, 2, 2, 2, 2, 2, 2, // 50 - 57
    2, 2, 2, 2, 2, 2, 2, 2, // 58 - 5f
    2, 2, 2, 2, 2, 2, 2, 2, // 60 - 67
    2, 2, 2, 2, 2, 2, 2, 2, // 68 - 6f
    2, 2, 2, 2, 2, 2, 2, 2, // 70 - 77
    2, 2, 2, 2, 2, 2, 2, 2, // 78 - 7f
    0, 0, 0, 0, 0, 0, 0, 0, // 80 - 87
    0, 0, 0, 0, 0, 0, 6, 0, // 88 - 8f
    0, 0, 0, 0, 0, 0, 0, 0, // 90 - 97
    0, 0, 0, 0, 0, 0, 0, 0, // 98 - 9f
    0, 3, 4, 4, 4, 4, 4, 4, // a0 - a7
    5, 5, 1, 1, 1, 1, 1, 1, // a8 - af
    1, 1, 1, 1, 1, 1, 1, 1, // b0 - b7
    1, 1, 1, 1, 1, 1, 1, 1, // b8 - bf
    1, 1, 3, 1, 3, 3, 3, 3, // c0 - c7
    3, 3, 3, 3, 3, 3, 3, 3, // c8 - cf
    3, 3, 3, 3, 3, 3, 3, 3, // d0 - d7
    3, 3, 3, 3, 3, 3, 3, 3, // d8 - df
    3, 3, 3, 3, 3, 3, 3, 3, // e0 - e7
    3, 3, 3, 3, 3, 3, 3, 3, // e8 - ef
    3, 3, 3, 3, 3, 3, 3, 3, // f0 - f7
    3, 3, 3, 3, 3, 3, 3, 0, // f8 - ff
];

pub const EUCTW_ST_SIZE: usize = 48;

#[rustfmt::skip]
pub const EUCTW_ST: [enums::MachineState; EUCTW_ST_SIZE] = [
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start, enums::MachineState::State3, // 00-03
    enums::MachineState::State3, enums::MachineState::State3, enums::MachineState::State4,  enums::MachineState::Error, // 04-07
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 08-0b
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 0c-0f
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 10-13
     enums::MachineState::ItsMe,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Error, // 14-17
     enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Error, // 18-1b
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 1c-1f
    enums::MachineState::State5,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 20-23
     enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 24-27
     enums::MachineState::Start,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 28-2b
     enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 2c-2f
];

pub const EUCTW_CHAR_LEN_TABLE_SIZE: usize = 7;

pub const EUCTW_CHAR_LEN_TABLE: [usize; EUCTW_CHAR_LEN_TABLE_SIZE] = [0, 0, 1, 2, 2, 2, 3];

pub struct EuctwSmModel {
    pub class_table: [usize; EUCTW_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; EUCTW_ST_SIZE],
    pub char_len_table: [usize; EUCTW_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for EuctwSmModel {
    fn default() -> Self {
        Self {
            class_table: EUCTW_CLS,
            class_factor: EUCTW_CHAR_LEN_TABLE_SIZE,
            state_table: EUCTW_ST,
            char_len_table: EUCTW_CHAR_LEN_TABLE,
            name: "x-euc-tw".into(),
        }
    }
}

pub const GB2312_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// GB2312
pub const GB2312_CLS: [usize; GB2312_CLS_SIZE] = [
    1, 1, 1, 1, 1, 1, 1, 1, // 00 - 07
    1, 1, 1, 1, 1, 1, 0, 0, // 08 - 0f
    1, 1, 1, 1, 1, 1, 1, 1, // 10 - 17
    1, 1, 1, 0, 1, 1, 1, 1, // 18 - 1f
    1, 1, 1, 1, 1, 1, 1, 1, // 20 - 27
    1, 1, 1, 1, 1, 1, 1, 1, // 28 - 2f
    3, 3, 3, 3, 3, 3, 3, 3, // 30 - 37
    3, 3, 1, 1, 1, 1, 1, 1, // 38 - 3f
    2, 2, 2, 2, 2, 2, 2, 2, // 40 - 47
    2, 2, 2, 2, 2, 2, 2, 2, // 48 - 4f
    2, 2, 2, 2, 2, 2, 2, 2, // 50 - 57
    2, 2, 2, 2, 2, 2, 2, 2, // 58 - 5f
    2, 2, 2, 2, 2, 2, 2, 2, // 60 - 67
    2, 2, 2, 2, 2, 2, 2, 2, // 68 - 6f
    2, 2, 2, 2, 2, 2, 2, 2, // 70 - 77
    2, 2, 2, 2, 2, 2, 2, 4, // 78 - 7f
    5, 6, 6, 6, 6, 6, 6, 6, // 80 - 87
    6, 6, 6, 6, 6, 6, 6, 6, // 88 - 8f
    6, 6, 6, 6, 6, 6, 6, 6, // 90 - 97
    6, 6, 6, 6, 6, 6, 6, 6, // 98 - 9f
    6, 6, 6, 6, 6, 6, 6, 6, // a0 - a7
    6, 6, 6, 6, 6, 6, 6, 6, // a8 - af
    6, 6, 6, 6, 6, 6, 6, 6, // b0 - b7
    6, 6, 6, 6, 6, 6, 6, 6, // b8 - bf
    6, 6, 6, 6, 6, 6, 6, 6, // c0 - c7
    6, 6, 6, 6, 6, 6, 6, 6, // c8 - cf
    6, 6, 6, 6, 6, 6, 6, 6, // d0 - d7
    6, 6, 6, 6, 6, 6, 6, 6, // d8 - df
    6, 6, 6, 6, 6, 6, 6, 6, // e0 - e7
    6, 6, 6, 6, 6, 6, 6, 6, // e8 - ef
    6, 6, 6, 6, 6, 6, 6, 6, // f0 - f7
    6, 6, 6, 6, 6, 6, 6, 0, // f8 - ff
];

pub const GB2312_ST_SIZE: usize = 48;

#[rustfmt::skip]
pub const GB2312_ST: [enums::MachineState; GB2312_ST_SIZE] = [
     enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 00-03
     enums::MachineState::Start,  enums::MachineState::Start, enums::MachineState::State3,  enums::MachineState::Error, // 04-07
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 08-0b
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 0c-0f
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 10-13
     enums::MachineState::ItsMe,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start, // 14-17
    enums::MachineState::State4,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 18-1b
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 1c-1f
     enums::MachineState::Error,  enums::MachineState::Error, enums::MachineState::State5,  enums::MachineState::Error, // 20-23
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::ItsMe,  enums::MachineState::Error, // 24-27
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 28-2b
     enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 2c-2f
];

pub const GB2312_CHAR_LEN_TABLE_SIZE: usize = 7;

/// Если быть точным, то длина класса 6 может быть либо 2, либо 4.
/// Но нет необходимости различать их, поскольку он используется только для частотного анализа, и мы проверяем каждый диапазон кода и там.
/// Поэтому здесь можно установить значение 2.
pub const GB2312_CHAR_LEN_TABLE: [usize; GB2312_CHAR_LEN_TABLE_SIZE] = [0, 1, 1, 1, 1, 1, 2];

pub struct Gb2312SmModel {
    pub class_table: [usize; GB2312_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; GB2312_ST_SIZE],
    pub char_len_table: [usize; GB2312_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for Gb2312SmModel {
    fn default() -> Self {
        Self {
            class_table: GB2312_CLS,
            class_factor: GB2312_CHAR_LEN_TABLE_SIZE,
            state_table: GB2312_ST,
            char_len_table: GB2312_CHAR_LEN_TABLE,
            name: "GB2312".into(),
        }
    }
}

pub const SJIS_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// Shift_JIS
pub const SJIS_CLS: [usize; SJIS_CLS_SIZE] = [
    1, 1, 1, 1, 1, 1, 1, 1, // 00 - 07
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
    3, 3, 3, 3, 3, 2, 2, 3, // 80 - 87
    3, 3, 3, 3, 3, 3, 3, 3, // 88 - 8f
    3, 3, 3, 3, 3, 3, 3, 3, // 90 - 97
    3, 3, 3, 3, 3, 3, 3, 3, // 98 - 9f
    // 0xa0 является незаконным в кодировке sjis, но некоторые страницы содержат такой байт. Нам нужно быть более снисходительными к ошибкам
    2, 2, 2, 2, 2, 2, 2, 2, // a0 - a7
    2, 2, 2, 2, 2, 2, 2, 2, // a8 - af
    2, 2, 2, 2, 2, 2, 2, 2, // b0 - b7
    2, 2, 2, 2, 2, 2, 2, 2, // b8 - bf
    2, 2, 2, 2, 2, 2, 2, 2, // c0 - c7
    2, 2, 2, 2, 2, 2, 2, 2, // c8 - cf
    2, 2, 2, 2, 2, 2, 2, 2, // d0 - d7
    2, 2, 2, 2, 2, 2, 2, 2, // d8 - df
    3, 3, 3, 3, 3, 3, 3, 3, // e0 - e7
    3, 3, 3, 3, 3, 4, 4, 4, // e8 - ef
    3, 3, 3, 3, 3, 3, 3, 3, // f0 - f7
    3, 3, 3, 3, 3, 0, 0, 0, // f8 - ff
];

pub const SJIS_ST_SIZE: usize = 24;

#[rustfmt::skip]
pub const SJIS_ST: [enums::MachineState; SJIS_ST_SIZE] = [
    enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, enums::MachineState::State3, // 00-03
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 04-07
    enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 08-0b
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 0c-0f
    enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::Error,  enums::MachineState::Error, // 10-13
    enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start,  enums::MachineState::Start, // 14-17
];

pub const SJIS_CHAR_LEN_TABLE_SIZE: usize = 6;

pub const SJIS_CHAR_LEN_TABLE: [usize; SJIS_CHAR_LEN_TABLE_SIZE] = [0, 1, 1, 2, 0, 0];

pub struct SjisSmModel {
    pub class_table: [usize; SJIS_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; SJIS_ST_SIZE],
    pub char_len_table: [usize; SJIS_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for SjisSmModel {
    fn default() -> Self {
        Self {
            class_table: SJIS_CLS,
            class_factor: SJIS_CHAR_LEN_TABLE_SIZE,
            state_table: SJIS_ST,
            char_len_table: SJIS_CHAR_LEN_TABLE,
            name: "Shift_JIS".into(),
        }
    }
}

pub const UCS2BE_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// UCS2-BE
pub const UCS2BE_CLS: [usize; UCS2BE_CLS_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, // 00 - 07
    0, 0, 1, 0, 0, 2, 0, 0, // 08 - 0f
    0, 0, 0, 0, 0, 0, 0, 0, // 10 - 17
    0, 0, 0, 3, 0, 0, 0, 0, // 18 - 1f
    0, 0, 0, 0, 0, 0, 0, 0, // 20 - 27
    0, 3, 3, 3, 3, 3, 0, 0, // 28 - 2f
    0, 0, 0, 0, 0, 0, 0, 0, // 30 - 37
    0, 0, 0, 0, 0, 0, 0, 0, // 38 - 3f
    0, 0, 0, 0, 0, 0, 0, 0, // 40 - 47
    0, 0, 0, 0, 0, 0, 0, 0, // 48 - 4f
    0, 0, 0, 0, 0, 0, 0, 0, // 50 - 57
    0, 0, 0, 0, 0, 0, 0, 0, // 58 - 5f
    0, 0, 0, 0, 0, 0, 0, 0, // 60 - 67
    0, 0, 0, 0, 0, 0, 0, 0, // 68 - 6f
    0, 0, 0, 0, 0, 0, 0, 0, // 70 - 77
    0, 0, 0, 0, 0, 0, 0, 0, // 78 - 7f
    0, 0, 0, 0, 0, 0, 0, 0, // 80 - 87
    0, 0, 0, 0, 0, 0, 0, 0, // 88 - 8f
    0, 0, 0, 0, 0, 0, 0, 0, // 90 - 97
    0, 0, 0, 0, 0, 0, 0, 0, // 98 - 9f
    0, 0, 0, 0, 0, 0, 0, 0, // a0 - a7
    0, 0, 0, 0, 0, 0, 0, 0, // a8 - af
    0, 0, 0, 0, 0, 0, 0, 0, // b0 - b7
    0, 0, 0, 0, 0, 0, 0, 0, // b8 - bf
    0, 0, 0, 0, 0, 0, 0, 0, // c0 - c7
    0, 0, 0, 0, 0, 0, 0, 0, // c8 - cf
    0, 0, 0, 0, 0, 0, 0, 0, // d0 - d7
    0, 0, 0, 0, 0, 0, 0, 0, // d8 - df
    0, 0, 0, 0, 0, 0, 0, 0, // e0 - e7
    0, 0, 0, 0, 0, 0, 0, 0, // e8 - ef
    0, 0, 0, 0, 0, 0, 0, 0, // f0 - f7
    0, 0, 0, 0, 0, 0, 4, 5, // f8 - ff
];

pub const UCS2BE_ST_SIZE: usize = 56;

#[rustfmt::skip]
pub const UCS2BE_ST: [enums::MachineState; UCS2BE_ST_SIZE] = [
    enums::MachineState::State6, enums::MachineState::State6, enums::MachineState::State7, enums::MachineState::State6, // 00-03
    enums::MachineState::State4, enums::MachineState::State3,  enums::MachineState::Error,  enums::MachineState::Error, // 04-07
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 08-0b
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 0c-0f
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe, enums::MachineState::State5, enums::MachineState::State5, // 10-13
    enums::MachineState::State5,  enums::MachineState::Error,  enums::MachineState::ItsMe,  enums::MachineState::Error, // 14-17
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 18-1b
    enums::MachineState::State5,  enums::MachineState::Error, enums::MachineState::State6, enums::MachineState::State6, // 1c-1f
    enums::MachineState::State7, enums::MachineState::State6, enums::MachineState::State8, enums::MachineState::State8, // 20-23
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 24-27
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 28-2b
     enums::MachineState::Error,  enums::MachineState::Error, enums::MachineState::State5, enums::MachineState::State5, // 2c-2f
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 30-33
    enums::MachineState::State5,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 34-37
];

pub const UCS2BE_CHAR_LEN_TABLE_SIZE: usize = 6;

pub const UCS2BE_CHAR_LEN_TABLE: [usize; UCS2BE_CHAR_LEN_TABLE_SIZE] = [2, 2, 2, 0, 2, 2];

pub struct Ucs2beSmModel {
    pub class_table: [usize; UCS2BE_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; UCS2BE_ST_SIZE],
    pub char_len_table: [usize; UCS2BE_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for Ucs2beSmModel {
    fn default() -> Self {
        Self {
            class_table: UCS2BE_CLS,
            class_factor: UCS2BE_CHAR_LEN_TABLE_SIZE,
            state_table: UCS2BE_ST,
            char_len_table: UCS2BE_CHAR_LEN_TABLE,
            name: "UTF-16BE".into(),
        }
    }
}

pub const UCS2LE_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// UCS2-LE
pub const UCS2LE_CLS: [usize; UCS2LE_CLS_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0, // 00 - 07
    0, 0, 1, 0, 0, 2, 0, 0, // 08 - 0f
    0, 0, 0, 0, 0, 0, 0, 0, // 10 - 17
    0, 0, 0, 3, 0, 0, 0, 0, // 18 - 1f
    0, 0, 0, 0, 0, 0, 0, 0, // 20 - 27
    0, 3, 3, 3, 3, 3, 0, 0, // 28 - 2f
    0, 0, 0, 0, 0, 0, 0, 0, // 30 - 37
    0, 0, 0, 0, 0, 0, 0, 0, // 38 - 3f
    0, 0, 0, 0, 0, 0, 0, 0, // 40 - 47
    0, 0, 0, 0, 0, 0, 0, 0, // 48 - 4f
    0, 0, 0, 0, 0, 0, 0, 0, // 50 - 57
    0, 0, 0, 0, 0, 0, 0, 0, // 58 - 5f
    0, 0, 0, 0, 0, 0, 0, 0, // 60 - 67
    0, 0, 0, 0, 0, 0, 0, 0, // 68 - 6f
    0, 0, 0, 0, 0, 0, 0, 0, // 70 - 77
    0, 0, 0, 0, 0, 0, 0, 0, // 78 - 7f
    0, 0, 0, 0, 0, 0, 0, 0, // 80 - 87
    0, 0, 0, 0, 0, 0, 0, 0, // 88 - 8f
    0, 0, 0, 0, 0, 0, 0, 0, // 90 - 97
    0, 0, 0, 0, 0, 0, 0, 0, // 98 - 9f
    0, 0, 0, 0, 0, 0, 0, 0, // a0 - a7
    0, 0, 0, 0, 0, 0, 0, 0, // a8 - af
    0, 0, 0, 0, 0, 0, 0, 0, // b0 - b7
    0, 0, 0, 0, 0, 0, 0, 0, // b8 - bf
    0, 0, 0, 0, 0, 0, 0, 0, // c0 - c7
    0, 0, 0, 0, 0, 0, 0, 0, // c8 - cf
    0, 0, 0, 0, 0, 0, 0, 0, // d0 - d7
    0, 0, 0, 0, 0, 0, 0, 0, // d8 - df
    0, 0, 0, 0, 0, 0, 0, 0, // e0 - e7
    0, 0, 0, 0, 0, 0, 0, 0, // e8 - ef
    0, 0, 0, 0, 0, 0, 0, 0, // f0 - f7
    0, 0, 0, 0, 0, 0, 4, 5, // f8 - ff
];

pub const UCS2LE_ST_SIZE: usize = 56;

#[rustfmt::skip]
pub const UCS2LE_ST: [enums::MachineState; UCS2LE_ST_SIZE] = [
    enums::MachineState::State6, enums::MachineState::State6, enums::MachineState::State7, enums::MachineState::State6, // 00-03
    enums::MachineState::State4, enums::MachineState::State3,  enums::MachineState::Error,  enums::MachineState::Error, // 04-07
     enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error,  enums::MachineState::Error, // 08-0b
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe,  enums::MachineState::ItsMe, // 0c-0f
     enums::MachineState::ItsMe,  enums::MachineState::ItsMe, enums::MachineState::State5, enums::MachineState::State5, // 10-13
    enums::MachineState::State5,  enums::MachineState::Error,  enums::MachineState::ItsMe,  enums::MachineState::Error, // 14-17
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 18-1b
    enums::MachineState::State5,  enums::MachineState::Error, enums::MachineState::State6, enums::MachineState::State6, // 1c-1f
    enums::MachineState::State7, enums::MachineState::State6, enums::MachineState::State8, enums::MachineState::State8, // 20-23
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 24-27
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 28-2b
     enums::MachineState::Error,  enums::MachineState::Error, enums::MachineState::State5, enums::MachineState::State5, // 2c-2f
    enums::MachineState::State5, enums::MachineState::State5, enums::MachineState::State5,  enums::MachineState::Error, // 30-33
    enums::MachineState::State5,  enums::MachineState::Error,  enums::MachineState::Start,  enums::MachineState::Start, // 34-37
];

pub const UCS2LE_CHAR_LEN_TABLE_SIZE: usize = 6;

pub const UCS2LE_CHAR_LEN_TABLE: [usize; UCS2LE_CHAR_LEN_TABLE_SIZE] = [2, 2, 2, 2, 2, 2];

pub struct Ucs2leSmModel {
    pub class_table: [usize; UCS2LE_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; UCS2LE_ST_SIZE],
    pub char_len_table: [usize; UCS2LE_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for Ucs2leSmModel {
    fn default() -> Self {
        Self {
            class_table: UCS2LE_CLS,
            class_factor: UCS2LE_CHAR_LEN_TABLE_SIZE,
            state_table: UCS2LE_ST,
            char_len_table: UCS2LE_CHAR_LEN_TABLE,
            name: "UTF-16LE".into(),
        }
    }
}

pub const UTF8_CLS_SIZE: usize = 256;

#[rustfmt::skip]
/// UTF-8
pub const UTF8_CLS: [usize; UTF8_CLS_SIZE] = [
     1,  1,  1,  1,  1,  1,  1,  1, // 00 - 07    // разрешить 0x00 в качестве законного значения
     1,  1,  1,  1,  1,  1,  0,  0, // 08 - 0f
     1,  1,  1,  1,  1,  1,  1,  1, // 10 - 17
     1,  1,  1,  0,  1,  1,  1,  1, // 18 - 1f
     1,  1,  1,  1,  1,  1,  1,  1, // 20 - 27
     1,  1,  1,  1,  1,  1,  1,  1, // 28 - 2f
     1,  1,  1,  1,  1,  1,  1,  1, // 30 - 37
     1,  1,  1,  1,  1,  1,  1,  1, // 38 - 3f
     1,  1,  1,  1,  1,  1,  1,  1, // 40 - 47
     1,  1,  1,  1,  1,  1,  1,  1, // 48 - 4f
     1,  1,  1,  1,  1,  1,  1,  1, // 50 - 57
     1,  1,  1,  1,  1,  1,  1,  1, // 58 - 5f
     1,  1,  1,  1,  1,  1,  1,  1, // 60 - 67
     1,  1,  1,  1,  1,  1,  1,  1, // 68 - 6f
     1,  1,  1,  1,  1,  1,  1,  1, // 70 - 77
     1,  1,  1,  1,  1,  1,  1,  1, // 78 - 7f
     2,  2,  2,  2,  3,  3,  3,  3, // 80 - 87
     4,  4,  4,  4,  4,  4,  4,  4, // 88 - 8f
     4,  4,  4,  4,  4,  4,  4,  4, // 90 - 97
     4,  4,  4,  4,  4,  4,  4,  4, // 98 - 9f
     5,  5,  5,  5,  5,  5,  5,  5, // a0 - a7
     5,  5,  5,  5,  5,  5,  5,  5, // a8 - af
     5,  5,  5,  5,  5,  5,  5,  5, // b0 - b7
     5,  5,  5,  5,  5,  5,  5,  5, // b8 - bf
     0,  0,  6,  6,  6,  6,  6,  6, // c0 - c7
     6,  6,  6,  6,  6,  6,  6,  6, // c8 - cf
     6,  6,  6,  6,  6,  6,  6,  6, // d0 - d7
     6,  6,  6,  6,  6,  6,  6,  6, // d8 - df
     7,  8,  8,  8,  8,  8,  8,  8, // e0 - e7
     8,  8,  8,  8,  8,  9,  8,  8, // e8 - ef
    10, 11, 11, 11, 11, 11, 11, 11, // f0 - f7
    12, 13, 13, 13, 14, 15,  0,  0, // f8 - ff
];

pub const UTF8_ST_SIZE: usize = 208;

#[rustfmt::skip]
pub const UTF8_ST: [enums::MachineState; UTF8_ST_SIZE] = [
      enums::MachineState::Error,   enums::MachineState::Start,   enums::MachineState::Error,   enums::MachineState::Error, // 00-03
      enums::MachineState::Error,   enums::MachineState::Error, enums::MachineState::State12, enums::MachineState::State10, // 04-07
     enums::MachineState::State9, enums::MachineState::State11,  enums::MachineState::State8,  enums::MachineState::State7, // 08-0b
     enums::MachineState::State6,  enums::MachineState::State5,  enums::MachineState::State4,  enums::MachineState::State3, // 0c-0f
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 10-13
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 14-17
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 18-1b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 1c-1f
      enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe, // 20-23
      enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe, // 24-27
      enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe, // 28-2b
      enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe,   enums::MachineState::ItsMe, // 2c-2f
      enums::MachineState::Error,   enums::MachineState::Error,  enums::MachineState::State5,  enums::MachineState::State5, // 30-33
     enums::MachineState::State5,  enums::MachineState::State5,   enums::MachineState::Error,   enums::MachineState::Error, // 34-37
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 38-3b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 3c-3f
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,  enums::MachineState::State5, // 40-43
     enums::MachineState::State5,  enums::MachineState::State5,   enums::MachineState::Error,   enums::MachineState::Error, // 44-47
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 48-4b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 4c-4f
      enums::MachineState::Error,   enums::MachineState::Error,  enums::MachineState::State7,  enums::MachineState::State7, // 50-53
     enums::MachineState::State7,  enums::MachineState::State7,   enums::MachineState::Error,   enums::MachineState::Error, // 54-57
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 58-5b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 5c-5f
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 60-63
     enums::MachineState::State7,  enums::MachineState::State7,   enums::MachineState::Error,   enums::MachineState::Error, // 64-67
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 68-6b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 6c-6f
      enums::MachineState::Error,   enums::MachineState::Error,  enums::MachineState::State9,  enums::MachineState::State9, // 70-73
     enums::MachineState::State9,  enums::MachineState::State9,   enums::MachineState::Error,   enums::MachineState::Error, // 74-77
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 78-7b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 7c-7f
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 80-83
      enums::MachineState::Error,  enums::MachineState::State9,   enums::MachineState::Error,   enums::MachineState::Error, // 84-87
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 88-8b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 8c-8f
      enums::MachineState::Error,   enums::MachineState::Error, enums::MachineState::State12, enums::MachineState::State12, // 90-93
    enums::MachineState::State12, enums::MachineState::State12,   enums::MachineState::Error,   enums::MachineState::Error, // 94-97
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 98-9b
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // 9c-9f
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // a0-a3
      enums::MachineState::Error, enums::MachineState::State12,   enums::MachineState::Error,   enums::MachineState::Error, // a4-a7
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // a8-ab
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // ac-af
      enums::MachineState::Error,   enums::MachineState::Error, enums::MachineState::State12, enums::MachineState::State12, // b0-b3
    enums::MachineState::State12,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // b4-b7
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // b8-bb
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // bc-bf
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Start,   enums::MachineState::Start, // c0-c3
      enums::MachineState::Start,   enums::MachineState::Start,   enums::MachineState::Error,   enums::MachineState::Error, // c4-c7
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // c8-cb
      enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error,   enums::MachineState::Error, // cc-cf
];

pub const UTF8_CHAR_LEN_TABLE_SIZE: usize = 16;

pub const UTF8_CHAR_LEN_TABLE: [usize; UTF8_CHAR_LEN_TABLE_SIZE] =
    [0, 1, 0, 0, 0, 0, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6];

pub struct Utf8SmModel {
    pub class_table: [usize; UTF8_CLS_SIZE],
    pub class_factor: usize,
    pub state_table: [enums::MachineState; UTF8_ST_SIZE],
    pub char_len_table: [usize; UTF8_CHAR_LEN_TABLE_SIZE],
    pub name: String,
}

impl Default for Utf8SmModel {
    fn default() -> Self {
        Self {
            class_table: UTF8_CLS,
            class_factor: UTF8_CHAR_LEN_TABLE_SIZE,
            state_table: UTF8_ST,
            char_len_table: UTF8_CHAR_LEN_TABLE,
            name: "UTF-8".into(),
        }
    }
}

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

    #[test]
    fn test_cp949_cls_len() {
        assert_eq!(CP949_CLS.len(), CP949_CLS_SIZE);
    }

    #[test]
    fn test_cp949_st_len() {
        assert_eq!(CP949_ST.len(), CP949_ST_SIZE);
    }

    #[test]
    fn test_cp949_char_len_table_len() {
        assert_eq!(CP949_CHAR_LEN_TABLE.len(), CP949_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_eucjp_cls_len() {
        assert_eq!(EUCJP_CLS.len(), EUCJP_CLS_SIZE);
    }

    #[test]
    fn test_eucjp_st_len() {
        assert_eq!(EUCJP_ST.len(), EUCJP_ST_SIZE);
    }

    #[test]
    fn test_eucjp_char_len_table_len() {
        assert_eq!(EUCJP_CHAR_LEN_TABLE.len(), EUCJP_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_euckr_cls_len() {
        assert_eq!(EUCKR_CLS.len(), EUCKR_CLS_SIZE);
    }

    #[test]
    fn test_euckr_st_len() {
        assert_eq!(EUCKR_ST.len(), EUCKR_ST_SIZE);
    }

    #[test]
    fn test_euckr_char_len_table_len() {
        assert_eq!(EUCKR_CHAR_LEN_TABLE.len(), EUCKR_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_johab_cls_len() {
        assert_eq!(JOHAB_CLS.len(), JOHAB_CLS_SIZE);
    }

    #[test]
    fn test_johab_st_len() {
        assert_eq!(JOHAB_ST.len(), JOHAB_ST_SIZE);
    }

    #[test]
    fn test_johab_char_len_table_len() {
        assert_eq!(JOHAB_CHAR_LEN_TABLE.len(), JOHAB_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_euctw_cls_len() {
        assert_eq!(EUCTW_CLS.len(), EUCTW_CLS_SIZE);
    }

    #[test]
    fn test_euctw_st_len() {
        assert_eq!(EUCTW_ST.len(), EUCTW_ST_SIZE);
    }

    #[test]
    fn test_euctw_char_len_table_len() {
        assert_eq!(EUCTW_CHAR_LEN_TABLE.len(), EUCTW_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_gb2312_cls_len() {
        assert_eq!(GB2312_CLS.len(), GB2312_CLS_SIZE);
    }

    #[test]
    fn test_gb2312_st_len() {
        assert_eq!(GB2312_ST.len(), GB2312_ST_SIZE);
    }

    #[test]
    fn test_gb2312_char_len_table_len() {
        assert_eq!(GB2312_CHAR_LEN_TABLE.len(), GB2312_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_sjis_cls_len() {
        assert_eq!(SJIS_CLS.len(), SJIS_CLS_SIZE);
    }

    #[test]
    fn test_sjis_st_len() {
        assert_eq!(SJIS_ST.len(), SJIS_ST_SIZE);
    }

    #[test]
    fn test_sjis_char_len_table_len() {
        assert_eq!(SJIS_CHAR_LEN_TABLE.len(), SJIS_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_ucs2be_cls_len() {
        assert_eq!(UCS2BE_CLS.len(), UCS2BE_CLS_SIZE);
    }

    #[test]
    fn test_ucs2be_st_len() {
        assert_eq!(UCS2BE_ST.len(), UCS2BE_ST_SIZE);
    }

    #[test]
    fn test_ucs2be_char_len_table_len() {
        assert_eq!(UCS2BE_CHAR_LEN_TABLE.len(), UCS2BE_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_ucs2le_cls_len() {
        assert_eq!(UCS2LE_CLS.len(), UCS2LE_CLS_SIZE);
    }

    #[test]
    fn test_ucs2le_st_len() {
        assert_eq!(UCS2LE_ST.len(), UCS2LE_ST_SIZE);
    }

    #[test]
    fn test_ucs2le_char_len_table_len() {
        assert_eq!(UCS2LE_CHAR_LEN_TABLE.len(), UCS2LE_CHAR_LEN_TABLE_SIZE);
    }

    #[test]
    fn test_utf8_cls_len() {
        assert_eq!(UTF8_CLS.len(), UTF8_CLS_SIZE);
    }

    #[test]
    fn test_utf8_st_len() {
        assert_eq!(UTF8_ST.len(), UTF8_ST_SIZE);
    }

    #[test]
    fn test_utf8_char_len_table_len() {
        assert_eq!(UTF8_CHAR_LEN_TABLE.len(), UTF8_CHAR_LEN_TABLE_SIZE);
    }
}
