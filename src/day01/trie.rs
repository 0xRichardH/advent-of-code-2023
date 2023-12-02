#[derive(Default)]
pub struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            let idx = Self::index_of_charactar(ch);
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> Option<&TrieNode> {
        let mut node = self;
        for ch in word.chars() {
            let idx = Self::index_of_charactar(ch);
            if let Some(n) = &node.children[idx] {
                node = n;
            } else {
                return None;
            }
        }

        Some(node)
    }

    pub fn is_end_of_word(&self) -> bool {
        self.is_end_of_word
    }

    pub fn index_of_charactar(ch: char) -> usize {
        ch as usize - 'a' as usize
    }
}
