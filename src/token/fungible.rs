use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
}

impl Token {
    pub fn new(name: &str, symbol: &str, initial_supply: u64, owner: &str) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner.to_string(), initial_supply);

        Token {
            name: name.to_string(),
            symbol: symbol.to_string(),
            total_supply: initial_supply,
            balances,
        }
    }

    pub fn balance_of(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            println!("Insufficient balance for {}", from);
            return false;
        }

        self.balances.insert(from.to_string(), from_balance - amount);
        let to_balance = self.balance_of(to);
        self.balances.insert(to.to_string(), to_balance + amount);

        println!("Transferred {} {} from {} to {}", amount, self.symbol, from, to);
        true
    }

    pub fn mint(&mut self, to: &str, amount: u64) {
        let current_balance = self.balance_of(to);
        self.balances.insert(to.to_string(), current_balance + amount);
        self.total_supply += amount;

        println!("Minted {} {} to {}", amount, self.symbol, to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_transfer() {
        let mut token = Token::new("TestToken", "TT", 1000, "admin");
        token.mint("user1", 100);
        assert_eq!(token.balance_of("user1"), 100);

        let success = token.transfer("user1", "user2", 50);
        assert!(success);
        assert_eq!(token.balance_of("user1"), 50);
        assert_eq!(token.balance_of("user2"), 50);
    }
}
