#![allow(non_upper_case_globals, missing_docs)]

/// Two-operand opcodes.
pub mod op2 {
    pub const je: u8 = 0x1;
    pub const jl: u8 = 0x2;
    pub const jg: u8 = 0x3;
    pub const dec_chk: u8 = 0x4;
    pub const inc_chk: u8 = 0x5;
    pub const jin: u8 = 0x6;
    pub const test: u8 = 0x7;
    pub const or: u8 = 0x8;
    pub const and: u8 = 0x9;
    pub const test_attr: u8 = 0xA;
    pub const set_attr: u8 = 0xB;
    pub const clear_attr: u8 = 0xC;
    pub const store: u8 = 0xD;
    pub const insert_obj: u8 = 0xE;
    pub const loadw: u8 = 0xF;
    pub const loadb: u8 = 0x10;
    pub const get_prop: u8 = 0x11;
    pub const get_prop_addr: u8 = 0x12;
    pub const get_next_prop: u8 = 0x13;
    pub const add: u8 = 0x14;
    pub const sub: u8 = 0x15;
    pub const mul: u8 = 0x16;
    pub const div: u8 = 0x17;
    pub const _mod: u8 = 0x18;
    pub const call_2s: u8 = 0x19;
    pub const call_2n: u8 = 0x1A;
    pub const set_color: u8 = 0x1B;
    pub const throw: u8 = 0x1C;
}

/// One-operand opcodes.
pub mod op1 {
    pub const jz: u8 = 0x0;
    pub const get_sibling: u8 = 0x1;
    pub const get_child: u8 = 0x2;
    pub const get_parent: u8 = 0x3;
    pub const get_prop_len: u8 = 0x4;
    pub const inc: u8 = 0x5;
    pub const dec: u8 = 0x6;
    pub const print_addr: u8 = 0x7;
    pub const call_1s: u8 = 0x8;
    pub const remove_obj: u8 = 0x9;
    pub const print_obj: u8 = 0xA;
    pub const ret: u8 = 0xB;
    pub const jump: u8 = 0xC;
    pub const print_paddr: u8 = 0xD;
    pub const load: u8 = 0xE;
    pub const not: u8 = 0xF;
    pub const call_1n: u8 = 0xF;
}

/// Zero-operand opcodes.
pub mod op0 {
    pub const rtrue: u8 = 0x0;
    pub const rfalse: u8 = 0x1;
    pub const print: u8 = 0x2;
    pub const print_ret: u8 = 0x3;
    pub const nop: u8 = 0x4;
    pub const save: u8 = 0x5;
    pub const restore: u8 = 0x6;
    pub const restart: u8 = 0x7;
    pub const ret_popped: u8 = 0x8;
    pub const pop: u8 = 0x9;
    pub const catch: u8 = 0x9;
    pub const quit: u8 = 0xA;
    pub const new_line: u8 = 0xB;
    pub const show_status: u8 = 0xC;
    pub const verify: u8 = 0xD;
    pub const extended: u8 = 0xE;
    pub const piracy: u8 = 0xF;
}

/// Variable-operand opcodes.
pub mod var {
    pub const call: u8 = 0x0;
    pub const call_vs: u8 = 0x0;
    pub const storew: u8 = 0x1;
    pub const storeb: u8 = 0x2;
    pub const put_prop: u8 = 0x3;
    pub const sread: u8 = 0x4;
    pub const aread: u8 = 0x4;
    pub const print_char: u8 = 0x5;
    pub const print_num: u8 = 0x6;
    pub const random: u8 = 0x7;
    pub const push: u8 = 0x8;
    pub const pull: u8 = 0x9;
    pub const split_window: u8 = 0xA;
    pub const set_window: u8 = 0xB;
    pub const call_vs2: u8 = 0xC;
    pub const erase_window: u8 = 0xD;
    pub const erase_line: u8 = 0xE;
    pub const set_cursor: u8 = 0xF;
    pub const get_cursor: u8 = 0x10;
    pub const set_text_style: u8 = 0x11;
    pub const buffer_mode: u8 = 0x12;
    pub const output_stream: u8 = 0x13;
    pub const input_stream: u8 = 0x14;
    pub const sound_effect: u8 = 0x15;
    pub const read_char: u8 = 0x16;
    pub const scan_table: u8 = 0x17;
    pub const not: u8 = 0x18;
    pub const call_vn: u8 = 0x19;
    pub const call_vn2: u8 = 0x1A;
    pub const tokenize: u8 = 0x1B;
    pub const encode_text: u8 = 0x1C;
    pub const copy_table: u8 = 0x1D;
    pub const print_table: u8 = 0x1E;
    pub const check_arg_count: u8 = 0x1F;
}

/// Extended opcodes.
pub mod ext {
    pub const save: u8 = 0x0;
    pub const restore: u8 = 0x1;
    pub const log_shift: u8 = 0x2;
    pub const art_shift: u8 = 0x3;
    pub const set_font: u8 = 0x4;
    pub const draw_picture: u8 = 0x5;
    pub const picture_data: u8 = 0x6;
    pub const erase_picture: u8 = 0x7;
    pub const set_margins: u8 = 0x8;
    pub const save_undo: u8 = 0x9;
    pub const restore_undo: u8 = 0xA;
    pub const print_unicode: u8 = 0xB;
    pub const check_unicode: u8 = 0xC;
    pub const set_true_color: u8 = 0xD;
    pub const move_window: u8 = 0x10;
    pub const window_size: u8 = 0x11;
    pub const window_style: u8 = 0x12;
    pub const get_wind_prop: u8 = 0x13;
    pub const scroll_window: u8 = 0x14;
    pub const pop_stack: u8 = 0x15;
    pub const read_mouse: u8 = 0x16;
    pub const mouse_window: u8 = 0x17;
    pub const push_stack: u8 = 0x18;
    pub const put_wind_prop: u8 = 0x19;
    pub const print_form: u8 = 0x1A;
    pub const make_menu: u8 = 0x1B;
    pub const picture_table: u8 = 0x1C;
    pub const buffer_screen: u8 = 0x1D;
}
