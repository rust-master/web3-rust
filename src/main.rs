
#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("https://data-seed-prebsc-1-s1.binance.org:8545")?;

    let web3 = web3::Web3::new(transport);
    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("61c473bbE4afB7bf51CE558d25090f6aC1C616ED".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}

fn pow(base: i64, exponent: usize) -> i64 {
    let mut res = 1;
       if exponent == 0 {
           return 1;
       }
       for _ in 0..exponent {
           res *= base as i64;
       }
    res
}

#[cfg(test)]
mod tests {
    use super::pow;
    #[test]
    fn minus_two_raised_three_is_minus_eight() {
        assert_eq!(pow(-2, 3), -8);
    }
}