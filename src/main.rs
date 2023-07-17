use capsolver::{CapSolver, Config};

#[tokio::main]
async fn main() { 
    let config = Config::new("CAI-7AFF7887960A71E568F95C364986E539", None);
    //Or load it from the environment
    let config = Config::from_env().unwrap();

    let capsolver = CapSolver::new(config);

    //Check balance
    match capsolver.get_balance().await {
        Ok(o) => {
            println!("Balance: {}\nPackages: {:?}", o.balance, o.packages);
        },
        Err(e) => {
            println!("Error checking balance\n{}", e);
        },
    }

    //Create task
    match capsolver
        .recognition()
        .image_to_text("l".to_string(), None, None, None)
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
    match capsolver.get_task_result("h").await {
        Ok(o) => {
            println!("{}", o.to_string());
        },
        Err(e) => {
            println!("{}", e);
        },
    };
}
