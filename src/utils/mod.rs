pub mod config;
pub mod password;
pub mod token;

pub fn print_running(local: &str, port: u16, ip: &str) {
    println!("    🚀 Backend server is running (Axum Rust)");
    println!("    - Local: http://{local}:{port}");
    println!("    - Network: http://{ip}:{port}");
}
