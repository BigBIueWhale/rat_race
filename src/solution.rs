mod traveler;

use traveler::{Traveler};

pub struct Solution
{
}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        // *2 because the LeetCode question uses is weird convention where
        // one transaction is buying and selling
        let max_transactions = k * 2;
        max_profit(max_transactions, &prices) as i32
    }
}

fn max_profit(max_transactions: i32, prices: &[i32]) -> i64 {
    let mut contestants: Vec<Traveler> = vec![Traveler::new()];
    for price in prices
    {
        let after_fork: Vec<Traveler> = fork_decisions(&contestants, price.clone(), max_transactions);
        let after_reduction: Vec<Traveler> = reduce_contestants(&after_fork);
        contestants = after_reduction;
    }
    contestants.iter().map(|contestant| contestant.get_profit()).max().unwrap()
}

// Tell all travelers to make a decision about
// the day in question.
// Each traveler has 2 possible decisions:
// Do nothing, buy/sell
// Returns a Vec<Travelers> that is approx. 2x longer
// than the given "contestants"
// All travelers in the returned vec will be in a state
// that is after making the decision about the specified day.
fn fork_decisions(contestants: &[Traveler], price: i32, max_transactions: i32) -> Vec<Traveler>
{
    let mut new_contestants: Vec<Traveler> = Vec::with_capacity(contestants.len() * 2);
    for contestant in contestants
    {
        new_contestants.push(contestant.clone());
        if contestant.get_transaction_count() < max_transactions
        {
            let mut new_contestant = contestant.clone();
            new_contestant.do_transaction(price);
            new_contestants.push(new_contestant);
        }
    }
    new_contestants
}

fn reduce_contestants(contestants: &[Traveler]) -> Vec<Traveler>
{
    let mut ret: Vec<Traveler> = Vec::with_capacity(contestants.len());
    for (index, contestant) in contestants.iter().enumerate()
    {
        if !is_any_alternative(&contestants, index)
        {
            ret.push(contestant.clone());
        }
    }
    ret
}

fn is_any_alternative(contestants: &[Traveler], index_defendant: usize) -> bool
{
    let defendant: Traveler = contestants[index_defendant];
    for (index, contestant) in contestants.iter().enumerate()
    {
        if index_defendant == index
        {
            continue;
        }
        let defendant_score: Comparison = is_left_worse_than_right_on_every_metric(defendant, contestant.clone());
        let is_alternative: bool = match defendant_score
        {
            Comparison::NotNecessarilyWorse => false,
            Comparison::WorseInEveryMetric => true,
            // In the case where there are multiple contestants with identical
            // metrics of success, we want to choose only one of them.
            // In order to achieve that, we'll make sure that the right-most
            // contestant out of that group will survive.
            Comparison::SameInEveryMetric => index_defendant < index,
        };
        if is_alternative
        {
            return true;
        }
    }
    false
}

// There are 3 parameters than determine the success of a traveler
// 1. Whether or not they own the stock currently (contestants are not comparable if different).
// 2. Profit so far (more is better).
// 3. Number of transactions already used up (less is better).
enum Comparison
{
    NotNecessarilyWorse,
    WorseInEveryMetric,
    SameInEveryMetric,
}
fn is_left_worse_than_right_on_every_metric(left: Traveler, right: Traveler) -> Comparison
{
    if left.does_own_stock() != right.does_own_stock()
    {
        // Not comparable
        return Comparison::NotNecessarilyWorse;
    }
    if left.get_profit() > right.get_profit()
    {
        // left has better profit
        return Comparison::NotNecessarilyWorse;
    }
    if left.get_transaction_count() < right.get_transaction_count()
    {
        // left has better transaction count
        return Comparison::NotNecessarilyWorse;
    }
    if left.get_profit() == right.get_profit()
        && left.get_transaction_count() == right.get_transaction_count()
    {
        return Comparison::SameInEveryMetric;
    }
    return Comparison::WorseInEveryMetric;
}
