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
fn main() {
    let config = Config::new("", None);
    let capsolver = CapSolver::new(config);

    let balance = capsolver.get_balance().await.unwrap().balance.unwrap();

    println!("Balance: {}", balance);

    let task_id = capsolver
        .token()
        .aws_waf("<Type>", "<Website URL>", None)
        .await
        .unwrap()["taskId"]
        .as_str();

    match capsolver.get_task_result(task_id).await {
        Ok(res) => {
            //Your logic
        }
        Err(err) => {
            println!(err);
        }
    }
}
```

 ## API Documentation

For detailed information on the available methods and usage examples, please refer to the [Capsolver API SDK documentation](https://docs.capsolver.com).

## License

This project is licensed under the [MIT License](LICENSE).
