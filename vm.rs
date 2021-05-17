// Opcode enumeration
#[allow(dead_code)]
pub enum Instr
{
    // Local variable access
    GetLocal { idx: u32 },
    SetLocal { idx: u32 },

    // Stack manipulation
    Push,
    Pop,
    Dup,
    Swap,

    /*
    // 32-bit integer operations
    I32_ADD,
    I32_SUB,
    I32_MUL,
    I32_DIV,
    I32_MOD,
    I32_SHL,
    I32_SHR,
    I32_USHR,
    I32_AND,
    I32_OR,
    I32_XOR,
    I32_NOT,
    I32_LT,
    I32_LE,
    I32_GT,
    I32_GE,
    I32_EQ,
    I32_INC,
    I32_DEC,

    // Conversion operations
    I32_TO_F32,
    I32_TO_STR,
    F32_TO_I32,
    F32_TO_STR,
    STR_TO_F32,

    // Miscellaneous
    EQ,
    HAS_TAG,
    GET_TAG,
    LOCAL_HAS_TAG,

    // String operations
    STR_LEN,
    STR_GET_CH_STR,
    STR_GET_CH_I32
    STR_CAT,

    // Object operations
    NEW_OBJECT,
    OBJ_FIELD_LIST
    OBJ_HAS,
    OBJ_SET,
    OBJ_GET,
    OBJ_GET_IMM,

    // Array operations
    NEW_ARRAY,
    ARRAY_LEN,
    ARRAY_PUSH,
    ARRAY_POP,
    ARRAY_GET,
    ARRAY_SET,

    // Branch instructions
    JUMP,
    IF_TRUE,
    IF_FALSE,
    CALL,
    RET,
    THROW
    */
}

// TODO: make instructions be fixed-width structs?