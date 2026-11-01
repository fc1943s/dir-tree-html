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
    US0_Direct,
    US0_SourceView,
    US0_TableView,
    US0_MarkdownView,
}
enum US1 {
    US1_Ok(Vec<u8>),
    US1_Error(std::io::Error),
}
#[derive(Clone)]
enum US2 {
    US2_Ok(std::string::String),
    US2_Error(std::string::FromUtf8Error),
}
fn last_dot_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'.';
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v7, v1);
                continue;
            } else {
                let v9: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v9, v2);
                continue;
            }
        }
    }
}
fn extension_1(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = -1i32;
    let mut v3: i32 = last_dot_2(v0.clone(), v1, v2);
    let mut v4: bool = v3 <= 0i32;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: i32 = v3.wrapping_add(1i32);
        let v7: i32 = (v0.clone().len() as i32);
        let v8: bool = v6 == v7;
        if v8 {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9
        } else {
            let v10: i32 = v7.wrapping_sub(1i32);
            let v11: Rc<str> = string_slice(&v0, v6 as i64, v10 as i64);
            v11
        }
    }
}
fn source_extension_3(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 == "spi";
    let mut v3: bool = if v1 {
        true
    } else {
        let v2: bool = &*v0 == "spir";
        v2
    };
    let v5: bool = if v3 {
        true
    } else {
        let mut v4: bool = &*v0 == "spiproj";
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = &*v0 == "toml";
        v6
    };
    let v9: bool = if v7 {
        true
    } else {
        let v8: bool = &*v0 == "ps1";
        v8
    };
    let v11: bool = if v9 {
        true
    } else {
        let v10: bool = &*v0 == "psm1";
        v10
    };
    let mut v13: bool = if v11 {
        true
    } else {
        let mut v12: bool = &*v0 == "fs";
        v12
    };
    let v15: bool = if v13 {
        true
    } else {
        let v14: bool = &*v0 == "fsx";
        v14
    };
    let v17: bool = if v15 {
        true
    } else {
        let v16: bool = &*v0 == "fsproj";
        v16
    };
    let v19: bool = if v17 {
        true
    } else {
        let v18: bool = &*v0 == "rs";
        v18
    };
    let v21: bool = if v19 {
        true
    } else {
        let v20: bool = &*v0 == "gleam";
        v20
    };
    let v23: bool = if v21 {
        true
    } else {
        let v22: bool = &*v0 == "ex";
        v22
    };
    let v25: bool = if v23 {
        true
    } else {
        let v24: bool = &*v0 == "exs";
        v24
    };
    let v27: bool = if v25 {
        true
    } else {
        let v26: bool = &*v0 == "zig";
        v26
    };
    let v29: bool = if v27 {
        true
    } else {
        let v28: bool = &*v0 == "c";
        v28
    };
    let v31: bool = if v29 {
        true
    } else {
        let v30: bool = &*v0 == "h";
        v30
    };
    let v33: bool = if v31 {
        true
    } else {
        let v32: bool = &*v0 == "cpp";
        v32
    };
    let v35: bool = if v33 {
        true
    } else {
        let v34: bool = &*v0 == "cs";
        v34
    };
    let v37: bool = if v35 {
        true
    } else {
        let v36: bool = &*v0 == "py";
        v36
    };
    let v39: bool = if v37 {
        true
    } else {
        let v38: bool = &*v0 == "sh";
        v38
    };
    let v41: bool = if v39 {
        true
    } else {
        let v40: bool = &*v0 == "cmd";
        v40
    };
    let v43: bool = if v41 {
        true
    } else {
        let v42: bool = &*v0 == "bat";
        v42
    };
    let v45: bool = if v43 {
        true
    } else {
        let v44: bool = &*v0 == "lock";
        v44
    };
    let v47: bool = if v45 {
        true
    } else {
        let v46: bool = &*v0 == "cfg";
        v46
    };
    let v49: bool = if v47 {
        true
    } else {
        let v48: bool = &*v0 == "ini";
        v48
    };
    let v51: bool = if v49 {
        true
    } else {
        let v50: bool = &*v0 == "sql";
        v50
    };
    let v53: bool = if v51 {
        true
    } else {
        let v52: bool = &*v0 == "csv";
        v52
    };
    let v55: bool = if v53 {
        true
    } else {
        let v54: bool = &*v0 == "dib";
        v54
    };
    if v55 {
        true
    } else {
        let v56: bool = &*v0 == "livemd";
        v56
    }
}
fn viewer_of_0(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = extension_1(v0);
    let v2: bool = &*v1 == "tsv";
    if v2 {
        US0::US0_TableView
    } else {
        let mut v4: bool = &*v1 == "md";
        if v4 {
            US0::US0_MarkdownView
        } else {
            let v6: bool = source_extension_3(v1);
            if v6 {
                US0::US0_SourceView
            } else {
                US0::US0_Direct
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let v1: US0 = viewer_of_0(v0.clone());
        match &v1 {
            US0::US0_Direct => {
                false
            }
            _ => {
                true
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn leaf_name_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3;
        if v4 {
            let v5: bool = v2 == v1;
            if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v6.clone();
            } else {
                let v7: i32 = v1.wrapping_sub(1i32);
                let v8: Rc<str> = string_slice(&v0.clone(), v2 as i64, v7 as i64);
                return v8.clone();
            }
        } else {
            let v10: u8 = v0.clone().as_bytes()[v1 as usize];
            let v11: bool = v10 == b'/';
            let mut v13: bool = if v11 {
                true
            } else {
                let mut v12: bool = v10 == b'\\';
                v12
            };
            if v13 {
                let v14: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v14, v14);
                continue;
            } else {
                let v16: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v16, v2);
                continue;
            }
        }
    }
}
fn present_5(mut v0: Rc<str>) -> bool {
    let v2: bool = std::path::Path::new(v0.as_ref()).is_file();
    v2
}
fn ok_c2() -> Rc<dyn Fn(Vec<u8>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<u8>) -> US1> = Rc::new(move |mut v0: Vec<u8>| -> US1 {
        US1::US1_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7() -> Rc<dyn Fn(Vec<u8>) -> US1> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c3()
}
fn method9(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn ok_c4() -> Rc<dyn Fn(std::string::String) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::String) -> US2> = Rc::new(move |mut v0: std::string::String| -> US2 {
        US2::US2_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn(std::string::String) -> US2> {
    ok_c4()
}
fn error_c5() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::FromUtf8Error) -> US2> = Rc::new(move |mut v0: std::string::FromUtf8Error| -> US2 {
        US2::US2_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US2> {
    error_c5()
}
fn slurp_6(mut v0: Rc<str>) -> Rc<str> {
    let v2: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let mut v3: Rc<dyn Fn(Vec<u8>) -> US1> = method7();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US1> = method8();
    let v5: US1 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    match v5 {
        US1::US1_Error(v6) => {
            let v6: std::io::Error = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US1::US1_Ok(v8) => {
            let v8: Vec<u8> = v8.clone();
            let v9: Vec<u8> = method9(v8);
            let v11: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v9);
            let mut v12: Rc<dyn Fn(std::string::String) -> US2> = method10();
            let mut v13: Rc<dyn Fn(std::string::FromUtf8Error) -> US2> = method11();
            let v14: US2 = match v11 { Ok(x) => v12(x), Err(e) => v13(e) };
            match &v14 {
                US2::US2_Error(v15) => {
                    let v15: std::string::FromUtf8Error = v15.clone();
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16
                }
                US2::US2_Ok(v17) => {
                    let v17: std::string::String = v17.clone();
                    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v21));
                    let v23: bool = v22.ends_with(&*v21);
                    let v26: bool = if v23 {
                        let v24: i32 = (v22.clone().len() as i32);
                        let v25: bool = v24 == 1i32;
                        v25
                    } else {
                        false
                    };
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v27
                    } else {
                        if v23 {
                            let v28: i32 = (v22.clone().len() as i32);
                            let v29: i32 = v28.wrapping_sub(2i32);
                            let v30: Rc<str> = string_slice(&v22.clone(), 0i32 as i64, v29 as i64);
                            v30
                        } else {
                            v22
                        }
                    }
                }
            }
        }
    }
}
fn find_nl_14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
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
fn find_tab_16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'\t';
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
fn tsv_cells_15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 > v4;
        if v5 {
            return v3.clone();
        } else {
            let v6: i32 = find_tab_16(v0.clone(), v1);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v2));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            let v11: bool = v1 == v6;
            let v15: Rc<str> = if v11 {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v12.clone()
            } else {
                let mut v13: i32 = v6.wrapping_sub(1i32);
                let v14: Rc<str> = string_slice(&v0.clone(), v1 as i64, v13 as i64);
                v14.clone()
            };
            let v16: Rc<str> = dtl_inline::dtl_escape(&*v15);
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v16));
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v17, v18)), v2)), v9));
            let v22: i32 = v6.wrapping_add(1i32);
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v21));
            (v0, v1, v2, v3) = (v0.clone(), v22, v2.clone(), v23.clone());
            continue;
        }
    }
}
fn tsv_rows_13(mut v0: Rc<str>, mut v1: i32, mut v2: bool, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 > v4;
        if v5 {
            return v3.clone();
        } else {
            let v6: i32 = find_nl_14(v0.clone(), v1);
            let v7: bool = v1 == v6;
            let v11: Rc<str> = if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let v9: i32 = v6.wrapping_sub(1i32);
                let v10: Rc<str> = string_slice(&v0.clone(), v1 as i64, v9 as i64);
                v10.clone()
            };
            let mut v12: bool = &*v11 == "";
            if v12 {
                let mut v13: i32 = v6.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v13, v2, v3.clone());
                continue;
            } else {
                let v17: Rc<str> = if v2 {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                } else {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                };
                let v18: i32 = v6.wrapping_add(1i32);
                let v19: bool = false;
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<tr>"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v20));
                let v22: i32 = 0i32;
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = tsv_cells_15(v11.clone(), v22, v17.clone(), v23.clone());
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v24));
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tr>\n"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                (v0, v1, v2, v3) = (v0.clone(), v18, v19, v27.clone());
                continue;
            }
        }
    }
}
fn viewer_body_12(mut v0: US0, mut v1: Rc<str>) -> Rc<str> {
    match &v0 {
        US0::US0_Direct => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_MarkdownView => {
            let v16: Rc<str> = dtl_markdown::dtl_markdown(&*v1);
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            v20
        }
        US0::US0_SourceView => {
            let mut v3: Rc<str> = dtl_inline::dtl_escape(&*v1);
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
            v7
        }
        US0::US0_TableView => {
            let v8: i32 = 0i32;
            let v9: bool = true;
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = tsv_rows_13(v1, v8, v9, v10);
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</table>\n"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            v15
        }
    }
}
fn viewer_page_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p><a href=\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v3)), v0));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\">"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let v8: Rc<str> = dtl_inline::dtl_escape(&*v0);
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a> <span>raw</span></p>\n"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v1));
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v12, Rc::<str>::from(format!("{}{}", v15, v16))));
    v18
}
fn last_sep_19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 == v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'/';
            let v9: i32 = if v6 {
                v1
            } else {
                let v7: bool = v5 == b'\\';
                if v7 {
                    v1
                } else {
                    v2
                }
            };
            let v10: i32 = v1.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v10, v9);
            continue;
        }
    }
}
fn put_18(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = 0i32;
    let mut v3: i32 = -1i32;
    let mut v4: i32 = last_sep_19(v0.clone(), v2, v3);
    let v5: bool = v4 == -1i32;
    let v16: Rc<str> = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let v7: i32 = v4.wrapping_sub(1i32);
        let v8: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v7 as i64);
        let v9: i32 = (v8.clone().len() as i32);
        let v10: bool = v9 == 2i32;
        let mut v13: bool = if v10 {
            let v11: u8 = v8.clone().as_bytes()[1i32 as usize];
            let mut v12: bool = v11 == b':';
            v12
        } else {
            false
        };
        if v13 {
            let v14: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v4 as i64);
            v14
        } else {
            v8
        }
    };
    let v17: i32 = (v16.clone().len() as i32);
    let v18: bool = v17 == 0i32;
    let v21: i32 = if v18 {
        0i32
    } else {
        let v20: i32 = i32::from(std::fs::create_dir_all(v16.as_ref()).is_err());
        v20
    };
    let v22: bool = v21 == 0i32;
    if v22 {
        let v24: i32 = i32::from(std::fs::write(v0.as_ref(), v1.as_ref().as_bytes()).is_err());
        v24
    } else {
        v21
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        let v1: i32 = 0i32;
        let v2: i32 = 0i32;
        let mut v3: Rc<str> = leaf_name_4(v0.clone(), v1, v2);
        let mut v4: US0 = viewer_of_0(v3.clone());
        match &v4 {
            US0::US0_Direct => {
                0i32
            }
            _ => {
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
                let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
                let v7: bool = present_5(v6.clone());
                if v7 {
                    0i32
                } else {
                    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
                    let v9: Rc<str> = slurp_6(v0.clone());
                    let v10: Rc<str> = viewer_body_12(v4.clone(), v9.clone());
                    let v11: Rc<str> = viewer_page_17(v3.clone(), v10.clone());
                    put_18(v8.clone(), v11.clone())
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn go_20(mut v0: i32, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 <= 0i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: u64 = v1 & 15u64;
            let v5: i32 = (v4 as i32);
            let v6: i32 = v0.wrapping_sub(1i32);
            let v7: u64 = v1.wrapping_shr((4i32) as u32);
            let v8: Rc<str> = string_slice(&{ thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789abcdef"); } LIT.with(|lit| lit.clone()) }, v5 as i64, v5 as i64);
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
            (v0, v1, v2) = (v6, v7, v9.clone());
            continue;
        }
    }
}
fn test_head_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p><a href=\""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v2)), v0));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\">"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v5)), v0));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a> <span>raw</span></p>\n"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    v9
}
fn test_page_22(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let mut v4: Rc<str> = slurp_6(v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: bool = v1.ends_with(&*v5);
    let v9: bool = if v6 {
        let v7: i32 = (v1.clone().len() as i32);
        let v8: bool = v7 == 1i32;
        v8
    } else {
        false
    };
    let v15: Rc<str> = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10
    } else {
        if v6 {
            let v11: i32 = (v1.clone().len() as i32);
            let mut v12: i32 = v11.wrapping_sub(2i32);
            let mut v13: Rc<str> = string_slice(&v1.clone(), 0i32 as i64, v12 as i64);
            v13
        } else {
            v1
        }
    };
    let v16: bool = &*v4 == &*v15;
    if v16 {
        0i32
    } else {
        1i32
    }
}
fn closure6() -> Rc<dyn Fn(i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> i32> = Rc::new(move |mut v0: i32| -> i32 {
        let v2: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let mut v3: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
        let v5: bool = true; // 1;
        let mut v3 = v3;
        let v7: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        std::hash::Hasher::write_u128(&mut v3, v7);
        let v8: u64 = std::hash::Hasher::finish(&v3);
        std::hash::Hasher::write_u64(&mut v3, v8);
        let v9: u64 = std::hash::Hasher::finish(&v3);
        let v10: u64 = v8 & 4294967295u64;
        let v11: i32 = 8i32;
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = go_20(v11, v10, v12.clone());
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
        let v16: u64 = v8.wrapping_shr((32i32) as u32);
        let v17: u64 = v16 & 65535u64;
        let v18: i32 = 4i32;
        let v19: Rc<str> = go_20(v18, v17, v12.clone());
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v19));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-4"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let v23: u64 = v8.wrapping_shr((48i32) as u32);
        let v24: u64 = v23 & 4095u64;
        let v25: i32 = 3i32;
        let v26: Rc<str> = go_20(v25, v24, v12.clone());
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v26)), v14));
        let v29: u64 = v9 & 16383u64;
        let v30: u64 = v29 | 32768u64;
        let v31: i32 = 4i32;
        let v32: Rc<str> = go_20(v31, v30, v12.clone());
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v32)), v14));
        let v35: u64 = v9.wrapping_shr((16i32) as u32);
        let v36: u64 = v35 & 281474976710655u64;
        let v37: i32 = 12i32;
        let v38: Rc<str> = go_20(v37, v36, v12.clone());
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v38));
        let v40: Rc<str> = Rc::<str>::from(v39.to_lowercase());
        let v41: Rc<str> = v40 ;
        let v43: Rc<str> = Rc::<str>::from(std::path::Path::new(v2.as_ref()).join(v41.as_ref()).display().to_string());
        let v45: i32 = i32::from(std::fs::create_dir_all(v43.as_ref()).is_err());
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi"); } LIT.with(|lit| lit.clone()) };
        let v48: Rc<str> = Rc::<str>::from(std::path::Path::new(v43.as_ref()).join(v47.as_ref()).display().to_string());
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = Rc::<str>::from(std::path::Path::new(v43.as_ref()).join(v50.as_ref()).display().to_string());
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c.txt"); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = Rc::<str>::from(std::path::Path::new(v43.as_ref()).join(v53.as_ref()).display().to_string());
        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x < 1"); } LIT.with(|lit| lit.clone()) };
        let v56: i32 = put_18(v48.clone(), v55.clone());
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1\th2\nv1\tv2"); } LIT.with(|lit| lit.clone()) };
        let v58: i32 = put_18(v51.clone(), v57.clone());
        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("t"); } LIT.with(|lit| lit.clone()) };
        let v60: i32 = put_18(v54.clone(), v59.clone());
        let v61: i32 = 0i32;
        let v62: i32 = 0i32;
        let v63: Rc<str> = leaf_name_4(v48.clone(), v61, v62);
        let v64: US0 = viewer_of_0(v63.clone());
        let v74: i32 = match &v64 {
            US0::US0_Direct => {
                0i32
            }
            _ => {
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v65));
                let v67: bool = present_5(v66.clone());
                if v67 {
                    0i32
                } else {
                    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v65));
                    let v69: Rc<str> = slurp_6(v48.clone());
                    let v70: Rc<str> = viewer_body_12(v64.clone(), v69.clone());
                    let v71: Rc<str> = viewer_page_17(v63.clone(), v70.clone());
                    put_18(v68.clone(), v71.clone())
                }
            }
        };
        let v75: i32 = 0i32;
        let v76: i32 = 0i32;
        let v77: Rc<str> = leaf_name_4(v51.clone(), v75, v76);
        let v78: US0 = viewer_of_0(v77.clone());
        let v88: i32 = match &v78 {
            US0::US0_Direct => {
                0i32
            }
            _ => {
                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
                let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v79));
                let v81: bool = present_5(v80.clone());
                if v81 {
                    0i32
                } else {
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v79));
                    let v83: Rc<str> = slurp_6(v51.clone());
                    let v84: Rc<str> = viewer_body_12(v78.clone(), v83.clone());
                    let v85: Rc<str> = viewer_page_17(v77.clone(), v84.clone());
                    put_18(v82.clone(), v85.clone())
                }
            }
        };
        let v89: i32 = v74.wrapping_add(v88);
        let v90: i32 = 0i32;
        let v91: i32 = 0i32;
        let v92: Rc<str> = leaf_name_4(v54.clone(), v90, v91);
        let v93: US0 = viewer_of_0(v92.clone());
        let v103: i32 = match &v93 {
            US0::US0_Direct => {
                0i32
            }
            _ => {
                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v94));
                let v96: bool = present_5(v95.clone());
                if v96 {
                    0i32
                } else {
                    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v94));
                    let v98: Rc<str> = slurp_6(v54.clone());
                    let v99: Rc<str> = viewer_body_12(v93.clone(), v98.clone());
                    let v100: Rc<str> = viewer_page_17(v92.clone(), v99.clone());
                    put_18(v97.clone(), v100.clone())
                }
            }
        };
        let v104: i32 = v89.wrapping_add(v103);
        let v105: i32 = 0i32;
        let v106: i32 = 0i32;
        let v107: Rc<str> = leaf_name_4(v48.clone(), v105, v106);
        let v108: US0 = viewer_of_0(v107.clone());
        let v118: i32 = match &v108 {
            US0::US0_Direct => {
                0i32
            }
            _ => {
                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
                let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v109));
                let v111: bool = present_5(v110.clone());
                if v111 {
                    0i32
                } else {
                    let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v109));
                    let v113: Rc<str> = slurp_6(v48.clone());
                    let v114: Rc<str> = viewer_body_12(v108.clone(), v113.clone());
                    let v115: Rc<str> = viewer_page_17(v107.clone(), v114.clone());
                    put_18(v112.clone(), v115.clone())
                }
            }
        };
        let v119: i32 = v104.wrapping_add(v118);
        let v120: Rc<str> = test_head_21(v47.clone());
        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>x &lt; 1</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v122, Rc::<str>::from(format!("{}{}", v124, v125))));
        let v128: i32 = test_page_22(v48.clone(), v127.clone());
        let v129: Rc<str> = test_head_21(v50.clone());
        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n<tr><th>h1</th><th>h2</th></tr>\n<tr><td>v1</td><td>v2</td></tr>\n</table>\n"); } LIT.with(|lit| lit.clone()) };
        let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
        let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v131, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v123, v12)), v125))));
        let v135: i32 = test_page_22(v51.clone(), v134.clone());
        let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
        let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v136));
        let v138: bool = present_5(v137.clone());
        let v139: i32 = if v138 {
            1i32
        } else {
            0i32
        };
        let v140: i32 = v119.wrapping_add(v128);
        let v141: i32 = v140.wrapping_add(v135);
        let v142: i32 = v141.wrapping_add(v139);
        v142
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_has_viewer(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
pub fn dtl_write_viewer(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
pub fn dtl_viewer_self_test(v0: i32) -> i32 {
    closure6()(v0)
}
