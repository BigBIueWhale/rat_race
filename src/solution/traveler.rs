#[derive(Debug, Copy, Clone)]
pub struct Traveler
{
    transaction_count: i32,
    profit: i64,
}

impl Traveler {
    pub fn new() -> Traveler
    {
        Traveler{
            transaction_count: 0,
            profit: 0,
        }
    }
    fn buy(&mut self, price: i32)
    {
        self.transaction_count += 1;
        self.profit -= i64::from(price);
    }
    fn sell(&mut self, price: i32)
    {
        self.transaction_count += 1;
        self.profit += i64::from(price);
    }
    pub fn do_transaction(&mut self, price: i32)
    {
        if self.does_own_stock()
        {
            self.sell(price);
        }
        else
        {
            self.buy(price);
        }
    }
    pub fn get_profit(&self) -> i64
    {
        self.profit
    }
    pub fn get_transaction_count(&self) -> i32
    {
        self.transaction_count
    }
    pub fn does_own_stock(&self) -> bool
    {
        self.transaction_count % 2 == 1
    }
}
