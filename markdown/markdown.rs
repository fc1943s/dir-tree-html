#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_MdNone,
    US0_MdParagraph,
    US0_MdList,
    US0_MdOrdered,
    US0_MdCode,
    US0_MdTable(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_AlignNone,
    US1_AlignLeft,
    US1_AlignCenter,
    US1_AlignRight,
}
fn close_block_1(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_MdCode => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_MdList => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n</ul>\n"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_MdNone => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_MdOrdered => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n</ol>\n"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_MdParagraph => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_MdTable(v6) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tbody>\n</table>\n"); } LIT.with(|lit| lit.clone()) };
            v7
        }
    }
}
fn find_nl_2(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'\n';
            if v5 {
                return v1;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            }
        }
    }
}
fn starts_at_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v2 == v3;
        if v4 {
            return true;
        } else {
            let v5: i32 = (v0.clone().len() as i32);
            let v6: bool = v2 == v5;
            if v6 {
                return false;
            } else {
                let v7: u8 = v0.clone().as_bytes()[v2 as usize];
                let v8: u8 = v1.clone().as_bytes()[v2 as usize];
                let v9: bool = v7 == v8;
                if v9 {
                    let v10: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v1.clone(), v10);
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn lead_spaces_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b' ';
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn table_row_4(mut v0: Rc<str>) -> bool {
    let v1: i32 = 0i32;
    let v2: i32 = lead_spaces_5(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v2 == v3;
    if v4 {
        false
    } else {
        let v5: u8 = v0.as_bytes()[v2 as usize];
        let v6: bool = v5 == b'|';
        v6
    }
}
fn trail_end_9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 == 0i32;
        if v2 {
            return 0i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            if v5 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn trimmed_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = lead_spaces_5(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = trail_end_9(v0.clone(), v3);
    let v5: bool = v2 == v4;
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let v7: i32 = v4.wrapping_sub(1i32);
        let v8: Rc<str> = string_slice(&v0, v2 as i64, v7 as i64);
        v8
    }
}
fn row_body_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = trimmed_8(v0);
    let v2: i32 = (v1.clone().len() as i32);
    let v3: bool = 1i32 == v2;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: i32 = v2.wrapping_sub(1i32);
        let v6: Rc<str> = string_slice(&v1, 1i32 as i64, v5 as i64);
        v6
    }
}
fn cell_end_11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'\\';
            let v8: bool = if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                let v7: bool = v6 < v2;
                v7
            } else {
                false
            };
            if v8 {
                let v9: i32 = v1.wrapping_add(2i32);
                (v0, v1) = (v0.clone(), v9);
                continue;
            } else {
                let v11: bool = v4 == b'|';
                if v11 {
                    return v1;
                } else {
                    let v12: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v12);
                    continue;
                }
            }
        }
    }
}
fn nth_cell_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let v6: i32 = cell_end_11(v0.clone(), v1);
            let v7: bool = v2 == 0i32;
            if v7 {
                let v8: bool = v1 == v6;
                let v12: Rc<str> = if v8 {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                } else {
                    let v10: i32 = v6.wrapping_sub(1i32);
                    let v11: Rc<str> = string_slice(&v0.clone(), v1 as i64, v10 as i64);
                    v11.clone()
                };
                return trimmed_8(v12.clone());
            } else {
                let v14: i32 = v6.wrapping_add(1i32);
                let v15: i32 = v2.wrapping_sub(1i32);
                (v0, v1, v2) = (v0.clone(), v14, v15);
                continue;
            }
        }
    }
}
fn cell_align_of_13(mut v0: Rc<str>) -> US1 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 == 0i32;
    if v2 {
        US1::US1_AlignNone
    } else {
        let v4: u8 = v0.clone().as_bytes()[0i32 as usize];
        let v5: bool = v4 == b':';
        let v6: i32 = v1.wrapping_sub(1i32);
        let v7: u8 = v0.as_bytes()[v6 as usize];
        let v8: bool = v7 == b':';
        let v9: bool = v5 && v8;
        if v9 {
            US1::US1_AlignCenter
        } else {
            if v8 {
                US1::US1_AlignRight
            } else {
                if v5 {
                    US1::US1_AlignLeft
                } else {
                    US1::US1_AlignNone
                }
            }
        }
    }
}
fn align_attr_14(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_AlignCenter => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: center\""); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_AlignLeft => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: left\""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_AlignNone => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_AlignRight => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: right\""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn row_cells_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> Rc<str> {
    loop {
        let v6: i32 = (v0.clone().len() as i32);
        let v7: bool = v3 >= v6;
        if v7 {
            return v5.clone();
        } else {
            let v8: i32 = cell_end_11(v0.clone(), v3);
            let v9: bool = v3 == v8;
            let v13: Rc<str> = if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            } else {
                let v11: i32 = v8.wrapping_sub(1i32);
                let v12: Rc<str> = string_slice(&v0.clone(), v3 as i64, v11 as i64);
                v12.clone()
            };
            let v14: Rc<str> = trimmed_8(v13.clone());
            let v15: bool = v8 == v6;
            let v17: bool = if v15 {
                let v16: bool = &*v14 == "";
                v16
            } else {
                false
            };
            if v17 {
                return v5.clone();
            } else {
                let v18: i32 = 0i32;
                let v19: Rc<str> = nth_cell_12(v1.clone(), v18, v4);
                let v20: US1 = cell_align_of_13(v19.clone());
                let v21: Rc<str> = align_attr_14(v20.clone());
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v2)), v21));
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
                let v27: Rc<str> = dtl_inline::dtl_render_inline(&*v14);
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v29)), v2)), v25));
                let v33: i32 = v8.wrapping_add(1i32);
                let v34: i32 = v4.wrapping_add(1i32);
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v32));
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2.clone(), v33, v34, v35.clone());
                continue;
            }
        }
    }
}
fn table_tr_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = row_body_7(v0);
    let v4: Rc<str> = row_body_7(v1);
    let v5: i32 = 0i32;
    let v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = row_cells_10(v3, v4, v2, v5, v6, v7);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<tr>"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v8));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tr>\n"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    v12
}
fn count_hashes_16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'#';
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn heading_level_15(mut v0: Rc<str>) -> i32 {
    let v1: i32 = 0i32;
    let v2: i32 = count_hashes_16(v0.clone(), v1);
    let v3: bool = v2 == 0i32;
    if v3 {
        0i32
    } else {
        let v4: bool = v2 > 6i32;
        if v4 {
            0i32
        } else {
            let v5: i32 = (v0.clone().len() as i32);
            let v6: bool = v2 == v5;
            if v6 {
                0i32
            } else {
                let v7: u8 = v0.as_bytes()[v2 as usize];
                let v8: bool = v7 == b' ';
                if v8 {
                    v2
                } else {
                    0i32
                }
            }
        }
    }
}
fn blank_from_17(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return true;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b' ';
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn line_at_18(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v1 > v2;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: i32 = find_nl_2(v0.clone(), v1);
        let v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = string_slice(&v0, v1 as i64, v6 as i64);
        v7
    }
}
fn delimiter_chars_20(mut v0: Rc<str>, mut v1: i32, mut v2: bool) -> bool {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 == v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'-';
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                let v8: bool = true;
                (v0, v1, v2) = (v0.clone(), v7, v8);
                continue;
            } else {
                let v10: bool = v5 == b'|';
                let v12: bool = if v10 {
                    true
                } else {
                    let v11: bool = v5 == b':';
                    v11
                };
                let v14: bool = if v12 {
                    true
                } else {
                    let v13: bool = v5 == b' ';
                    v13
                };
                if v14 {
                    let v15: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v15, v2);
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn delimiter_row_19(mut v0: Rc<str>) -> bool {
    let v1: bool = table_row_4(v0.clone());
    if v1 {
        let v2: i32 = 0i32;
        let v3: bool = false;
        delimiter_chars_20(v0, v2, v3)
    } else {
        false
    }
}
fn bullet_start_21(mut v0: Rc<str>) -> i32 {
    let v1: i32 = 0i32;
    let v2: i32 = lead_spaces_5(v0.clone(), v1);
    let v3: i32 = v2.wrapping_add(2i32);
    let v4: i32 = (v0.clone().len() as i32);
    let v5: bool = v3 > v4;
    if v5 {
        -1i32
    } else {
        let v6: i32 = v2.wrapping_add(1i32);
        let v7: u8 = v0.clone().as_bytes()[v6 as usize];
        let v8: bool = v7 == b' ';
        if v8 {
            let v9: u8 = v0.as_bytes()[v2 as usize];
            let v10: bool = v9 == b'-';
            if v10 {
                v3
            } else {
                let v11: bool = v9 == b'*';
                if v11 {
                    v3
                } else {
                    -1i32
                }
            }
        } else {
            -1i32
        }
    }
}
fn digits_end_23(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 >= b'0';
            let v7: bool = if v5 {
                let v6: bool = v4 <= b'9';
                v6
            } else {
                false
            };
            if v7 {
                let v8: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v8);
                continue;
            } else {
                return v1;
            }
        }
    }
}
fn ordered_start_22(mut v0: Rc<str>) -> i32 {
    let v1: i32 = 0i32;
    let v2: i32 = lead_spaces_5(v0.clone(), v1);
    let v3: i32 = digits_end_23(v0.clone(), v2);
    let v4: bool = v3 == v2;
    if v4 {
        -1i32
    } else {
        let v5: i32 = v3.wrapping_add(2i32);
        let v6: i32 = (v0.clone().len() as i32);
        let v7: bool = v5 > v6;
        if v7 {
            -1i32
        } else {
            let v8: u8 = v0.clone().as_bytes()[v3 as usize];
            let v9: bool = v8 == b'.';
            let v13: bool = if v9 {
                let v10: i32 = v3.wrapping_add(1i32);
                let v11: u8 = v0.as_bytes()[v10 as usize];
                let v12: bool = v11 == b' ';
                v12
            } else {
                false
            };
            if v13 {
                v5
            } else {
                -1i32
            }
        }
    }
}
fn ordered_open_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = lead_spaces_5(v0.clone(), v1);
    let v3: i32 = digits_end_23(v0.clone(), v2);
    let v4: bool = v2 == v3;
    let v8: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: i32 = v3.wrapping_sub(1i32);
        let v7: Rc<str> = string_slice(&v0, v2 as i64, v6 as i64);
        v7
    };
    let v9: bool = &*v8 == "1";
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ol>\n"); } LIT.with(|lit| lit.clone()) };
        v10
    } else {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ol start=\""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v8));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\">\n"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
        v14
    }
}
fn markdown_lines_0(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 > v4;
        if v5 {
            let v6: Rc<str> = close_block_1(v2.clone());
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v6));
            return v7.clone();
        } else {
            let v8: i32 = find_nl_2(v0.clone(), v1);
            let v9: i32 = v8.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v0.clone(), v1 as i64, v9 as i64);
            let v11: i32 = v8.wrapping_add(1i32);
            match &v2 {
                US0::US0_MdCode => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                    let v13: i32 = 0i32;
                    let v14: bool = starts_at_3(v10.clone(), v12.clone(), v13);
                    if v14 {
                        let v15: US0 = US0::US0_MdNone;
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v16));
                        (v0, v1, v2, v3) = (v0.clone(), v11, v15.clone(), v17.clone());
                        continue;
                    } else {
                        let v19: US0 = US0::US0_MdCode;
                        let v20: Rc<str> = dtl_inline::dtl_escape(&*v10);
                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v20));
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                        (v0, v1, v2, v3) = (v0.clone(), v11, v19.clone(), v23.clone());
                        continue;
                    }
                }
                US0::US0_MdTable(v26) => {
                    let v26: Rc<str> = v26.clone();
                    let v27: bool = table_row_4(v10.clone());
                    if v27 {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = table_tr_6(v10.clone(), v26.clone(), v28.clone());
                        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v29));
                        (v0, v1, v2, v3) = (v0.clone(), v11, v2.clone(), v30.clone());
                        continue;
                    } else {
                        let v32: US0 = US0::US0_MdNone;
                        let v33: Rc<str> = close_block_1(v2.clone());
                        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v33));
                        (v0, v1, v2, v3) = (v0.clone(), v1, v32.clone(), v34.clone());
                        continue;
                    }
                }
                _ => {
                    let v37: i32 = heading_level_15(v10.clone());
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                    let v39: i32 = 0i32;
                    let v40: bool = starts_at_3(v10.clone(), v38.clone(), v39);
                    if v40 {
                        let v41: US0 = US0::US0_MdCode;
                        let v42: Rc<str> = close_block_1(v2.clone());
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v42));
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                        (v0, v1, v2, v3) = (v0.clone(), v11, v41.clone(), v45.clone());
                        continue;
                    } else {
                        let v47: i32 = 0i32;
                        let v48: bool = blank_from_17(v10.clone(), v47);
                        if v48 {
                            let v49: US0 = US0::US0_MdNone;
                            let v50: Rc<str> = close_block_1(v2.clone());
                            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v50));
                            (v0, v1, v2, v3) = (v0.clone(), v11, v49.clone(), v51.clone());
                            continue;
                        } else {
                            let v53: bool = v37 > 0i32;
                            if v53 {
                                let v54: bool = v37 == 1i32;
                                let v69: Rc<str> = if v54 {
                                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1"); } LIT.with(|lit| lit.clone()) };
                                    v55.clone()
                                } else {
                                    let v56: bool = v37 == 2i32;
                                    if v56 {
                                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h2"); } LIT.with(|lit| lit.clone()) };
                                        v57.clone()
                                    } else {
                                        let v58: bool = v37 == 3i32;
                                        if v58 {
                                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h3"); } LIT.with(|lit| lit.clone()) };
                                            v59.clone()
                                        } else {
                                            let v60: bool = v37 == 4i32;
                                            if v60 {
                                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h4"); } LIT.with(|lit| lit.clone()) };
                                                v61.clone()
                                            } else {
                                                let v62: bool = v37 == 5i32;
                                                if v62 {
                                                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h5"); } LIT.with(|lit| lit.clone()) };
                                                    v63.clone()
                                                } else {
                                                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h6"); } LIT.with(|lit| lit.clone()) };
                                                    v64.clone()
                                                }
                                            }
                                        }
                                    }
                                };
                                let v70: i32 = v37.wrapping_add(1i32);
                                let v71: i32 = (v10.clone().len() as i32);
                                let v72: i32 = v71.wrapping_sub(1i32);
                                let v73: Rc<str> = string_slice(&v10.clone(), v70 as i64, v72 as i64);
                                let v74: US0 = US0::US0_MdNone;
                                let v75: Rc<str> = close_block_1(v2.clone());
                                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v75));
                                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                                let v79: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v76, v77)), v69));
                                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                                let v82: Rc<str> = dtl_inline::dtl_render_inline(&*v73);
                                let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                                let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
                                let v86: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v83, v84)), v69));
                                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">\n"); } LIT.with(|lit| lit.clone()) };
                                let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
                                (v0, v1, v2, v3) = (v0.clone(), v11, v74.clone(), v88.clone());
                                continue;
                            } else {
                                let v90: bool = &*v10 == "---";
                                if v90 {
                                    let v91: US0 = US0::US0_MdNone;
                                    let v92: Rc<str> = close_block_1(v2.clone());
                                    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v92));
                                    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<hr>\n"); } LIT.with(|lit| lit.clone()) };
                                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v94));
                                    (v0, v1, v2, v3) = (v0.clone(), v11, v91.clone(), v95.clone());
                                    continue;
                                } else {
                                    let v97: bool = table_row_4(v10.clone());
                                    let v100: bool = if v97 {
                                        let v98: Rc<str> = line_at_18(v0.clone(), v11);
                                        delimiter_row_19(v98.clone())
                                    } else {
                                        false
                                    };
                                    if v100 {
                                        let v101: Rc<str> = line_at_18(v0.clone(), v11);
                                        let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
                                        let v103: Rc<str> = table_tr_6(v10.clone(), v101.clone(), v102.clone());
                                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n<thead>\n"); } LIT.with(|lit| lit.clone()) };
                                        let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v103));
                                        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</thead>\n<tbody>\n"); } LIT.with(|lit| lit.clone()) };
                                        let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v106));
                                        let v108: i32 = find_nl_2(v0.clone(), v11);
                                        let v109: i32 = v108.wrapping_add(1i32);
                                        let v110: US0 = US0::US0_MdTable(v101.clone());
                                        let v111: Rc<str> = close_block_1(v2.clone());
                                        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v111)), v107));
                                        (v0, v1, v2, v3) = (v0.clone(), v109, v110.clone(), v113.clone());
                                        continue;
                                    } else {
                                        let v115: i32 = bullet_start_21(v10.clone());
                                        let v116: bool = v115 > 0i32;
                                        if v116 {
                                            let v117: i32 = bullet_start_21(v10.clone());
                                            let v118: i32 = (v10.clone().len() as i32);
                                            let v119: bool = v117 == v118;
                                            let v123: Rc<str> = if v119 {
                                                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                v120.clone()
                                            } else {
                                                let v121: i32 = v118.wrapping_sub(1i32);
                                                let v122: Rc<str> = string_slice(&v10.clone(), v117 as i64, v121 as i64);
                                                v122.clone()
                                            };
                                            let v124: Rc<str> = dtl_inline::dtl_render_inline(&*v123);
                                            let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) };
                                            let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v124));
                                            match &v2 {
                                                US0::US0_MdList => {
                                                    let v127: US0 = US0::US0_MdList;
                                                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) };
                                                    let v130: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v128)), v126));
                                                    (v0, v1, v2, v3) = (v0.clone(), v11, v127.clone(), v130.clone());
                                                    continue;
                                                }
                                                _ => {
                                                    let v132: US0 = US0::US0_MdList;
                                                    let v133: Rc<str> = close_block_1(v2.clone());
                                                    let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v133));
                                                    let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ul>\n"); } LIT.with(|lit| lit.clone()) };
                                                    let v137: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v134, v135)), v126));
                                                    (v0, v1, v2, v3) = (v0.clone(), v11, v132.clone(), v137.clone());
                                                    continue;
                                                }
                                            }
                                        } else {
                                            let v140: i32 = ordered_start_22(v10.clone());
                                            let v141: bool = v140 > 0i32;
                                            if v141 {
                                                let v142: i32 = ordered_start_22(v10.clone());
                                                let v143: i32 = (v10.clone().len() as i32);
                                                let v144: bool = v142 == v143;
                                                let v148: Rc<str> = if v144 {
                                                    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    v145.clone()
                                                } else {
                                                    let v146: i32 = v143.wrapping_sub(1i32);
                                                    let v147: Rc<str> = string_slice(&v10.clone(), v142 as i64, v146 as i64);
                                                    v147.clone()
                                                };
                                                let v149: Rc<str> = dtl_inline::dtl_render_inline(&*v148);
                                                let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) };
                                                let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v149));
                                                match &v2 {
                                                    US0::US0_MdOrdered => {
                                                        let v152: US0 = US0::US0_MdOrdered;
                                                        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) };
                                                        let v155: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v153)), v151));
                                                        (v0, v1, v2, v3) = (v0.clone(), v11, v152.clone(), v155.clone());
                                                        continue;
                                                    }
                                                    _ => {
                                                        let v157: US0 = US0::US0_MdOrdered;
                                                        let v158: Rc<str> = close_block_1(v2.clone());
                                                        let v159: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v158));
                                                        let v160: Rc<str> = ordered_open_24(v10.clone());
                                                        let v162: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v159, v160)), v151));
                                                        (v0, v1, v2, v3) = (v0.clone(), v11, v157.clone(), v162.clone());
                                                        continue;
                                                    }
                                                }
                                            } else {
                                                match &v2 {
                                                    US0::US0_MdList => {
                                                        let v171: US0 = US0::US0_MdList;
                                                        let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                        let v173: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v172));
                                                        let v174: Rc<str> = trimmed_8(v10.clone());
                                                        let v175: Rc<str> = dtl_inline::dtl_render_inline(&*v174);
                                                        let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v175));
                                                        (v0, v1, v2, v3) = (v0.clone(), v11, v171.clone(), v176.clone());
                                                        continue;
                                                    }
                                                    US0::US0_MdOrdered => {
                                                        let v178: US0 = US0::US0_MdOrdered;
                                                        let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                        let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v179));
                                                        let v181: Rc<str> = trimmed_8(v10.clone());
                                                        let v182: Rc<str> = dtl_inline::dtl_render_inline(&*v181);
                                                        let v183: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v182));
                                                        (v0, v1, v2, v3) = (v0.clone(), v11, v178.clone(), v183.clone());
                                                        continue;
                                                    }
                                                    US0::US0_MdParagraph => {
                                                        let v165: US0 = US0::US0_MdParagraph;
                                                        let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                        let v167: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v166));
                                                        let v168: Rc<str> = dtl_inline::dtl_render_inline(&*v10);
                                                        let v169: Rc<str> = Rc::<str>::from(format!("{}{}", v167, v168));
                                                        (v0, v1, v2, v3) = (v0.clone(), v11, v165.clone(), v169.clone());
                                                        continue;
                                                    }
                                                    _ => {
                                                        let v185: US0 = US0::US0_MdParagraph;
                                                        let v186: Rc<str> = close_block_1(v2.clone());
                                                        let v187: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v186));
                                                        let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) };
                                                        let v189: Rc<str> = Rc::<str>::from(format!("{}{}", v187, v188));
                                                        let v190: Rc<str> = dtl_inline::dtl_render_inline(&*v10);
                                                        let v191: Rc<str> = Rc::<str>::from(format!("{}{}", v189, v190));
                                                        (v0, v1, v2, v3) = (v0.clone(), v11, v185.clone(), v191.clone());
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: US0 = US0::US0_MdNone;
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        markdown_lines_0(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_markdown(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
