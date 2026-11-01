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
fn escape_from_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 == v4;
        if v5 {
            let v6: bool = v2 == v1;
            let v10: Rc<str> = if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7.clone()
            } else {
                let v8: i32 = v1.wrapping_sub(1i32);
                let v9: Rc<str> = string_slice(&v0.clone(), v2 as i64, v8 as i64);
                v9.clone()
            };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v10));
            return v11.clone();
        } else {
            let v12: u8 = v0.clone().as_bytes()[v1 as usize];
            let v13: bool = v12 == b'&';
            let v25: Rc<str> = if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&amp;"); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                let v15: bool = v12 == b'<';
                if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&lt;"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let v17: bool = v12 == b'>';
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&gt;"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let v19: bool = v12 == b'"';
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&quot;"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        }
                    }
                }
            };
            let v26: bool = &*v25 == "";
            if v26 {
                let v27: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v27, v2, v3.clone());
                continue;
            } else {
                let v29: i32 = v1.wrapping_add(1i32);
                let v30: bool = v2 == v1;
                let v34: Rc<str> = if v30 {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v31.clone()
                } else {
                    let v32: i32 = v1.wrapping_sub(1i32);
                    let v33: Rc<str> = string_slice(&v0.clone(), v2 as i64, v32 as i64);
                    v33.clone()
                };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v34)), v25));
                (v0, v1, v2, v3) = (v0.clone(), v29, v29, v36.clone());
                continue;
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: i32 = 0i32;
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        escape_from_0(v0.clone(), v1, v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn find_char_from_2(mut v0: Rc<str>, mut v1: u8, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v2 == v3;
        if v4 {
            return -1i32;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v2 as usize];
            let v6: bool = v5 == v1;
            if v6 {
                return v2;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v7);
                continue;
            }
        }
    }
}
fn next_inline_marker_from_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'`';
            if v5 {
                return v1;
            } else {
                let v6: bool = v4 == b'[';
                if v6 {
                    return v1;
                } else {
                    let v7: bool = v4 == b'!';
                    if v7 {
                        return v1;
                    } else {
                        let v8: bool = v4 == b'*';
                        if v8 {
                            return v1;
                        } else {
                            let v9: bool = v4 == b'<';
                            if v9 {
                                return v1;
                            } else {
                                let v10: bool = v4 == b'\\';
                                if v10 {
                                    return v1;
                                } else {
                                    let v11: i32 = v1.wrapping_add(1i32);
                                    (v0, v1) = (v0.clone(), v11);
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
fn double_close_4(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 == v2;
        if v3 {
            return -1i32;
        } else {
            let v4: i32 = v1.wrapping_add(1i32);
            let v5: bool = v4 == v2;
            if v5 {
                return -1i32;
            } else {
                let v6: u8 = v0.clone().as_bytes()[v1 as usize];
                let v7: bool = v6 == b'*';
                if v7 {
                    let v8: u8 = v0.clone().as_bytes()[v4 as usize];
                    let v9: bool = v8 == b'*';
                    if v9 {
                        return v1;
                    } else {
                        (v0, v1) = (v0.clone(), v4);
                        continue;
                    }
                } else {
                    (v0, v1) = (v0.clone(), v4);
                    continue;
                }
            }
        }
    }
}
fn image_close_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: u8 = v0.clone().as_bytes()[v1 as usize];
    let v4: bool = v3 == b'!';
    if v4 {
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: bool = v5 == v2;
        if v6 {
            -1i32
        } else {
            let v7: u8 = v0.clone().as_bytes()[v5 as usize];
            let v8: bool = v7 == b'[';
            if v8 {
                let v9: u8 = b']';
                let v10: i32 = v1.wrapping_add(2i32);
                let v11: i32 = find_char_from_2(v0.clone(), v9, v10);
                let v12: bool = v11 == -1i32;
                if v12 {
                    -1i32
                } else {
                    let v13: i32 = v11.wrapping_add(1i32);
                    let v14: bool = v13 == v2;
                    if v14 {
                        -1i32
                    } else {
                        let v15: u8 = v0.clone().as_bytes()[v13 as usize];
                        let v16: bool = v15 == b'(';
                        if v16 {
                            let v17: u8 = b')';
                            let v18: i32 = v11.wrapping_add(2i32);
                            find_char_from_2(v0, v17, v18)
                        } else {
                            -1i32
                        }
                    }
                }
            } else {
                -1i32
            }
        }
    } else {
        -1i32
    }
}
fn badge_close_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: u8 = v0.clone().as_bytes()[v1 as usize];
    let v4: bool = v3 == b'[';
    if v4 {
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: bool = v5 == v2;
        if v6 {
            -1i32
        } else {
            let v7: i32 = image_close_5(v0.clone(), v5);
            let v8: bool = v7 == -1i32;
            if v8 {
                -1i32
            } else {
                let v9: i32 = v7.wrapping_add(1i32);
                let v10: bool = v9 == v2;
                if v10 {
                    -1i32
                } else {
                    let v11: u8 = v0.clone().as_bytes()[v9 as usize];
                    let v12: bool = v11 == b']';
                    if v12 {
                        let v13: i32 = v7.wrapping_add(2i32);
                        let v14: bool = v13 == v2;
                        if v14 {
                            -1i32
                        } else {
                            let v15: u8 = v0.clone().as_bytes()[v13 as usize];
                            let v16: bool = v15 == b'(';
                            if v16 {
                                let v17: u8 = b')';
                                let v18: i32 = v7.wrapping_add(3i32);
                                find_char_from_2(v0, v17, v18)
                            } else {
                                -1i32
                            }
                        }
                    } else {
                        -1i32
                    }
                }
            }
        }
    } else {
        -1i32
    }
}
fn inline_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 == v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'`';
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                let v8: i32 = find_char_from_2(v0.clone(), v5, v7);
                let v9: bool = v8 == -1i32;
                if v9 {
                    let v10: i32 = next_inline_marker_from_3(v0.clone(), v7);
                    let v11: bool = v1 == v10;
                    let v15: Rc<str> = if v11 {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v12.clone()
                    } else {
                        let v13: i32 = v10.wrapping_sub(1i32);
                        let v14: Rc<str> = string_slice(&v0.clone(), v1 as i64, v13 as i64);
                        v14.clone()
                    };
                    let v16: i32 = 0i32;
                    let v17: i32 = 0i32;
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = escape_from_0(v15.clone(), v16, v17, v18.clone());
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v19));
                    (v0, v1, v2) = (v0.clone(), v10, v20.clone());
                    continue;
                } else {
                    let v22: i32 = v8.wrapping_add(1i32);
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<code>"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v23));
                    let v25: i32 = v8.wrapping_sub(1i32);
                    let v26: Rc<str> = string_slice(&v0.clone(), v7 as i64, v25 as i64);
                    let v27: i32 = 0i32;
                    let v28: i32 = 0i32;
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = escape_from_0(v26.clone(), v27, v28, v29.clone());
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v30));
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code>"); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    (v0, v1, v2) = (v0.clone(), v22, v33.clone());
                    continue;
                }
            } else {
                let v36: bool = v5 == b'\\';
                if v36 {
                    let v37: i32 = v1.wrapping_add(1i32);
                    let v38: bool = v37 == v3;
                    if v38 {
                        let v39: i32 = next_inline_marker_from_3(v0.clone(), v37);
                        let v40: bool = v1 == v39;
                        let v44: Rc<str> = if v40 {
                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v41.clone()
                        } else {
                            let v42: i32 = v39.wrapping_sub(1i32);
                            let v43: Rc<str> = string_slice(&v0.clone(), v1 as i64, v42 as i64);
                            v43.clone()
                        };
                        let v45: i32 = 0i32;
                        let v46: i32 = 0i32;
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = escape_from_0(v44.clone(), v45, v46, v47.clone());
                        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v48));
                        (v0, v1, v2) = (v0.clone(), v39, v49.clone());
                        continue;
                    } else {
                        let v51: u8 = v0.clone().as_bytes()[v37 as usize];
                        let v52: bool = v51 == b'*';
                        let v72: bool = if v52 {
                            true
                        } else {
                            let v53: bool = v51 == b'_';
                            if v53 {
                                true
                            } else {
                                let v54: bool = v51 == b'\\';
                                if v54 {
                                    true
                                } else {
                                    let v55: bool = v51 == b'`';
                                    if v55 {
                                        true
                                    } else {
                                        let v56: bool = v51 == b'[';
                                        if v56 {
                                            true
                                        } else {
                                            let v57: bool = v51 == b']';
                                            if v57 {
                                                true
                                            } else {
                                                let v58: bool = v51 == b'#';
                                                if v58 {
                                                    true
                                                } else {
                                                    let v59: bool = v51 == b'-';
                                                    if v59 {
                                                        true
                                                    } else {
                                                        let v60: bool = v51 == b'!';
                                                        if v60 {
                                                            true
                                                        } else {
                                                            let v61: bool = v51 == b'<';
                                                            if v61 {
                                                                true
                                                            } else {
                                                                let v62: bool = v51 == b'|';
                                                                v62
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
                        if v72 {
                            let v73: i32 = v1.wrapping_add(2i32);
                            let v74: bool = v37 == v73;
                            let v78: Rc<str> = if v74 {
                                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v75.clone()
                            } else {
                                let v76: i32 = v73.wrapping_sub(1i32);
                                let v77: Rc<str> = string_slice(&v0.clone(), v37 as i64, v76 as i64);
                                v77.clone()
                            };
                            let v79: i32 = 0i32;
                            let v80: i32 = 0i32;
                            let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v82: Rc<str> = escape_from_0(v78.clone(), v79, v80, v81.clone());
                            let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v82));
                            (v0, v1, v2) = (v0.clone(), v73, v83.clone());
                            continue;
                        } else {
                            let v85: i32 = next_inline_marker_from_3(v0.clone(), v37);
                            let v86: bool = v1 == v85;
                            let v90: Rc<str> = if v86 {
                                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v87.clone()
                            } else {
                                let v88: i32 = v85.wrapping_sub(1i32);
                                let v89: Rc<str> = string_slice(&v0.clone(), v1 as i64, v88 as i64);
                                v89.clone()
                            };
                            let v91: i32 = 0i32;
                            let v92: i32 = 0i32;
                            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v94: Rc<str> = escape_from_0(v90.clone(), v91, v92, v93.clone());
                            let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v94));
                            (v0, v1, v2) = (v0.clone(), v85, v95.clone());
                            continue;
                        }
                    }
                } else {
                    let v99: bool = v5 == b'*';
                    if v99 {
                        let v100: i32 = v1.wrapping_add(1i32);
                        let v101: bool = v100 == v3;
                        if v101 {
                            let v102: i32 = next_inline_marker_from_3(v0.clone(), v100);
                            let v103: bool = v1 == v102;
                            let v107: Rc<str> = if v103 {
                                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v104.clone()
                            } else {
                                let v105: i32 = v102.wrapping_sub(1i32);
                                let v106: Rc<str> = string_slice(&v0.clone(), v1 as i64, v105 as i64);
                                v106.clone()
                            };
                            let v108: i32 = 0i32;
                            let v109: i32 = 0i32;
                            let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v111: Rc<str> = escape_from_0(v107.clone(), v108, v109, v110.clone());
                            let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v111));
                            (v0, v1, v2) = (v0.clone(), v102, v112.clone());
                            continue;
                        } else {
                            let v114: u8 = v0.clone().as_bytes()[v100 as usize];
                            let v115: bool = v114 == b'*';
                            if v115 {
                                let v116: i32 = v1.wrapping_add(2i32);
                                let v117: i32 = double_close_4(v0.clone(), v116);
                                let v118: bool = v117 == -1i32;
                                if v118 {
                                    let v119: i32 = next_inline_marker_from_3(v0.clone(), v100);
                                    let v120: bool = v1 == v119;
                                    let v124: Rc<str> = if v120 {
                                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v121.clone()
                                    } else {
                                        let v122: i32 = v119.wrapping_sub(1i32);
                                        let v123: Rc<str> = string_slice(&v0.clone(), v1 as i64, v122 as i64);
                                        v123.clone()
                                    };
                                    let v125: i32 = 0i32;
                                    let v126: i32 = 0i32;
                                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v128: Rc<str> = escape_from_0(v124.clone(), v125, v126, v127.clone());
                                    let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v128));
                                    (v0, v1, v2) = (v0.clone(), v119, v129.clone());
                                    continue;
                                } else {
                                    let v131: i32 = v117.wrapping_add(2i32);
                                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<strong>"); } LIT.with(|lit| lit.clone()) };
                                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v132));
                                    let v134: bool = v116 == v117;
                                    let v138: Rc<str> = if v134 {
                                        let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v135.clone()
                                    } else {
                                        let v136: i32 = v117.wrapping_sub(1i32);
                                        let v137: Rc<str> = string_slice(&v0.clone(), v116 as i64, v136 as i64);
                                        v137.clone()
                                    };
                                    let v139: i32 = 0i32;
                                    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v141: Rc<str> = inline_from_1(v138.clone(), v139, v140.clone());
                                    let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v141));
                                    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</strong>"); } LIT.with(|lit| lit.clone()) };
                                    let v144: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v143));
                                    (v0, v1, v2) = (v0.clone(), v131, v144.clone());
                                    continue;
                                }
                            } else {
                                let v147: i32 = next_inline_marker_from_3(v0.clone(), v100);
                                let v148: bool = v1 == v147;
                                let v152: Rc<str> = if v148 {
                                    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v149.clone()
                                } else {
                                    let v150: i32 = v147.wrapping_sub(1i32);
                                    let v151: Rc<str> = string_slice(&v0.clone(), v1 as i64, v150 as i64);
                                    v151.clone()
                                };
                                let v153: i32 = 0i32;
                                let v154: i32 = 0i32;
                                let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v156: Rc<str> = escape_from_0(v152.clone(), v153, v154, v155.clone());
                                let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v156));
                                (v0, v1, v2) = (v0.clone(), v147, v157.clone());
                                continue;
                            }
                        }
                    } else {
                        let v161: bool = v5 == b'<';
                        if v161 {
                            let v162: u8 = b'>';
                            let v163: i32 = v1.wrapping_add(1i32);
                            let v164: i32 = find_char_from_2(v0.clone(), v162, v163);
                            let v165: bool = v164 == -1i32;
                            if v165 {
                                let v166: i32 = next_inline_marker_from_3(v0.clone(), v163);
                                let v167: bool = v1 == v166;
                                let v171: Rc<str> = if v167 {
                                    let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v168.clone()
                                } else {
                                    let v169: i32 = v166.wrapping_sub(1i32);
                                    let v170: Rc<str> = string_slice(&v0.clone(), v1 as i64, v169 as i64);
                                    v170.clone()
                                };
                                let v172: i32 = 0i32;
                                let v173: i32 = 0i32;
                                let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v175: Rc<str> = escape_from_0(v171.clone(), v172, v173, v174.clone());
                                let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v175));
                                (v0, v1, v2) = (v0.clone(), v166, v176.clone());
                                continue;
                            } else {
                                let v178: i32 = v1.wrapping_add(5i32);
                                let v179: bool = v164 > v178;
                                if v179 {
                                    let v180: bool = v163 == v164;
                                    let v184: Rc<str> = if v180 {
                                        let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v181.clone()
                                    } else {
                                        let v182: i32 = v164.wrapping_sub(1i32);
                                        let v183: Rc<str> = string_slice(&v0.clone(), v163 as i64, v182 as i64);
                                        v183.clone()
                                    };
                                    let v185: Rc<str> = string_slice(&v184.clone(), 0i32 as i64, 3i32 as i64);
                                    let v186: bool = &*v185 == "http";
                                    if v186 {
                                        let v187: i32 = v164.wrapping_add(1i32);
                                        let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
                                        let v189: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v188));
                                        let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                        let v191: Rc<str> = Rc::<str>::from(format!("{}{}", v189, v190));
                                        let v192: i32 = 0i32;
                                        let v193: i32 = 0i32;
                                        let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v195: Rc<str> = escape_from_0(v184.clone(), v192, v193, v194.clone());
                                        let v197: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v191, v195)), v190));
                                        let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                        let v199: Rc<str> = Rc::<str>::from(format!("{}{}", v197, v198));
                                        let v200: i32 = 0i32;
                                        let v201: i32 = 0i32;
                                        let v202: Rc<str> = escape_from_0(v184.clone(), v200, v201, v194.clone());
                                        let v203: Rc<str> = Rc::<str>::from(format!("{}{}", v199, v202));
                                        let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
                                        let v205: Rc<str> = Rc::<str>::from(format!("{}{}", v203, v204));
                                        (v0, v1, v2) = (v0.clone(), v187, v205.clone());
                                        continue;
                                    } else {
                                        let v207: i32 = next_inline_marker_from_3(v0.clone(), v163);
                                        let v208: bool = v1 == v207;
                                        let v212: Rc<str> = if v208 {
                                            let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v209.clone()
                                        } else {
                                            let v210: i32 = v207.wrapping_sub(1i32);
                                            let v211: Rc<str> = string_slice(&v0.clone(), v1 as i64, v210 as i64);
                                            v211.clone()
                                        };
                                        let v213: i32 = 0i32;
                                        let v214: i32 = 0i32;
                                        let v215: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v216: Rc<str> = escape_from_0(v212.clone(), v213, v214, v215.clone());
                                        let v217: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v216));
                                        (v0, v1, v2) = (v0.clone(), v207, v217.clone());
                                        continue;
                                    }
                                } else {
                                    let v220: i32 = next_inline_marker_from_3(v0.clone(), v163);
                                    let v221: bool = v1 == v220;
                                    let v225: Rc<str> = if v221 {
                                        let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v222.clone()
                                    } else {
                                        let v223: i32 = v220.wrapping_sub(1i32);
                                        let v224: Rc<str> = string_slice(&v0.clone(), v1 as i64, v223 as i64);
                                        v224.clone()
                                    };
                                    let v226: i32 = 0i32;
                                    let v227: i32 = 0i32;
                                    let v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v229: Rc<str> = escape_from_0(v225.clone(), v226, v227, v228.clone());
                                    let v230: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v229));
                                    (v0, v1, v2) = (v0.clone(), v220, v230.clone());
                                    continue;
                                }
                            }
                        } else {
                            let v234: bool = v5 == b'!';
                            if v234 {
                                let v235: i32 = image_close_5(v0.clone(), v1);
                                let v236: bool = v235 == -1i32;
                                if v236 {
                                    let v237: i32 = v1.wrapping_add(1i32);
                                    let v238: i32 = next_inline_marker_from_3(v0.clone(), v237);
                                    let v239: bool = v1 == v238;
                                    let v243: Rc<str> = if v239 {
                                        let v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v240.clone()
                                    } else {
                                        let v241: i32 = v238.wrapping_sub(1i32);
                                        let v242: Rc<str> = string_slice(&v0.clone(), v1 as i64, v241 as i64);
                                        v242.clone()
                                    };
                                    let v244: i32 = 0i32;
                                    let v245: i32 = 0i32;
                                    let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v247: Rc<str> = escape_from_0(v243.clone(), v244, v245, v246.clone());
                                    let v248: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v247));
                                    (v0, v1, v2) = (v0.clone(), v238, v248.clone());
                                    continue;
                                } else {
                                    let v250: i32 = v235.wrapping_add(1i32);
                                    let v251: u8 = b']';
                                    let v252: i32 = v1.wrapping_add(2i32);
                                    let v253: i32 = find_char_from_2(v0.clone(), v251, v252);
                                    let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src="); } LIT.with(|lit| lit.clone()) };
                                    let v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                    let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src=\""); } LIT.with(|lit| lit.clone()) };
                                    let v257: i32 = v253.wrapping_add(2i32);
                                    let v258: bool = v257 == v235;
                                    let v262: Rc<str> = if v258 {
                                        let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v259.clone()
                                    } else {
                                        let v260: i32 = v235.wrapping_sub(1i32);
                                        let v261: Rc<str> = string_slice(&v0.clone(), v257 as i64, v260 as i64);
                                        v261.clone()
                                    };
                                    let v263: i32 = 0i32;
                                    let v264: i32 = 0i32;
                                    let v265: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v266: Rc<str> = escape_from_0(v262.clone(), v263, v264, v265.clone());
                                    let v268: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v256, v266)), v255));
                                    let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" alt="); } LIT.with(|lit| lit.clone()) };
                                    let v271: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v268, v269)), v255));
                                    let v272: bool = v252 == v253;
                                    let v275: Rc<str> = if v272 {
                                        v265.clone()
                                    } else {
                                        let v273: i32 = v253.wrapping_sub(1i32);
                                        let v274: Rc<str> = string_slice(&v0.clone(), v252 as i64, v273 as i64);
                                        v274.clone()
                                    };
                                    let v276: i32 = 0i32;
                                    let v277: i32 = 0i32;
                                    let v278: Rc<str> = escape_from_0(v275.clone(), v276, v277, v265.clone());
                                    let v280: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v271, v278)), v255));
                                    let v281: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                    let v283: Rc<str> = Rc::<str>::from(format!("{}{}", v2, Rc::<str>::from(format!("{}{}", v280, v281))));
                                    (v0, v1, v2) = (v0.clone(), v250, v283.clone());
                                    continue;
                                }
                            } else {
                                let v286: i32 = badge_close_6(v0.clone(), v1);
                                let v287: bool = v286 == -1i32;
                                if v287 {
                                    let v288: bool = v5 == b'[';
                                    if v288 {
                                        let v289: u8 = b']';
                                        let v290: i32 = v1.wrapping_add(1i32);
                                        let v291: i32 = find_char_from_2(v0.clone(), v289, v290);
                                        let v292: bool = v291 == -1i32;
                                        if v292 {
                                            let v293: i32 = next_inline_marker_from_3(v0.clone(), v290);
                                            let v294: bool = v1 == v293;
                                            let v298: Rc<str> = if v294 {
                                                let v295: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                v295.clone()
                                            } else {
                                                let v296: i32 = v293.wrapping_sub(1i32);
                                                let v297: Rc<str> = string_slice(&v0.clone(), v1 as i64, v296 as i64);
                                                v297.clone()
                                            };
                                            let v299: i32 = 0i32;
                                            let v300: i32 = 0i32;
                                            let v301: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v302: Rc<str> = escape_from_0(v298.clone(), v299, v300, v301.clone());
                                            let v303: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v302));
                                            (v0, v1, v2) = (v0.clone(), v293, v303.clone());
                                            continue;
                                        } else {
                                            let v305: i32 = v291.wrapping_add(1i32);
                                            let v306: bool = v305 == v3;
                                            if v306 {
                                                let v307: i32 = next_inline_marker_from_3(v0.clone(), v290);
                                                let v308: bool = v1 == v307;
                                                let v312: Rc<str> = if v308 {
                                                    let v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    v309.clone()
                                                } else {
                                                    let v310: i32 = v307.wrapping_sub(1i32);
                                                    let v311: Rc<str> = string_slice(&v0.clone(), v1 as i64, v310 as i64);
                                                    v311.clone()
                                                };
                                                let v313: i32 = 0i32;
                                                let v314: i32 = 0i32;
                                                let v315: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let v316: Rc<str> = escape_from_0(v312.clone(), v313, v314, v315.clone());
                                                let v317: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v316));
                                                (v0, v1, v2) = (v0.clone(), v307, v317.clone());
                                                continue;
                                            } else {
                                                let v319: u8 = v0.clone().as_bytes()[v305 as usize];
                                                let v320: bool = v319 == b'(';
                                                if v320 {
                                                    let v321: u8 = b')';
                                                    let v322: i32 = v291.wrapping_add(2i32);
                                                    let v323: i32 = find_char_from_2(v0.clone(), v321, v322);
                                                    let v324: bool = v323 == -1i32;
                                                    if v324 {
                                                        let v325: i32 = next_inline_marker_from_3(v0.clone(), v290);
                                                        let v326: bool = v1 == v325;
                                                        let v330: Rc<str> = if v326 {
                                                            let v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            v327.clone()
                                                        } else {
                                                            let v328: i32 = v325.wrapping_sub(1i32);
                                                            let v329: Rc<str> = string_slice(&v0.clone(), v1 as i64, v328 as i64);
                                                            v329.clone()
                                                        };
                                                        let v331: i32 = 0i32;
                                                        let v332: i32 = 0i32;
                                                        let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        let v334: Rc<str> = escape_from_0(v330.clone(), v331, v332, v333.clone());
                                                        let v335: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v334));
                                                        (v0, v1, v2) = (v0.clone(), v325, v335.clone());
                                                        continue;
                                                    } else {
                                                        let v337: i32 = v291.wrapping_sub(1i32);
                                                        let v338: Rc<str> = string_slice(&v0.clone(), v290 as i64, v337 as i64);
                                                        let v339: i32 = v323.wrapping_sub(1i32);
                                                        let v340: Rc<str> = string_slice(&v0.clone(), v322 as i64, v339 as i64);
                                                        let v341: i32 = v323.wrapping_add(1i32);
                                                        let v342: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
                                                        let v343: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v342));
                                                        let v344: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                        let v345: Rc<str> = Rc::<str>::from(format!("{}{}", v343, v344));
                                                        let v346: i32 = 0i32;
                                                        let v347: i32 = 0i32;
                                                        let v348: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        let v349: Rc<str> = escape_from_0(v340.clone(), v346, v347, v348.clone());
                                                        let v351: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v345, v349)), v344));
                                                        let v352: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                                        let v353: Rc<str> = Rc::<str>::from(format!("{}{}", v351, v352));
                                                        let v354: i32 = 0i32;
                                                        let v355: i32 = 0i32;
                                                        let v356: Rc<str> = escape_from_0(v338.clone(), v354, v355, v348.clone());
                                                        let v357: Rc<str> = Rc::<str>::from(format!("{}{}", v353, v356));
                                                        let v358: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
                                                        let v359: Rc<str> = Rc::<str>::from(format!("{}{}", v357, v358));
                                                        (v0, v1, v2) = (v0.clone(), v341, v359.clone());
                                                        continue;
                                                    }
                                                } else {
                                                    let v362: i32 = next_inline_marker_from_3(v0.clone(), v290);
                                                    let v363: bool = v1 == v362;
                                                    let v367: Rc<str> = if v363 {
                                                        let v364: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v364.clone()
                                                    } else {
                                                        let v365: i32 = v362.wrapping_sub(1i32);
                                                        let v366: Rc<str> = string_slice(&v0.clone(), v1 as i64, v365 as i64);
                                                        v366.clone()
                                                    };
                                                    let v368: i32 = 0i32;
                                                    let v369: i32 = 0i32;
                                                    let v370: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    let v371: Rc<str> = escape_from_0(v367.clone(), v368, v369, v370.clone());
                                                    let v372: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v371));
                                                    (v0, v1, v2) = (v0.clone(), v362, v372.clone());
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        let v377: i32 = v1.wrapping_add(1i32);
                                        let v378: i32 = next_inline_marker_from_3(v0.clone(), v377);
                                        let v379: bool = v1 == v378;
                                        let v383: Rc<str> = if v379 {
                                            let v380: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v380.clone()
                                        } else {
                                            let v381: i32 = v378.wrapping_sub(1i32);
                                            let v382: Rc<str> = string_slice(&v0.clone(), v1 as i64, v381 as i64);
                                            v382.clone()
                                        };
                                        let v384: i32 = 0i32;
                                        let v385: i32 = 0i32;
                                        let v386: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v387: Rc<str> = escape_from_0(v383.clone(), v384, v385, v386.clone());
                                        let v388: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v387));
                                        (v0, v1, v2) = (v0.clone(), v378, v388.clone());
                                        continue;
                                    }
                                } else {
                                    let v391: i32 = badge_close_6(v0.clone(), v1);
                                    let v392: i32 = v1.wrapping_add(1i32);
                                    let v393: i32 = image_close_5(v0.clone(), v392);
                                    let v394: i32 = v391.wrapping_add(1i32);
                                    let v395: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
                                    let v396: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v395));
                                    let v397: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                    let v398: Rc<str> = Rc::<str>::from(format!("{}{}", v396, v397));
                                    let v399: i32 = v393.wrapping_add(3i32);
                                    let v400: bool = v399 == v391;
                                    let v404: Rc<str> = if v400 {
                                        let v401: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        v401.clone()
                                    } else {
                                        let v402: i32 = v391.wrapping_sub(1i32);
                                        let v403: Rc<str> = string_slice(&v0.clone(), v399 as i64, v402 as i64);
                                        v403.clone()
                                    };
                                    let v405: i32 = 0i32;
                                    let v406: i32 = 0i32;
                                    let v407: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v408: Rc<str> = escape_from_0(v404.clone(), v405, v406, v407.clone());
                                    let v410: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v398, v408)), v397));
                                    let v411: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                    let v412: Rc<str> = Rc::<str>::from(format!("{}{}", v410, v411));
                                    let v413: u8 = b']';
                                    let v414: i32 = v392.wrapping_add(2i32);
                                    let v415: i32 = find_char_from_2(v0.clone(), v413, v414);
                                    let v416: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src="); } LIT.with(|lit| lit.clone()) };
                                    let v417: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src=\""); } LIT.with(|lit| lit.clone()) };
                                    let v418: i32 = v415.wrapping_add(2i32);
                                    let v419: bool = v418 == v393;
                                    let v422: Rc<str> = if v419 {
                                        v407.clone()
                                    } else {
                                        let v420: i32 = v393.wrapping_sub(1i32);
                                        let v421: Rc<str> = string_slice(&v0.clone(), v418 as i64, v420 as i64);
                                        v421.clone()
                                    };
                                    let v423: i32 = 0i32;
                                    let v424: i32 = 0i32;
                                    let v425: Rc<str> = escape_from_0(v422.clone(), v423, v424, v407.clone());
                                    let v427: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v417, v425)), v397));
                                    let v428: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" alt="); } LIT.with(|lit| lit.clone()) };
                                    let v430: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v427, v428)), v397));
                                    let v431: bool = v414 == v415;
                                    let v434: Rc<str> = if v431 {
                                        v407.clone()
                                    } else {
                                        let v432: i32 = v415.wrapping_sub(1i32);
                                        let v433: Rc<str> = string_slice(&v0.clone(), v414 as i64, v432 as i64);
                                        v433.clone()
                                    };
                                    let v435: i32 = 0i32;
                                    let v436: i32 = 0i32;
                                    let v437: Rc<str> = escape_from_0(v434.clone(), v435, v436, v407.clone());
                                    let v441: Rc<str> = Rc::<str>::from(format!("{}{}", v412, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v430, v437)), v397)), v411))));
                                    let v442: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
                                    let v443: Rc<str> = Rc::<str>::from(format!("{}{}", v441, v442));
                                    (v0, v1, v2) = (v0.clone(), v394, v443.clone());
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
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        inline_from_1(v0.clone(), v1, v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn dtl_escape(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn dtl_render_inline(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
