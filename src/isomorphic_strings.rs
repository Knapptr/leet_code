use std::collections::{HashMap};

pub fn is_isomorphic(s:String, t:String) -> bool {
    if s.len() != t.len() { 
        false
    }else 
    {
        let mut i = 0usize;
        let mut s_mapping = HashMap::new();
        let mut t_mapping = HashMap::new();

        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        while i < s.len() {
            println!("sChars at loop: {s_chars:?}");
            let s_char = s_chars[i];
            let t_char = t_chars[i];

            if *s_mapping.entry(s_char).or_insert(t_char) != t_char {return false}
            if *t_mapping.entry(t_char).or_insert(s_char) != s_char {return false}


            i += 1;

            
        }
        true
    }
}

#[cfg(test)]

#[test]
fn single_letter(){
    assert!(is_isomorphic("a".to_string(), "b".to_string()));
}

#[test]
fn two_letters(){
    assert!(is_isomorphic("ab".to_string(), "qc".to_string()));
}

#[test]
fn repeat_letter_true(){
    assert!(is_isomorphic("aba".to_string(), "qcq".to_string()));
}

#[test]
fn three_letters_false(){
    assert!(!is_isomorphic("aba".to_string(), "qcg".to_string()));
}

#[test]
fn maps_twice_false(){
    assert!(!is_isomorphic("badc".to_string(), "baba".to_string()));
}
