use capsolver::{CapSolver, Config, OnlyToken};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let capsolver = CapSolver::new(config);

    //Check balance
    match capsolver.get_balance().await {
        Ok(o) => {
            println!("Balance: {}\nPackages: {:?}", o.balance, o.packages);
        }
        Err(e) => {
            println!("Error checking balance\n{}", e);
        }
    }

    //Create task
    match capsolver
        .token()
        .fun_captcha("<WebsiteUrl>", "WebsitePublicKey", None, None, None)
        .await
    {
        Ok(o) => {
            println!("{}", o.to_string());
        }
        Err(e) => {
            println!("{}", e);
        }
    };

    //Get task result
    match capsolver.get_task_result::<OnlyToken>("h").await {
        Ok(o) => {
            println!("{}", o.token);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
