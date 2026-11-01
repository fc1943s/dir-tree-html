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
enum US0 {
    US0_Ok(Vec<u8>),
    US0_Error(std::io::Error),
}
#[derive(Clone)]
enum US1 {
    US1_Ok(std::string::String),
    US1_Error(std::string::FromUtf8Error),
}
fn go_0(mut v0: i32, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 <= 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: u64 = v1 & 15u64;
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
fn times_2(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let mut v2: bool = v1 <= 0i64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: i64 = v1.wrapping_sub(1i64);
        let v5: Rc<str> = times_2(v0.clone(), v4);
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
        v6
    }
}
fn last_sep_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
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
fn lay_1(mut v0: Rc<str>, mut v1: i64) -> i32 {
    loop {
        let mut v2: bool = v1 < 0i64;
        if v2 {
            return 0i32;
        } else {
            let v4: i32 = i32::from(std::fs::create_dir_all(v0.as_ref()).is_err());
            let v5: bool = v1 == 0i64;
            let v35: Rc<str> = if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            } else {
                let v7: bool = v1 == 1i64;
                if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                } else {
                    let v9: bool = v1 == 2i64;
                    if v9 {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
                        v10.clone()
                    } else {
                        let v11: bool = v1 == 3i64;
                        if v11 {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        } else {
                            let v13: bool = v1 == 4i64;
                            if v13 {
                                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                                v14.clone()
                            } else {
                                let v15: bool = v1 == 5i64;
                                if v15 {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5"); } LIT.with(|lit| lit.clone()) };
                                    v16.clone()
                                } else {
                                    let v17: bool = v1 == 6i64;
                                    if v17 {
                                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
                                        v18.clone()
                                    } else {
                                        let v19: bool = v1 == 7i64;
                                        if v19 {
                                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("7"); } LIT.with(|lit| lit.clone()) };
                                            v20.clone()
                                        } else {
                                            let v21: bool = v1 == 8i64;
                                            if v21 {
                                                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("8"); } LIT.with(|lit| lit.clone()) };
                                                v22.clone()
                                            } else {
                                                let v23: bool = v1 == 9i64;
                                                if v23 {
                                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("9"); } LIT.with(|lit| lit.clone()) };
                                                    v24.clone()
                                                } else {
                                                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                                                    v25.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v37.as_ref()).display().to_string());
            let v39: i64 = v1.wrapping_add(1i64);
            let v40: Rc<str> = times_2(v35.clone(), v39);
            let v41: i32 = 0i32;
            let v42: i32 = -1i32;
            let v43: i32 = last_sep_3(v38.clone(), v41, v42);
            let v44: bool = v43 == -1i32;
            let v55: Rc<str> = if v44 {
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v45.clone()
            } else {
                let v46: i32 = v43.wrapping_sub(1i32);
                let v47: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v46 as i64);
                let v48: i32 = (v47.clone().len() as i32);
                let v49: bool = v48 == 2i32;
                let v52: bool = if v49 {
                    let v50: u8 = v47.clone().as_bytes()[1i32 as usize];
                    let v51: bool = v50 == b':';
                    v51
                } else {
                    false
                };
                if v52 {
                    let v53: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v43 as i64);
                    v53.clone()
                } else {
                    v47.clone()
                }
            };
            let v56: i32 = (v55.clone().len() as i32);
            let v57: bool = v56 == 0i32;
            let v60: i32 = if v57 {
                0i32
            } else {
                let v59: i32 = i32::from(std::fs::create_dir_all(v55.as_ref()).is_err());
                v59
            };
            let v61: bool = v60 == 0i32;
            let v64: i32 = if v61 {
                let v63: i32 = i32::from(std::fs::write(v38.as_ref(), v40.as_ref().as_bytes()).is_err());
                v63
            } else {
                v60
            };
            let v66: Rc<str> = Rc::<str>::from(std::path::Path::new(v0.as_ref()).join(v35.as_ref()).display().to_string());
            let v67: i64 = v1.wrapping_sub(1i64);
            (v0, v1) = (v66.clone(), v67);
            continue;
        }
    }
}
fn same_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: bool = &*v0 == &*v1;
    if v3 {
        0i32
    } else {
        let v5: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let v7: Rc<str> = Rc::<str>::from(std::path::Path::new(v5.as_ref()).join(v2.as_ref()).display().to_string());
        let v8: i32 = 0i32;
        let v9: i32 = -1i32;
        let v10: i32 = last_sep_3(v7.clone(), v8, v9);
        let v11: bool = v10 == -1i32;
        let v22: Rc<str> = if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12
        } else {
            let v13: i32 = v10.wrapping_sub(1i32);
            let v14: Rc<str> = string_slice(&v7.clone(), 0i32 as i64, v13 as i64);
            let v15: i32 = (v14.clone().len() as i32);
            let v16: bool = v15 == 2i32;
            let v19: bool = if v16 {
                let v17: u8 = v14.clone().as_bytes()[1i32 as usize];
                let v18: bool = v17 == b':';
                v18
            } else {
                false
            };
            if v19 {
                let v20: Rc<str> = string_slice(&v7.clone(), 0i32 as i64, v10 as i64);
                v20
            } else {
                v14
            }
        };
        let v23: i32 = (v22.clone().len() as i32);
        let v24: bool = v23 == 0i32;
        let v27: i32 = if v24 {
            0i32
        } else {
            let v26: i32 = i32::from(std::fs::create_dir_all(v22.as_ref()).is_err());
            v26
        };
        let v28: bool = v27 == 0i32;
        let v31: i32 = if v28 {
            let v30: i32 = i32::from(std::fs::write(v7.as_ref(), v0.as_ref().as_bytes()).is_err());
            v30
        } else {
            v27
        };
        1i32
    }
}
fn ok_c1() -> Rc<dyn Fn(Vec<u8>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<u8>) -> US0> = Rc::new(move |mut v0: Vec<u8>| -> US0 {
        US0::US0_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5() -> Rc<dyn Fn(Vec<u8>) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US0> = Rc::new(move |mut v0: std::io::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6() -> Rc<dyn Fn(std::io::Error) -> US0> {
    error_c2()
}
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn ok_c3() -> Rc<dyn Fn(std::string::String) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::String) -> US1> = Rc::new(move |mut v0: std::string::String| -> US1 {
        US1::US1_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8() -> Rc<dyn Fn(std::string::String) -> US1> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::string::FromUtf8Error) -> US1> = Rc::new(move |mut v0: std::string::FromUtf8Error| -> US1 {
        US1::US1_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn(std::string::FromUtf8Error) -> US1> {
    error_c4()
}
fn viewers_test_10() -> i32 {
    let v1: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
    let mut v2: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
    let v4: bool = true; // 1;
    let mut v2 = v2;
    let v6: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    std::hash::Hasher::write_u128(&mut v2, v6);
    let v7: u64 = std::hash::Hasher::finish(&v2);
    std::hash::Hasher::write_u64(&mut v2, v7);
    let v8: u64 = std::hash::Hasher::finish(&v2);
    let v9: u64 = v7 & 4294967295u64;
    let v10: i32 = 8i32;
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = go_0(v10, v9, v11.clone());
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: u64 = v7.wrapping_shr((32i32) as u32);
    let v16: u64 = v15 & 65535u64;
    let v17: i32 = 4i32;
    let v18: Rc<str> = go_0(v17, v16, v11.clone());
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-4"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: u64 = v7.wrapping_shr((48i32) as u32);
    let v23: u64 = v22 & 4095u64;
    let v24: i32 = 3i32;
    let v25: Rc<str> = go_0(v24, v23, v11.clone());
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v25)), v13));
    let v28: u64 = v8 & 16383u64;
    let v29: u64 = v28 | 32768u64;
    let v30: i32 = 4i32;
    let v31: Rc<str> = go_0(v30, v29, v11.clone());
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v27, v31)), v13));
    let v34: u64 = v8.wrapping_shr((16i32) as u32);
    let v35: u64 = v34 & 281474976710655u64;
    let v36: i32 = 12i32;
    let v37: Rc<str> = go_0(v36, v35, v11.clone());
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v37));
    let v39: Rc<str> = Rc::<str>::from(v38.to_lowercase());
    let v40: Rc<str> = v39 ;
    let v42: Rc<str> = Rc::<str>::from(std::path::Path::new(v1.as_ref()).join(v40.as_ref()).display().to_string());
    let v44: i32 = i32::from(std::fs::create_dir_all(v42.as_ref()).is_err());
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi"); } LIT.with(|lit| lit.clone()) };
    let v47: Rc<str> = Rc::<str>::from(std::path::Path::new(v42.as_ref()).join(v46.as_ref()).display().to_string());
    let v48: i32 = 0i32;
    let v49: i32 = -1i32;
    let v50: i32 = last_sep_3(v47.clone(), v48, v49);
    let v51: bool = v50 == -1i32;
    let v61: Rc<str> = if v51 {
        v11.clone()
    } else {
        let v52: i32 = v50.wrapping_sub(1i32);
        let v53: Rc<str> = string_slice(&v47.clone(), 0i32 as i64, v52 as i64);
        let v54: i32 = (v53.clone().len() as i32);
        let v55: bool = v54 == 2i32;
        let v58: bool = if v55 {
            let v56: u8 = v53.clone().as_bytes()[1i32 as usize];
            let v57: bool = v56 == b':';
            v57
        } else {
            false
        };
        if v58 {
            let v59: Rc<str> = string_slice(&v47.clone(), 0i32 as i64, v50 as i64);
            v59
        } else {
            v53
        }
    };
    let v62: i32 = (v61.clone().len() as i32);
    let v63: bool = v62 == 0i32;
    let v66: i32 = if v63 {
        0i32
    } else {
        let v65: i32 = i32::from(std::fs::create_dir_all(v61.as_ref()).is_err());
        v65
    };
    let v67: bool = v66 == 0i32;
    let v71: i32 = if v67 {
        let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x < 1"); } LIT.with(|lit| lit.clone()) };
        let v70: i32 = i32::from(std::fs::write(v47.as_ref(), v69.as_ref().as_bytes()).is_err());
        v70
    } else {
        v66
    };
    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv"); } LIT.with(|lit| lit.clone()) };
    let v74: Rc<str> = Rc::<str>::from(std::path::Path::new(v42.as_ref()).join(v73.as_ref()).display().to_string());
    let v75: i32 = 0i32;
    let v76: i32 = -1i32;
    let v77: i32 = last_sep_3(v74.clone(), v75, v76);
    let v78: bool = v77 == -1i32;
    let v88: Rc<str> = if v78 {
        v11.clone()
    } else {
        let v79: i32 = v77.wrapping_sub(1i32);
        let v80: Rc<str> = string_slice(&v74.clone(), 0i32 as i64, v79 as i64);
        let v81: i32 = (v80.clone().len() as i32);
        let v82: bool = v81 == 2i32;
        let v85: bool = if v82 {
            let v83: u8 = v80.clone().as_bytes()[1i32 as usize];
            let v84: bool = v83 == b':';
            v84
        } else {
            false
        };
        if v85 {
            let v86: Rc<str> = string_slice(&v74.clone(), 0i32 as i64, v77 as i64);
            v86
        } else {
            v80
        }
    };
    let v89: i32 = (v88.clone().len() as i32);
    let v90: bool = v89 == 0i32;
    let v93: i32 = if v90 {
        0i32
    } else {
        let v92: i32 = i32::from(std::fs::create_dir_all(v88.as_ref()).is_err());
        v92
    };
    let v94: bool = v93 == 0i32;
    let v98: i32 = if v94 {
        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1\th2\nv1\tv2"); } LIT.with(|lit| lit.clone()) };
        let v97: i32 = i32::from(std::fs::write(v74.as_ref(), v96.as_ref().as_bytes()).is_err());
        v97
    } else {
        v93
    };
    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c.txt"); } LIT.with(|lit| lit.clone()) };
    let v101: Rc<str> = Rc::<str>::from(std::path::Path::new(v42.as_ref()).join(v100.as_ref()).display().to_string());
    let v102: i32 = 0i32;
    let v103: i32 = -1i32;
    let v104: i32 = last_sep_3(v101.clone(), v102, v103);
    let v105: bool = v104 == -1i32;
    let v115: Rc<str> = if v105 {
        v11
    } else {
        let v106: i32 = v104.wrapping_sub(1i32);
        let v107: Rc<str> = string_slice(&v101.clone(), 0i32 as i64, v106 as i64);
        let v108: i32 = (v107.clone().len() as i32);
        let v109: bool = v108 == 2i32;
        let v112: bool = if v109 {
            let v110: u8 = v107.clone().as_bytes()[1i32 as usize];
            let v111: bool = v110 == b':';
            v111
        } else {
            false
        };
        if v112 {
            let v113: Rc<str> = string_slice(&v101.clone(), 0i32 as i64, v104 as i64);
            v113
        } else {
            v107
        }
    };
    let v116: i32 = (v115.clone().len() as i32);
    let v117: bool = v116 == 0i32;
    let v120: i32 = if v117 {
        0i32
    } else {
        let v119: i32 = i32::from(std::fs::create_dir_all(v115.as_ref()).is_err());
        v119
    };
    let v121: bool = v120 == 0i32;
    let v125: i32 = if v121 {
        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("t"); } LIT.with(|lit| lit.clone()) };
        let v124: i32 = i32::from(std::fs::write(v101.as_ref(), v123.as_ref().as_bytes()).is_err());
        v124
    } else {
        v120
    };
    let v126: Rc<str> = dtl_tree::dtl_tree_html_viewers(&*v42);
    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>&#128196; <a href=\"a.spi.html\">a.spi</a><span> (5.00 B)</span> <a href=\"a.spi\"><span>raw</span></a></div><div>&#128196; <a href=\"b.tsv.html\">b.tsv</a><span> (11.00 B)</span> <a href=\"b.tsv\"><span>raw</span></a></div><div>&#128196; <a href=\"c.txt\">c.txt</a><span> (1.00 B)</span></div>"); } LIT.with(|lit| lit.clone()) };
    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-viewers-got.html"); } LIT.with(|lit| lit.clone()) };
    let v129: i32 = same_4(v126, v127, v128);
    let v130: i32 = dtl_tree::dtl_write_viewers(&*v42);
    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi.html"); } LIT.with(|lit| lit.clone()) };
    let v133: Rc<str> = Rc::<str>::from(std::path::Path::new(v42.as_ref()).join(v132.as_ref()).display().to_string());
    let v135: bool = std::path::Path::new(v133.as_ref()).is_file();
    let v141: bool = if v135 {
        let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv.html"); } LIT.with(|lit| lit.clone()) };
        let v138: Rc<str> = Rc::<str>::from(std::path::Path::new(v42.as_ref()).join(v137.as_ref()).display().to_string());
        let v140: bool = std::path::Path::new(v138.as_ref()).is_file();
        v140
    } else {
        false
    };
    let v142: i32 = if v141 {
        0i32
    } else {
        1i32
    };
    let v143: i32 = v129.wrapping_add(v130);
    let v144: i32 = v143.wrapping_add(v142);
    let v145: i32 = dtl_viewer::dtl_viewer_self_test(0i32);
    let v146: i32 = v144.wrapping_add(v145);
    v146
}
fn closure0() -> Rc<dyn Fn(i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> i32> = Rc::new(move |mut v0: i32| -> i32 {
        let mut v2: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
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
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = go_0(v11, v10, v12.clone());
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
        let v16: u64 = v8.wrapping_shr((32i32) as u32);
        let v17: u64 = v16 & 65535u64;
        let v18: i32 = 4i32;
        let v19: Rc<str> = go_0(v18, v17, v12.clone());
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v19));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-4"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let v23: u64 = v8.wrapping_shr((48i32) as u32);
        let v24: u64 = v23 & 4095u64;
        let v25: i32 = 3i32;
        let v26: Rc<str> = go_0(v25, v24, v12.clone());
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v26)), v14));
        let v29: u64 = v9 & 16383u64;
        let v30: u64 = v29 | 32768u64;
        let v31: i32 = 4i32;
        let v32: Rc<str> = go_0(v31, v30, v12.clone());
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v32)), v14));
        let v35: u64 = v9.wrapping_shr((16i32) as u32);
        let v36: u64 = v35 & 281474976710655u64;
        let v37: i32 = 12i32;
        let v38: Rc<str> = go_0(v37, v36, v12.clone());
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v38));
        let v40: Rc<str> = Rc::<str>::from(v39.to_lowercase());
        let v41: Rc<str> = v40 ;
        let v43: Rc<str> = Rc::<str>::from(std::path::Path::new(v2.as_ref()).join(v41.as_ref()).display().to_string());
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_.root"); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = Rc::<str>::from(std::path::Path::new(v43.as_ref()).join(v45.as_ref()).display().to_string());
        let v47: i64 = 3i64;
        let v48: i32 = lay_1(v46.clone(), v47);
        let v49: Rc<str> = dtl_tree::dtl_tree_html(&*v43);
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v49));
        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v51, Rc::<str>::from(format!("{}{}", v53, v54))));
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div><details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v58, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v12)), v54))));
        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-got.html"); } LIT.with(|lit| lit.clone()) };
        let v63: i32 = same_4(v56.clone(), v61.clone(), v62.clone());
        let v64: i64 = dtl_tree::dtl_tree_bytes(&*v43);
        let v65: bool = v64 == 10i64;
        let v66: i32 = if v65 {
            0i32
        } else {
            3i32
        };
        let v68: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let mut v69: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
        let v71: bool = true; // 1;
        let mut v69 = v69;
        let v73: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        std::hash::Hasher::write_u128(&mut v69, v73);
        let v74: u64 = std::hash::Hasher::finish(&v69);
        std::hash::Hasher::write_u64(&mut v69, v74);
        let v75: u64 = std::hash::Hasher::finish(&v69);
        let v76: u64 = v74 & 4294967295u64;
        let v77: i32 = 8i32;
        let v78: Rc<str> = go_0(v77, v76, v12.clone());
        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v14));
        let v80: u64 = v74.wrapping_shr((32i32) as u32);
        let v81: u64 = v80 & 65535u64;
        let v82: i32 = 4i32;
        let v83: Rc<str> = go_0(v82, v81, v12.clone());
        let v85: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v79, v83)), v21));
        let v86: u64 = v74.wrapping_shr((48i32) as u32);
        let v87: u64 = v86 & 4095u64;
        let v88: i32 = 3i32;
        let v89: Rc<str> = go_0(v88, v87, v12.clone());
        let v91: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v85, v89)), v14));
        let v92: u64 = v75 & 16383u64;
        let v93: u64 = v92 | 32768u64;
        let v94: i32 = 4i32;
        let v95: Rc<str> = go_0(v94, v93, v12.clone());
        let v97: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v91, v95)), v14));
        let v98: u64 = v75.wrapping_shr((16i32) as u32);
        let v99: u64 = v98 & 281474976710655u64;
        let v100: i32 = 12i32;
        let v101: Rc<str> = go_0(v100, v99, v12.clone());
        let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v101));
        let v103: Rc<str> = Rc::<str>::from(v102.to_lowercase());
        let v104: Rc<str> = v103 ;
        let v106: Rc<str> = Rc::<str>::from(std::path::Path::new(v68.as_ref()).join(v104.as_ref()).display().to_string());
        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let v109: Rc<str> = Rc::<str>::from(std::path::Path::new(v106.as_ref()).join(v108.as_ref()).display().to_string());
        let v111: i32 = i32::from(std::fs::create_dir_all(v109.as_ref()).is_err());
        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.txt"); } LIT.with(|lit| lit.clone()) };
        let v114: Rc<str> = Rc::<str>::from(std::path::Path::new(v106.as_ref()).join(v113.as_ref()).display().to_string());
        let v115: i32 = 0i32;
        let v116: i32 = -1i32;
        let v117: i32 = last_sep_3(v114.clone(), v115, v116);
        let v118: bool = v117 == -1i32;
        let v128: Rc<str> = if v118 {
            v12.clone()
        } else {
            let v119: i32 = v117.wrapping_sub(1i32);
            let v120: Rc<str> = string_slice(&v114.clone(), 0i32 as i64, v119 as i64);
            let v121: i32 = (v120.clone().len() as i32);
            let v122: bool = v121 == 2i32;
            let v125: bool = if v122 {
                let v123: u8 = v120.clone().as_bytes()[1i32 as usize];
                let v124: bool = v123 == b':';
                v124
            } else {
                false
            };
            if v125 {
                let v126: Rc<str> = string_slice(&v114.clone(), 0i32 as i64, v117 as i64);
                v126.clone()
            } else {
                v120.clone()
            }
        };
        let v129: i32 = (v128.clone().len() as i32);
        let v130: bool = v129 == 0i32;
        let v133: i32 = if v130 {
            0i32
        } else {
            let v132: i32 = i32::from(std::fs::create_dir_all(v128.as_ref()).is_err());
            v132
        };
        let v134: bool = v133 == 0i32;
        let v138: i32 = if v134 {
            let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x"); } LIT.with(|lit| lit.clone()) };
            let v137: i32 = i32::from(std::fs::write(v114.as_ref(), v136.as_ref().as_bytes()).is_err());
            v137
        } else {
            v133
        };
        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.txt"); } LIT.with(|lit| lit.clone()) };
        let v141: Rc<str> = Rc::<str>::from(std::path::Path::new(v106.as_ref()).join(v140.as_ref()).display().to_string());
        let v142: i32 = 0i32;
        let v143: i32 = -1i32;
        let v144: i32 = last_sep_3(v141.clone(), v142, v143);
        let v145: bool = v144 == -1i32;
        let v155: Rc<str> = if v145 {
            v12.clone()
        } else {
            let v146: i32 = v144.wrapping_sub(1i32);
            let v147: Rc<str> = string_slice(&v141.clone(), 0i32 as i64, v146 as i64);
            let v148: i32 = (v147.clone().len() as i32);
            let v149: bool = v148 == 2i32;
            let v152: bool = if v149 {
                let v150: u8 = v147.clone().as_bytes()[1i32 as usize];
                let v151: bool = v150 == b':';
                v151
            } else {
                false
            };
            if v152 {
                let v153: Rc<str> = string_slice(&v141.clone(), 0i32 as i64, v144 as i64);
                v153.clone()
            } else {
                v147.clone()
            }
        };
        let v156: i32 = (v155.clone().len() as i32);
        let v157: bool = v156 == 0i32;
        let v160: i32 = if v157 {
            0i32
        } else {
            let v159: i32 = i32::from(std::fs::create_dir_all(v155.as_ref()).is_err());
            v159
        };
        let v161: bool = v160 == 0i32;
        let v165: i32 = if v161 {
            let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yy"); } LIT.with(|lit| lit.clone()) };
            let v164: i32 = i32::from(std::fs::write(v141.as_ref(), v163.as_ref().as_bytes()).is_err());
            v164
        } else {
            v160
        };
        let v167: Rc<str> = Rc::<str>::from(std::path::Path::new(v106.as_ref()).join(v108.as_ref()).display().to_string());
        let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
        let v170: Rc<str> = Rc::<str>::from(std::path::Path::new(v167.as_ref()).join(v169.as_ref()).display().to_string());
        let v171: i32 = 0i32;
        let v172: i32 = -1i32;
        let v173: i32 = last_sep_3(v170.clone(), v171, v172);
        let v174: bool = v173 == -1i32;
        let v184: Rc<str> = if v174 {
            v12.clone()
        } else {
            let v175: i32 = v173.wrapping_sub(1i32);
            let v176: Rc<str> = string_slice(&v170.clone(), 0i32 as i64, v175 as i64);
            let v177: i32 = (v176.clone().len() as i32);
            let v178: bool = v177 == 2i32;
            let v181: bool = if v178 {
                let v179: u8 = v176.clone().as_bytes()[1i32 as usize];
                let v180: bool = v179 == b':';
                v180
            } else {
                false
            };
            if v181 {
                let v182: Rc<str> = string_slice(&v170.clone(), 0i32 as i64, v173 as i64);
                v182.clone()
            } else {
                v176.clone()
            }
        };
        let v185: i32 = (v184.clone().len() as i32);
        let v186: bool = v185 == 0i32;
        let v189: i32 = if v186 {
            0i32
        } else {
            let v188: i32 = i32::from(std::fs::create_dir_all(v184.as_ref()).is_err());
            v188
        };
        let v190: bool = v189 == 0i32;
        let v194: i32 = if v190 {
            let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("z"); } LIT.with(|lit| lit.clone()) };
            let v193: i32 = i32::from(std::fs::write(v170.as_ref(), v192.as_ref().as_bytes()).is_err());
            v193
        } else {
            v189
        };
        let v195: Rc<str> = dtl_tree::dtl_tree_html(&*v106);
        let v196: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v195));
        let v199: Rc<str> = Rc::<str>::from(format!("{}{}", v196, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v12)), v54))));
        let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"m\">m</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"m/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"a.txt\">a.txt</a><span> (2.00 B)</span></div><div>&#128196; <a href=\"b.txt\">b.txt</a><span> (1.00 B)</span></div>"); } LIT.with(|lit| lit.clone()) };
        let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}\n  </style>\n</head>\n<body>\n  <div><details open=\"true\"><summary>&#128194; <a href=\"m\">m</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"m/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"a.txt\">a.txt</a><span> (2.00 B)</span></div><div>&#128196; <a href=\"b.txt\">b.txt</a><span> (1.00 B)</span></div>"); } LIT.with(|lit| lit.clone()) };
        let v204: Rc<str> = Rc::<str>::from(format!("{}{}", v201, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v12)), v54))));
        let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-order-got.html"); } LIT.with(|lit| lit.clone()) };
        let v206: i32 = same_4(v199.clone(), v204.clone(), v205.clone());
        let v208: Rc<str> = Rc::<str>::from(std::env::temp_dir().display().to_string());
        let mut v209: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
        let v211: bool = true; // 1;
        let mut v209 = v209;
        let v213: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        std::hash::Hasher::write_u128(&mut v209, v213);
        let v214: u64 = std::hash::Hasher::finish(&v209);
        std::hash::Hasher::write_u64(&mut v209, v214);
        let v215: u64 = std::hash::Hasher::finish(&v209);
        let v216: u64 = v214 & 4294967295u64;
        let v217: i32 = 8i32;
        let v218: Rc<str> = go_0(v217, v216, v12.clone());
        let v219: Rc<str> = Rc::<str>::from(format!("{}{}", v218, v14));
        let v220: u64 = v214.wrapping_shr((32i32) as u32);
        let v221: u64 = v220 & 65535u64;
        let v222: i32 = 4i32;
        let v223: Rc<str> = go_0(v222, v221, v12.clone());
        let v225: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v219, v223)), v21));
        let v226: u64 = v214.wrapping_shr((48i32) as u32);
        let v227: u64 = v226 & 4095u64;
        let v228: i32 = 3i32;
        let v229: Rc<str> = go_0(v228, v227, v12.clone());
        let v231: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v225, v229)), v14));
        let v232: u64 = v215 & 16383u64;
        let v233: u64 = v232 | 32768u64;
        let v234: i32 = 4i32;
        let v235: Rc<str> = go_0(v234, v233, v12.clone());
        let v237: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v231, v235)), v14));
        let v238: u64 = v215.wrapping_shr((16i32) as u32);
        let v239: u64 = v238 & 281474976710655u64;
        let v240: i32 = 12i32;
        let v241: Rc<str> = go_0(v240, v239, v12.clone());
        let v242: Rc<str> = Rc::<str>::from(format!("{}{}", v237, v241));
        let v243: Rc<str> = Rc::<str>::from(v242.to_lowercase());
        let v244: Rc<str> = v243 ;
        let v246: Rc<str> = Rc::<str>::from(std::path::Path::new(v208.as_ref()).join(v244.as_ref()).display().to_string());
        let v248: i32 = i32::from(std::fs::create_dir_all(v246.as_ref()).is_err());
        let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
        let v251: Rc<str> = Rc::<str>::from(std::path::Path::new(v246.as_ref()).join(v250.as_ref()).display().to_string());
        let v252: i32 = 0i32;
        let v253: i32 = -1i32;
        let v254: i32 = last_sep_3(v251.clone(), v252, v253);
        let v255: bool = v254 == -1i32;
        let v265: Rc<str> = if v255 {
            v12.clone()
        } else {
            let v256: i32 = v254.wrapping_sub(1i32);
            let v257: Rc<str> = string_slice(&v251.clone(), 0i32 as i64, v256 as i64);
            let v258: i32 = (v257.clone().len() as i32);
            let v259: bool = v258 == 2i32;
            let v262: bool = if v259 {
                let v260: u8 = v257.clone().as_bytes()[1i32 as usize];
                let v261: bool = v260 == b':';
                v261
            } else {
                false
            };
            if v262 {
                let v263: Rc<str> = string_slice(&v251.clone(), 0i32 as i64, v254 as i64);
                v263.clone()
            } else {
                v257.clone()
            }
        };
        let v266: i32 = (v265.clone().len() as i32);
        let v267: bool = v266 == 0i32;
        let v270: i32 = if v267 {
            0i32
        } else {
            let v269: i32 = i32::from(std::fs::create_dir_all(v265.as_ref()).is_err());
            v269
        };
        let v271: bool = v270 == 0i32;
        let v275: i32 = if v271 {
            let v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# Title\n\nSome `code` and [a link](x.html).\n\n- one\n- two <b>\n\n봉쥬르 & é [깃허브](k.html)\n\nHi! ![logo](logo.png) [![CI](b.svg)](ci.html)\n\n**Bold** and <https://x.org> and \\*star\\*\n\n---\n\n- top\n  - nested\n\n```\nlet x = 1 < 2\n```\n\n| A | B |\n| :- | -: |\n| `x` | y \\| z |\n\n3. three\n   wrapped\n4. four\n- lazy\nmore\n"); } LIT.with(|lit| lit.clone()) };
            let v274: i32 = i32::from(std::fs::write(v251.as_ref(), v273.as_ref().as_bytes()).is_err());
            v274
        } else {
            v270
        };
        let v277: Rc<str> = Rc::<str>::from(std::path::Path::new(v246.as_ref()).join(v250.as_ref()).display().to_string());
        let v279: bool = std::path::Path::new(v277.as_ref()).is_file();
        let v318: Rc<str> = if v279 {
            let v281: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v277);
            let mut v282: Rc<dyn Fn(Vec<u8>) -> US0> = method5();
            let mut v283: Rc<dyn Fn(std::io::Error) -> US0> = method6();
            let v284: US0 = match v281 { Ok(x) => v282(x), Err(e) => v283(e) };
            let v312: Rc<str> = match v284 {
                US0::US0_Error(v285) => {
                    let v285: std::io::Error = v285;
                    v12.clone()
                }
                US0::US0_Ok(v286) => {
                    let v286: Vec<u8> = v286.clone();
                    let v287: Vec<u8> = method7(v286.clone());
                    let v289: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v287);
                    let mut v290: Rc<dyn Fn(std::string::String) -> US1> = method8();
                    let mut v291: Rc<dyn Fn(std::string::FromUtf8Error) -> US1> = method9();
                    let v292: US1 = match v289 { Ok(x) => v290(x), Err(e) => v291(e) };
                    match &v292 {
                        US1::US1_Error(v293) => {
                            let v293: std::string::FromUtf8Error = v293.clone();
                            v12.clone()
                        }
                        US1::US1_Ok(v294) => {
                            let v294: std::string::String = v294.clone();
                            let v296: Rc<str> = Rc::<str>::from(String::as_str(&v294));
                            let v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
                            let v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v299: Rc<str> = Rc::<str>::from(v296.replace(&*v297, &*v298));
                            let v300: bool = v299.ends_with(&*v298);
                            let v303: bool = if v300 {
                                let v301: i32 = (v299.clone().len() as i32);
                                let v302: bool = v301 == 1i32;
                                v302
                            } else {
                                false
                            };
                            if v303 {
                                v12.clone()
                            } else {
                                if v300 {
                                    let v304: i32 = (v299.clone().len() as i32);
                                    let v305: i32 = v304.wrapping_sub(2i32);
                                    let v306: Rc<str> = string_slice(&v299.clone(), 0i32 as i64, v305 as i64);
                                    v306.clone()
                                } else {
                                    v299.clone()
                                }
                            }
                        }
                    }
                }
            };
            let v313: Rc<str> = dtl_markdown::dtl_markdown(&*v312);
            let v314: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
            let v315: Rc<str> = Rc::<str>::from(format!("{}{}", v314, v313));
            let v316: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
            let v317: Rc<str> = Rc::<str>::from(format!("{}{}", v315, v316));
            v317.clone()
        } else {
            v12.clone()
        };
        let v319: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n<h1>Title</h1>\n<p>Some <code>code</code> and <a href=\"x.html\">a link</a>.</p>\n<ul>\n<li>one</li>\n<li>two &lt;b&gt;</li>\n</ul>\n<p>봉쥬르 &amp; é <a href=\"k.html\">깃허브</a></p>\n<p>Hi! <img src=\"logo.png\" alt=\"logo\"> <a href=\"ci.html\"><img src=\"b.svg\" alt=\"CI\"></a></p>\n<p><strong>Bold</strong> and <a href=\"https://x.org\">https://x.org</a> and *star*</p>\n<hr>\n<ul>\n<li>top</li>\n<li>nested</li>\n</ul>\n<pre><code>let x = 1 &lt; 2\n</code></pre>\n<table>\n<thead>\n<tr><th style=\"text-align: left\">A</th><th style=\"text-align: right\">B</th></tr>\n</thead>\n<tbody>\n<tr><td style=\"text-align: left\"><code>x</code></td><td style=\"text-align: right\">y | z</td></tr>\n</tbody>\n</table>\n<ol start=\"3\">\n<li>three wrapped</li>\n<li>four</li>\n</ol>\n<ul>\n<li>lazy more</li>\n</ul>\n</section>\n"); } LIT.with(|lit| lit.clone()) };
        let v320: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-readme-got.html"); } LIT.with(|lit| lit.clone()) };
        let v321: i32 = same_4(v318.clone(), v319.clone(), v320.clone());
        let v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>r</p>\n"); } LIT.with(|lit| lit.clone()) };
        let v324: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v52, v322)), v54));
        let v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n<p>r</p>\n</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
        let v326: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir-tree-html-close-got.html"); } LIT.with(|lit| lit.clone()) };
        let v327: i32 = same_4(v324.clone(), v325.clone(), v326.clone());
        let v328: Rc<str> = dtl_tree::dtl_size_label(1024i64);
        let v329: bool = &*v328 == "1024.00 B";
        let v339: i32 = if v329 {
            let v330: Rc<str> = dtl_tree::dtl_size_label(1025i64);
            let v331: bool = &*v330 == "1.00 KB";
            if v331 {
                let v332: Rc<str> = dtl_tree::dtl_size_label(1048576i64);
                let v333: bool = &*v332 == "1024.00 KB";
                if v333 {
                    let v334: Rc<str> = dtl_tree::dtl_size_label(1048577i64);
                    let v335: bool = &*v334 == "1.00 MB";
                    if v335 {
                        0i32
                    } else {
                        1i32
                    }
                } else {
                    1i32
                }
            } else {
                1i32
            }
        } else {
            1i32
        };
        let v340: bool = v63 == 0i32;
        if v340 {
            let v341: bool = v66 == 0i32;
            if v341 {
                let v342: bool = v206 == 0i32;
                if v342 {
                    let v343: bool = v321 == 0i32;
                    if v343 {
                        let v344: bool = v327 == 0i32;
                        if v344 {
                            let v345: i32 = viewers_test_10();
                            let v346: bool = v345 == 0i32;
                            if v346 {
                                v339
                            } else {
                                6i32
                            }
                        } else {
                            5i32
                        }
                    } else {
                        4i32
                    }
                } else {
                    v206
                }
            } else {
                v66
            }
        } else {
            v63
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_self_test(v0: i32) -> i32 {
    closure0()(v0)
}
