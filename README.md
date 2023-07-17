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
use capsolver::{CapSolver, Config};

#[tokio::main]
async fn main() { 
    let config = Config::new("<YOUR CLIENT KEY>", None);
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
```

 ## API Documentation

For detailed information on the available methods and usage examples, please refer to the [Capsolver API SDK documentation](https://docs.capsolver.com).

## License

This project is licensed under the [MIT License](LICENSE).
