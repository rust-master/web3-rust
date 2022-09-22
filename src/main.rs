
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}