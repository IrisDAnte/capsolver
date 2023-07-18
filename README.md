# Capsolver API SDK (Rust)

![Capsolver](https://www.capsolver.com/_nuxt/logo.eb4b912e.png)

The Capsolver API SDK in Rust is a library that allows developers to easily integrate the Capsolver captcha solving service into their Rust applications. Capsolver is a powerful captcha solving service designed to handle a wide range of captchas with high accuracy.

## Getting Started

To get started with the Capsolver API SDK, follow the steps below:

### Prerequisites

- Rust programming language
- Cargo package manager

## Usage

```rust
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
    let task = capsolver
        .token()
        .fun_captcha("<WebsiteUrl>", "<WebsitePublicKey>", None, None, None)
        .await
        .unwrap();
    let task_id = task["taskId"].as_str().unwrap();

    //Get task result
    match capsolver.get_task_result::<OnlyToken>(task_id).await {
        Ok(o) => {
            println!("{}", o.token);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
```

 ## API Documentation

For detailed information on the available methods and usage examples, please refer to the [Capsolver API SDK documentation](https://docs.capsolver.com).

## License

This project is licensed under the [MIT License](LICENSE).
