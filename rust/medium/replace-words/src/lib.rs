use std::{collections::HashMap, hash::Hash};

fn find_root<'l>(mut dict: &Trie<u8>, word: &'l str) -> Option<&'l str> {
    for (index, char) in word.bytes().enumerate() {
        let node = match dict.get(&char) {
            Some(node) => node,
            None => break,
        };

        if let Trie::End = node {
            return Some(&word[..=index]);
        }

        dict = node
    }

    None
}

enum Trie<T: Eq + Hash> {
    Node(HashMap<T, Trie<T>>),
    End,
}

impl<T: Eq + Hash> Trie<T> {
    pub fn insert(&mut self, items: impl IntoIterator<Item = T>) {
        let mut items = items.into_iter();

        let next = match items.next() {
            Some(value) => value,
            None => return *self = Self::End,
        };

        match self {
            Trie::Node(dict) => dict.entry(next).or_insert(Trie::default()).insert(items),
            Trie::End => return,
        }
    }

    pub fn get(&self, key: &T) -> Option<&Trie<T>> {
        match self {
            Self::Node(dict) => dict.get(key),
            Self::End => None,
        }
    }
}

impl<T: Eq + Hash> Default for Trie<T> {
    fn default() -> Self {
        Self::Node(HashMap::default())
    }
}

/// In English, we have a concept called **root** , which can be followed by some other word to form another longer word - let's call this word **derivative**. For example, when the **root** `"help"` is followed by the word `"ful"`, we can form a derivative `"helpful"`.
///
/// Given a `dictionary` consisting of many **roots** and a `sentence` consisting of words separated by spaces, replace all the derivatives in the sentence with the **root** forming it. If a derivative can be replaced by more than one **root** , replace it with the **root** that has **the shortest length**.
///
/// Return _the`sentence`_ after the replacement.
///
/// # Example
/// ```
/// use replace_words::replace_words;
///
/// assert_eq!(replace_words(["cat", "bat", "rat"].into_iter().map(String::from).collect(), "the cattle was rattled by the battery".to_string()), "the cat was rat by the bat");
/// assert_eq!(replace_words(["a", "b", "c"].into_iter().map(String::from).collect(), "aadsfasf absbs bbab cadsfafs".to_string()), "a a b c");
/// assert_eq!(replace_words(["a", "aa", "aaa", "aaaa"].into_iter().map(String::from).collect(), "a a a a a a a a bbb baba a".to_string()), "a a a a a a a a bbb baba a".to_string());
/// ```
pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie: Trie<u8> = Trie::default();

    for root in dictionary {
        trie.insert(root.into_bytes());
    }

    sentence
        .split_whitespace()
        .map(|word| find_root(&trie, word).unwrap_or(word))
        .collect::<Vec<&str>>()
        .join(" ")
}
