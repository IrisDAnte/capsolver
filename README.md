# Rust crate for [Capsolver API]
The easiest way to quickly integrate [Capsolver] into your rust code to automate solving of any type of captcha.

- [Installation](#installation)
- [Configuration](#configuration)
- [Example Usage](#example-usage)
- [Contribution](#contribution)
- [Licence](#licence)

## Installation
To install, use one of them:

Using `cargo`
```bash
cargo add capsolver
```
Using `Cargo.toml`
```toml
[dependencies]
capsolver = "0.4.1"
```

> **Warning**
> 
> This requires an asynchronous runtime e.g `tokio`

## Configuration

Import the crate like this:
```rust
use capsolver::{Capsolver, Token, Recognition, Config};
```
> **Note**
>
> Importing `Token` or `Recognition` is optional
> 
> `Capsolver` includes both of them

A `Config` can be created like:
```rust
let config = Config::new(ClientKey, ApiUrl, Interval)

//Or simply create from env
let config = Config::from_env()?;
```
> **Note**
>
> `ClientKey` is required and it's your client key from [Capsolver Dashboard]
>
> `ApiUrl` is optional and its default value is [Capsolver API]
>
> `Interval` is also optional and it is the interval in `ms` at which it'll check for task results. Default value `3000`

A client can be created like this:
```rust
//Use any of them as per your needs

let token = Token::new(config);

let recognition = Recognition::new(config);

let capsolver = Capsolver::new(config);
```
> **Note**
>
> `Token` is for the token APIs
>
> `Recognition` is meant for recognition APIs only
>
> `Capsolver` includes both of them

## Example Usage
Checking your Capsolver balance:
```rust
let res = capsolver.get_balance().await?;

println!("Balance: {},\nPackages: {:?}", res.balance, res.packages);
```
Using `ImageToText` recognition API:
```rust
let task = capsolver
  .recognition()
  .image_to_text("Base64 image string", None, None, None)
  .await?;
let solution = task["solution"]["text"].as_str()?;

println!("Solution: {}", solution);
```
Using `FunCapctha` token API:
```rust
use capsolver::{OnlyToken};

let task = capsolver
  .token()
  .fun_captcha("websiteURL", "websitePublicKey", None, None, None)
  .await?;
let task_id = task["taskId"].as_str()?;
let soltion: OnlyToken = capsolver
  .get_task_result(task_id)
  .await?;

println!("Solution: {}", solution.token);
```
> **Note** Refer to [Capsolver Docs] for the options that are passed in the above functions

> **Note** The return type of `get_task_result()` of `Token` task results
> can be better if you cast the following types individually according to the [this](#better-types)

## Better Types
- `HCaptchaToken`
  - `HCaptcha`
- `OnlyToken`
  - `FunCaptcha`
  - `MtCaptcha`
  - `CyberSi Ara`
- `GeeTestV3Token`
  - `GeeTestV3`
- `GeeTestV4Token`
  - `GeeTestV4`
- `ReCaptchaToken`
  - `ReCaptchaV3`
  - `ReCaptchaV4`
- `DataDomeToken`
  - `DataDome`
- `AwsWafToken`
  - `AwsWaf`
- `CloudFlareToken`
  - `CloudFlare (Turnstile)`
  - `CloudFlare (Challenge)`

> **Note** This list only applies to token task results

Example:
```rust
use capsolver::{OnlyToken, HCaptchaToken, AwsWafToken};

//This example assumes that you already have task id for each of them
let fun_captcha_task_id = TaskId;
let h_captcha_task_id = TaskId;
let aws_waf_task_id = TaskId;

let fun_captcha_solution: OnlyToken = capsolver
  .get_task_result(fun_captcha_task_id)
  .await?;
let h_captcha_solution: HCaptchaToken = capsolver
  .get_task_result(fun_captcha_task_id)
  .await?;
let aws_waf_solution: AwsWafToken = capsolver
  .get_task_result(fun_captcha_task_id)
  .await?;

println!("Funcaptcha: {}", fun_captcha_solution.token);
println!("HCaptcha: {}", h_captcha_solution.captcha_key);
println!("AwsWaf: {}", aws_waf_solution.cookie);
```
That's the best I can explain :D
## Contribution
They're welcome :D

## Licence
This project is licensed under the MIT License

[Capsolver]: https://capsolver.com/
[Capsolver API]: https://api.capsolver.com/
[Capsolver Docs]: https://docs.capsolver.com/
