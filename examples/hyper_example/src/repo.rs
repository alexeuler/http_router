use super::error::{Error, ErrorKind};
use failure::Fail;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub users: Vec<User>,
}

impl Repo {
    pub fn get_users(&self) -> Vec<User> {
        self.users.clone()
    }

    pub fn find_user(&mut self, id: usize) -> Result<&mut User, Error> {
        self.users.iter_mut().find(|u| u.id == id).ok_or(
            format_err!("User with id: {}", id)
                .context(ErrorKind::NotFound)
                .into(),
        )
    }

    pub fn create_user(&mut self, user: User) -> User {
        self.users.push(user.clone());
        user
    }

    pub fn update_user(&mut self, user: User) -> User {
        self.delete_user(user.id);
        self.create_user(user)
    }

    pub fn delete_user(&mut self, id: usize) -> () {
        self.users = self
            .users
            .clone()
            .into_iter()
            .filter(|u| u.id != id)
            .collect();
    }

    pub fn get_transactions(&mut self, user_id: usize) -> Result<Vec<Transaction>, Error> {
        self.find_user(user_id).map(|user| user.transactions.clone())
    }

    pub fn create_transaction(&mut self, user_id: usize, tx: Transaction) -> Result<Transaction, Error> {
        self.find_user(user_id).map(move |ref mut user| {
            user.transactions.push(tx.clone());
            tx
        })
    }

    pub fn delete_transactions(&mut self, user_id: usize, hash: String) -> Result<(), Error> {
        self.find_user(user_id).map(|ref mut user| {
            user.transactions = user.transactions.clone().into_iter().filter(|t| t.hash == hash).collect();
        })
    }

    pub fn update_transaction(&mut self, user_id: usize, tx: Transaction) -> Result<Transaction, Error> {
        self.delete_transactions(user_id, tx.hash.clone()).and_then(|_| {
            self.create_transaction(user_id, tx)
        })
    }
}
