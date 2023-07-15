use capsolver::{CapSolver, Config};

#[tokio::main]
async fn main() { 
    let config = Config::new("CAI-7AFF7887960A71E568F95C364986E539", None);
    let capsolver = CapSolver::new(config);

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
   
    match capsolver.get_task_result("h").await {
        Ok(o) => {
            println!("{}", o.to_string());
        },
        Err(e) => {
            println!("{}", e);
        },
    };
}
