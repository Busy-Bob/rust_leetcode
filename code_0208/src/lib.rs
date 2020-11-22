/*
    1、Option.get_or_insert() 在处理链表时候很有用
    2、Option.as_ref() 与 Option.as_mut() 起到了不发生所有权转移的作用，只使用&T 和&mut T
    3、match &Option  ==   match Option.as_ref() !!!! 就可以使得所有权不丢失了。

*/


#[derive(Debug, Clone)]
struct Trie {
    is_string: bool,
    next: Vec<Option<Box<Trie>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            is_string: false,
            next: vec![None; 26],
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut root = self;
        for &c in word.as_bytes() {
            let index = (c - b'a') as usize;
            // if root.next[index].is_none() {
            //     root.next[index] = Some(Box::new(Trie::new()));
            // }
            // root = root.next[index].as_mut().unwrap();

            // 这个更方便
            root = root.next[index].get_or_insert(Box::new(Trie::new()));
        }

        root.is_string = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut root = self;
        for &c in word.as_bytes() {
            let index = (c - b'a') as usize;
            match &root.next[index] {
                None => return false,
                Some(x) => root = x.as_ref(),
            }
        }

        if root.is_string {
            true
        } else {
            false
        }
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut root = self;
        for &c in prefix.as_bytes() {
            let index = (c - b'a') as usize;
            match &root.next[index] {
                None => return false,
                Some(x) => root = x.as_ref(),
            }
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
