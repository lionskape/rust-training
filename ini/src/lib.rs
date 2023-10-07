#![forbid(unsafe_code)]

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

pub type IniFile = HashMap<String, HashMap<String, String>>;

pub fn parse(content: &str) -> IniFile {
    let mut res: IniFile = IniFile::new();
    let mut sec: &str = "";

    let lines = content.trim().lines();
    for l in lines.map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if l.starts_with('[') {
            assert!(l.ends_with(']'));
            sec = &l[1..l.len() - 1];
            assert!(!sec.contains(['[', ']']));
            res.entry(sec.to_string()).or_default();
            continue;
        }
        assert!(!sec.is_empty());
        let cur_sec = res.entry(sec.to_string()).or_default();
        let key_val: Vec<_> = l.split('=').collect();
        if key_val.len() == 1 {
            let key = key_val[0].trim();
            cur_sec.insert(key.to_string(), "".to_string());
            continue;
        }
        if key_val.len() == 2 {
            let (key, val) = (key_val[0].trim(), key_val[1].trim());
            cur_sec.insert(key.to_string(), val.to_string());
            continue;
        }
        panic!("key_val len should be 1 or 2");
    }
    res
}
