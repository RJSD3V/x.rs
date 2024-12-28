//Lifetimes: How long does the owner of a value or a reference exists. 
#[derive(Debug)]
struct Account {
    id : u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id:u32, holder:String) -> Self {
        
        Account {
        id: id, 
        balance: 0,
        holder: holder
        
        }
    }
    
    fn deposit(&mut self, amount: i32) -> i32{
        self.balance+=amount;
        self.balance
    }
    
    fn withdraw(&mut self, amount: i32) -> i32{
        self.balance-=amount;
        self.balance
        
    }
    
    fn summary(&self) -> String{
        format!("{} has a balance of {} ", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank{
            accounts : vec![]
        }
    }
    
    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }
    
    fn total_balance(&self) -> i32{
        self.accounts.iter().map(|account| account.balance).sum()
    }
    
    fn summary(&self) -> Vec<String>{
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }

}



fn main(){

    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Raajas Sode"));
    
    account.deposit(500);
    account.withdraw(200);
    
    println!("{:?}",account.summary());
    
    bank.add_account(account);
    
    
    
    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
    
}