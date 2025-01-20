/// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
///
/// - '.' Matches any single character.​​​​
/// - '*' Matches zero or more of the preceding element.
///
/// The matching should cover the entire input string (not partial).
///
/// # Example
/// ```
/// use regular_expression_matching::is_match;
///
/// assert!(!is_match("aa".to_string(), "a".to_string()));
/// assert!(is_match("aa".to_string(), "..".to_string()));
/// assert!(is_match("".to_string(), "a*".to_string()));
/// assert!(is_match("aa".to_string(), "a*".to_string()));
/// assert!(is_match("ab".to_string(), ".*".to_string()));
/// assert!(is_match("aab".to_string(), "c*a*b".to_string()));
/// assert!(!is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
/// ```
pub fn is_match(s: String, p: String) -> bool {
    // based on a non-deterministic finite automata
    let mut states = vec![s.as_bytes().into_iter().rev()];
    let patterns = p.as_bytes().into_iter().rev();

    // *
    let mut kleene_closure = false;

    for pattern in patterns {
        if pattern == &b'*' {
            kleene_closure = true;
            continue;
        }

        let mut new_states = Vec::new();
        for mut state in states.into_iter() {
            if kleene_closure {
                // match zero tokens
                new_states.push(state.clone());

                // match one token for each iteration
                while let Some(next) = state.next() {
                    if next != pattern && pattern != &b'.' {
                        break;
                    }
                    new_states.push(state.clone());
                }
            } else if let Some(next) = state.next() {
                // check if the next token matches
                if next == pattern || pattern == &b'.' {
                    new_states.push(state);
                }
            }
        }

        kleene_closure = false;
        states = new_states;
    }

    // check if any active states are accepted
    states.into_iter().any(|mut s| s.next() == None)
}
