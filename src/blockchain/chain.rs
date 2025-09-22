use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string(), difficulty);
        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    pub fn latest_hash(&self) -> String {
        self.blocks
            .last()
            .map(|block| block.hash.clone())
            .unwrap_or_else(|| "0".to_string())
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        let index = self.blocks.len() as u64;
        let previous_hash = self.latest_hash();
        let block = Block::new(index, transactions, previous_hash, self.difficulty);
        self.blocks.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain::new(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blockchain_addition() {
        let mut chain = Blockchain::default();
        assert_eq!(chain.blocks.len(), 1); // Only genesis block

        chain.add_block(vec!["Tx1".to_string()]);
        assert_eq!(chain.blocks.len(), 2);
    }

    #[test]
    fn test_chain_validation() {
        let mut chain = Blockchain::default();
        chain.add_block(vec!["Tx1".to_string()]);
        assert!(chain.is_valid());
    }
}
