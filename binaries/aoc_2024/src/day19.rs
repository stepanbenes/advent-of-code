use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end_of_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /// Inserts a word into the Trie
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            // If the character is not already a child, create a new TrieNode
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }

        // Mark the last node as the end of a word
        current.is_end_of_word = true;
    }

    /// Builds a Trie from a list of words
    pub fn build_from_words(&mut self, words: &[&str]) {
        for &word in words {
            self.insert(word);
        }
    }

    pub fn try_match(&self, word: &str) -> usize {
        let mut memo = HashMap::new();
        self.dfs_with_memo(word, &mut memo)
    }

    fn dfs_with_memo(&self, word: &str, memo: &mut HashMap<String, usize>) -> usize {
        // Base cases
        if word.is_empty() {
            return 1;
        }

        // Check memoization cache
        if let Some(&count) = memo.get(word) {
            return count;
        }

        let mut count = 0;
        let mut current = &self.root;

        // Use iterators instead of indexing
        for (i, ch) in word.chars().enumerate() {
            match current.children.get(&ch) {
                Some(next_node) => {
                    current = next_node;
                    if current.is_end_of_word {
                        count += self.dfs_with_memo(&word[i + 1..], memo);
                    }
                }
                None => break,
            }
        }

        // Cache the result
        memo.insert(word.to_string(), count);
        count
    }
}

pub fn count_of_possible_designs() -> (usize, usize) {
    let mut trie = Trie::new();

    let (input_patterns, input_designs) = get_input_patterns_and_designs();

    // Build the Trie
    trie.build_from_words(&input_patterns);

    // Debug print the Trie structure
    println!("{:#?}", trie);

    // Input words to split

    let mut possible_designs = 0;
    let mut possible_designs_combinations = 0;
    for input_design in input_designs {
        // Find all possible ways to split the word
        let count = trie.try_match(input_design);

        println!("Design: {}, Possible Designs: {}", input_design, count);

        possible_designs_combinations += count;

        if count > 0 {
            possible_designs += 1;
        }
    }

    (possible_designs, possible_designs_combinations)
}

fn get_input_patterns_and_designs() -> (Vec<&'static str>, Vec<&'static str>) {
    //     let input_patterns = r"r, wr, b, g, bwu, rb, gb, br";
    //     let input_designs = r"brwrr
    // bggr
    // gbbr
    // rrbgbr
    // ubwu
    // bwurrg
    // brgr
    // bbrgwb";
    let input_patterns = include_str!("../input/day19_patterns.txt");
    let input_designs = include_str!("../input/day19_designs.txt");
    (
        input_patterns.split(", ").collect(),
        input_designs.lines().collect(),
    )
}
