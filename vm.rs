/*
// Objects are aligned to 16-byte (2x64-bit) boundaries,
// which gives us 4 bits to spare for type tags.

TODO: need some kind of Value struct/type
- 32-bit payload for integers?
- Way to tag and untag pointers, integers, etc.

Basic types, kinds of values:
- Nil (should probably be all zeros, evaluate to false)
- True/false (special pointers?)
- Int32
- Float32
- Object
- Function
- Array
- Resource handle
*/

// Opcode enumeration
pub enum Instr
{
    // Local variable access
    GetLocal { idx: u32 },
    SetLocal { idx: u32 },

    // Stack manipulation
    Push { val: u64 },
    Pop,
    Dup,
    Swap,

    // 32-bit integer operations
    I32Add,
    I32Sub,
    I32Mul,
    /*
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
    OBJ_KEYS,
    OBJ_DEF_CONST,
    OBJ_SET,
    OBJ_GET,
    OBJ_GET_IMM,
    OBJ_HAS,

    // Array operations
    NEW_ARRAY,
    ARRAY_LEN,
    ARRAY_PUSH,
    ARRAY_POP,
    ARRAY_GET,
    ARRAY_SET,
    */

    // Branch instructions
    Jump { target: u32 },
    IfTrue { target: u32 },
    IfFalse { target: u32 },
    Call,
    Ret,
    //Throw,
}
