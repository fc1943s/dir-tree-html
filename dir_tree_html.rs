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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_MissingFlag(Rc<str>),
    US2_BadNumber(Rc<str>, Rc<str>),
    US2_UnknownVerb(Rc<str>, Rc<UH0>),
    US2_NoVerb(Rc<UH0>),
    US2_Help(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Ok(Rc<str>, Rc<str>),
    US1_Error(US2),
}
#[derive(Clone)]
enum US3 {
    US3_Some(US2),
    US3_None,
}
enum US4 {
    US4_Ok(Vec<u8>),
    US4_Error(std::io::Error),
}
#[derive(Clone)]
enum US5 {
    US5_Ok(std::string::String),
    US5_Error(std::string::FromUtf8Error),
}
fn command_line_argument_count_1() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_0(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_1();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_2(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_0(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn walk_3(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v2 == &*v0;
                if v4 {
                    return true;
                } else {
                    (v0, v1) = (v0.clone(), v3.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn value_after_4(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == &*v0;
                        if v6 {
                            return US0::US0_Some(v4.clone());
                        } else {
                            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                            (v0, v1) = (v0.clone(), v8.clone());
                            continue;
                        }
                    }
                    _ => {
                        return US0::US0_None;
                    }
                }
            }
            _ => {
                return US0::US0_None;
            }
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v1 == "";
                let v8: Rc<str> = if v4 {
                    v2.clone()
                } else {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | "); } LIT.with(|lit| lit.clone()) };
                    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v5)), v2));
                    v7.clone()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn ok_c0() -> Rc<dyn Fn(Vec<u8>) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<u8>) -> US4> = Rc::new(move |mut v0: Vec<u8>| -> US4 {
        US4::US4_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6() -> Rc<dyn Fn(Vec<u8>) -> US4> {
    ok_c0()
}
fn error_c1() -> Rc<dyn Fn(std::io::Error) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US4> = Rc::new(move |mut v0: std::io::Error| -> US4 {
        US4::US4_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7() -> Rc<dyn Fn(std::io::Error) -> US4> {
    error_c1()
}
fn method8(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn ok_c2() -> Rc<dyn Fn(std::string::String) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::String) -> US5> = Rc::new(move |mut v0: std::string::String| -> US5 {
        US5::US5_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(std::string::String) -> US5> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::FromUtf8Error) -> US5> = Rc::new(move |mut v0: std::string::FromUtf8Error| -> US5 {
        US5::US5_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US5> {
    error_c3()
}
fn last_sep_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 == v3;
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
fn spiral_main() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_0(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("self-test"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--self-test"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = walk_3(v4, v1.clone());
    if v5 {
        let v6: i32 = dtl_self_test::dtl_self_test(0i32);
        v6
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help"); } LIT.with(|lit| lit.clone()) };
        let v9: bool = walk_3(v8, v1.clone());
        let v72: US1 = if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <text>"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir <text>"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = &*v13 == "";
            let v16: Rc<str> = if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                v13
            };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v17)), v12));
            let v20: bool = &*v19 == "";
            let v26: Rc<str> = if v20 {
                v16.clone()
            } else {
                let v21: bool = &*v16 == "";
                if v21 {
                    v19.clone()
                } else {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v22)), v19));
                    v24
                }
            };
            let v27: US2 = US2::US2_Help(v26);
            US1::US1_Error(v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
            let v31: US0 = value_after_4(v30, v1.clone());
            let v37: US3 = match &v31 {
                US0::US0_None => {
                    let v34: US2 = US2::US2_MissingFlag(v29);
                    US3::US3_Some(v34)
                }
                US0::US0_Some(v32) => {
                    let v32: Rc<str> = v32.clone();
                    US3::US3_None
                }
            };
            let v49: US3 = match &v37 {
                US3::US3_None => {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                    let v41: US0 = value_after_4(v40, v1.clone());
                    match &v41 {
                        US0::US0_None => {
                            let v44: US2 = US2::US2_MissingFlag(v39);
                            US3::US3_Some(v44)
                        }
                        US0::US0_Some(v42) => {
                            let v42: Rc<str> = v42.clone();
                            US3::US3_None
                        }
                    }
                }
                US3::US3_Some(v38) => {
                    let v38: US2 = v38.clone();
                    v37
                }
            };
            match &v49 {
                US3::US3_None => {
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
                    let v53: US0 = value_after_4(v52, v1.clone());
                    let v59: Rc<str> = match &v53 {
                        US0::US0_None => {
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --dir"); } LIT.with(|lit| lit.clone()) };
                            std::panic::panic_any::<std::string::String>(format!("{}", v56))
                        }
                        US0::US0_Some(v54) => {
                            let v54: Rc<str> = v54.clone();
                            v54
                        }
                    };
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                    let v62: US0 = value_after_4(v61, v1.clone());
                    let v68: Rc<str> = match &v62 {
                        US0::US0_None => {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --html"); } LIT.with(|lit| lit.clone()) };
                            std::panic::panic_any::<std::string::String>(format!("{}", v65))
                        }
                        US0::US0_Some(v63) => {
                            let v63: Rc<str> = v63.clone();
                            v63
                        }
                    };
                    US1::US1_Ok(v59, v68)
                }
                US3::US3_Some(v50) => {
                    let v50: US2 = v50.clone();
                    US1::US1_Error(v50)
                }
            }
        };
        match &v72 {
            US1::US1_Error(v242) => {
                let v242: US2 = v242.clone();
                let v276: Rc<str> = match &v242 {
                    US2::US2_BadNumber(v248, v249) => {
                        let v248: Rc<str> = v248.clone();
                        let v249: Rc<str> = v249.clone();
                        let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: --"); } LIT.with(|lit| lit.clone()) };
                        let v251: Rc<str> = Rc::<str>::from(format!("{}{}", v250, v248));
                        let v252: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects an integer, got '"); } LIT.with(|lit| lit.clone()) };
                        let v254: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v251, v252)), v249));
                        let v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v256: Rc<str> = Rc::<str>::from(format!("{}{}", v254, v255));
                        v256
                    }
                    US2::US2_Help(v271) => {
                        let v271: Rc<str> = v271.clone();
                        v271
                    }
                    US2::US2_MissingFlag(v243) => {
                        let v243: Rc<str> = v243.clone();
                        let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: missing --"); } LIT.with(|lit| lit.clone()) };
                        let v245: Rc<str> = Rc::<str>::from(format!("{}{}", v244, v243));
                        let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <value>"); } LIT.with(|lit| lit.clone()) };
                        let v247: Rc<str> = Rc::<str>::from(format!("{}{}", v245, v246));
                        v247
                    }
                    US2::US2_NoVerb(v266) => {
                        let v266: Rc<UH0> = v266.clone();
                        let v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v268: Rc<str> = method5(v266, v267);
                        let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: no command given\nvalid: "); } LIT.with(|lit| lit.clone()) };
                        let v270: Rc<str> = Rc::<str>::from(format!("{}{}", v269, v268));
                        v270
                    }
                    US2::US2_UnknownVerb(v257, v258) => {
                        let v257: Rc<str> = v257.clone();
                        let v258: Rc<UH0> = v258.clone();
                        let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: unknown command '"); } LIT.with(|lit| lit.clone()) };
                        let v260: Rc<str> = Rc::<str>::from(format!("{}{}", v259, v257));
                        let v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\nvalid: "); } LIT.with(|lit| lit.clone()) };
                        let v262: Rc<str> = Rc::<str>::from(format!("{}{}", v260, v261));
                        let v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v264: Rc<str> = method5(v258, v263);
                        let v265: Rc<str> = Rc::<str>::from(format!("{}{}", v262, v264));
                        v265
                    }
                };
                eprintln!("{}", v276);
                2i32
            }
            US1::US1_Ok(v73, v74) => {
                let v73: Rc<str> = v73.clone();
                let v74: Rc<str> = v74.clone();
                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("viewers"); } LIT.with(|lit| lit.clone()) };
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--viewers"); } LIT.with(|lit| lit.clone()) };
                let v77: bool = walk_3(v76, v1);
                if v77 {
                    let v78: Rc<str> = dtl_tree::dtl_tree_html_viewers(&*v73);
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
                    let v83: Rc<str> = Rc::<str>::from(std::path::Path::new(v73.as_ref()).join(v82.as_ref()).display().to_string());
                    let v85: bool = std::path::Path::new(v83.as_ref()).is_file();
                    let v128: Rc<str> = if v85 {
                        let v87: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v83);
                        let mut v88: Rc<dyn Fn(Vec<u8>) -> US4> = method6();
                        let mut v89: Rc<dyn Fn(std::io::Error) -> US4> = method7();
                        let v90: US4 = match v87 { Ok(x) => v88(x), Err(e) => v89(e) };
                        let v121: Rc<str> = match v90 {
                            US4::US4_Error(v91) => {
                                let v91: std::io::Error = v91;
                                let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v92
                            }
                            US4::US4_Ok(v93) => {
                                let v93: Vec<u8> = v93.clone();
                                let v94: Vec<u8> = method8(v93);
                                let v96: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v94);
                                let mut v97: Rc<dyn Fn(std::string::String) -> US5> = method9();
                                let mut v98: Rc<dyn Fn(std::string::FromUtf8Error) -> US5> = method10();
                                let v99: US5 = match v96 { Ok(x) => v97(x), Err(e) => v98(e) };
                                match &v99 {
                                    US5::US5_Error(v100) => {
                                        let v100: std::string::FromUtf8Error = v100.clone();
                                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v101
                                    }
                                    US5::US5_Ok(v102) => {
                                        let v102: std::string::String = v102.clone();
                                        let v104: Rc<str> = Rc::<str>::from(String::as_str(&v102));
                                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
                                        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let v107: Rc<str> = Rc::<str>::from(v104.replace(&*v105, &*v106));
                                        let v108: bool = v107.ends_with(&*v106);
                                        let v111: bool = if v108 {
                                            let v109: i32 = (v107.clone().len() as i32);
                                            let v110: bool = v109 == 1i32;
                                            v110
                                        } else {
                                            false
                                        };
                                        if v111 {
                                            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v112
                                        } else {
                                            if v108 {
                                                let v113: i32 = (v107.clone().len() as i32);
                                                let v114: i32 = v113.wrapping_sub(2i32);
                                                let v115: Rc<str> = string_slice(&v107.clone(), 0i32 as i64, v114 as i64);
                                                v115
                                            } else {
                                                v107
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        let v122: Rc<str> = dtl_markdown::dtl_markdown(&*v121);
                        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
                        let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v122));
                        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
                        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v125));
                        v126
                    } else {
                        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v127
                    };
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v128));
                    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v80, Rc::<str>::from(format!("{}{}", v130, v131))));
                    let v134: i32 = dtl_tree::dtl_write_viewers(&*v73);
                    let v135: bool = v134 == 0i32;
                    if v135 {
                        let v136: i32 = 0i32;
                        let v137: i32 = -1i32;
                        let v138: i32 = last_sep_11(v74.clone(), v136, v137);
                        let v139: bool = v138 == -1i32;
                        let v150: Rc<str> = if v139 {
                            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v140
                        } else {
                            let v141: i32 = v138.wrapping_sub(1i32);
                            let v142: Rc<str> = string_slice(&v74.clone(), 0i32 as i64, v141 as i64);
                            let v143: i32 = (v142.clone().len() as i32);
                            let v144: bool = v143 == 2i32;
                            let v147: bool = if v144 {
                                let v145: u8 = v142.clone().as_bytes()[1i32 as usize];
                                let v146: bool = v145 == b':';
                                v146
                            } else {
                                false
                            };
                            if v147 {
                                let v148: Rc<str> = string_slice(&v74.clone(), 0i32 as i64, v138 as i64);
                                v148
                            } else {
                                v142
                            }
                        };
                        let v151: i32 = (v150.clone().len() as i32);
                        let v152: bool = v151 == 0i32;
                        let v155: i32 = if v152 {
                            0i32
                        } else {
                            let v154: i32 = i32::from(std::fs::create_dir_all(v150.as_ref()).is_err());
                            v154
                        };
                        let v156: bool = v155 == 0i32;
                        if v156 {
                            let v158: i32 = i32::from(std::fs::write(v74.as_ref(), v133.as_ref().as_bytes()).is_err());
                            v158
                        } else {
                            v155
                        }
                    } else {
                        1i32
                    }
                } else {
                    let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
                    let v163: Rc<str> = Rc::<str>::from(std::path::Path::new(v73.as_ref()).join(v162.as_ref()).display().to_string());
                    let v165: bool = std::path::Path::new(v163.as_ref()).is_file();
                    let v208: Rc<str> = if v165 {
                        let v167: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v163);
                        let mut v168: Rc<dyn Fn(Vec<u8>) -> US4> = method6();
                        let mut v169: Rc<dyn Fn(std::io::Error) -> US4> = method7();
                        let v170: US4 = match v167 { Ok(x) => v168(x), Err(e) => v169(e) };
                        let v201: Rc<str> = match v170 {
                            US4::US4_Error(v171) => {
                                let v171: std::io::Error = v171;
                                let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v172
                            }
                            US4::US4_Ok(v173) => {
                                let v173: Vec<u8> = v173.clone();
                                let v174: Vec<u8> = method8(v173);
                                let v176: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v174);
                                let mut v177: Rc<dyn Fn(std::string::String) -> US5> = method9();
                                let mut v178: Rc<dyn Fn(std::string::FromUtf8Error) -> US5> = method10();
                                let v179: US5 = match v176 { Ok(x) => v177(x), Err(e) => v178(e) };
                                match &v179 {
                                    US5::US5_Error(v180) => {
                                        let v180: std::string::FromUtf8Error = v180.clone();
                                        let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v181
                                    }
                                    US5::US5_Ok(v182) => {
                                        let v182: std::string::String = v182.clone();
                                        let v184: Rc<str> = Rc::<str>::from(String::as_str(&v182));
                                        let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
                                        let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                        let v187: Rc<str> = Rc::<str>::from(v184.replace(&*v185, &*v186));
                                        let v188: bool = v187.ends_with(&*v186);
                                        let v191: bool = if v188 {
                                            let v189: i32 = (v187.clone().len() as i32);
                                            let v190: bool = v189 == 1i32;
                                            v190
                                        } else {
                                            false
                                        };
                                        if v191 {
                                            let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v192
                                        } else {
                                            if v188 {
                                                let v193: i32 = (v187.clone().len() as i32);
                                                let v194: i32 = v193.wrapping_sub(2i32);
                                                let v195: Rc<str> = string_slice(&v187.clone(), 0i32 as i64, v194 as i64);
                                                v195
                                            } else {
                                                v187
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        let v202: Rc<str> = dtl_markdown::dtl_markdown(&*v201);
                        let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
                        let v204: Rc<str> = Rc::<str>::from(format!("{}{}", v203, v202));
                        let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
                        let v206: Rc<str> = Rc::<str>::from(format!("{}{}", v204, v205));
                        v206
                    } else {
                        let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v207
                    };
                    let v209: Rc<str> = dtl_tree::dtl_tree_html(&*v73);
                    let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
                    let v211: Rc<str> = Rc::<str>::from(format!("{}{}", v210, v209));
                    let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
                    let v213: Rc<str> = Rc::<str>::from(format!("{}{}", v212, v208));
                    let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
                    let v216: Rc<str> = Rc::<str>::from(format!("{}{}", v211, Rc::<str>::from(format!("{}{}", v213, v214))));
                    let v217: i32 = 0i32;
                    let v218: i32 = -1i32;
                    let v219: i32 = last_sep_11(v74.clone(), v217, v218);
                    let v220: bool = v219 == -1i32;
                    let v231: Rc<str> = if v220 {
                        let v221: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v221
                    } else {
                        let v222: i32 = v219.wrapping_sub(1i32);
                        let v223: Rc<str> = string_slice(&v74.clone(), 0i32 as i64, v222 as i64);
                        let v224: i32 = (v223.clone().len() as i32);
                        let v225: bool = v224 == 2i32;
                        let v228: bool = if v225 {
                            let v226: u8 = v223.clone().as_bytes()[1i32 as usize];
                            let v227: bool = v226 == b':';
                            v227
                        } else {
                            false
                        };
                        if v228 {
                            let v229: Rc<str> = string_slice(&v74.clone(), 0i32 as i64, v219 as i64);
                            v229
                        } else {
                            v223
                        }
                    };
                    let v232: i32 = (v231.clone().len() as i32);
                    let v233: bool = v232 == 0i32;
                    let v236: i32 = if v233 {
                        0i32
                    } else {
                        let v235: i32 = i32::from(std::fs::create_dir_all(v231.as_ref()).is_err());
                        v235
                    };
                    let v237: bool = v236 == 0i32;
                    if v237 {
                        let v239: i32 = i32::from(std::fs::write(v74.as_ref(), v216.as_ref().as_bytes()).is_err());
                        v239
                    } else {
                        v236
                    }
                }
            }
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
