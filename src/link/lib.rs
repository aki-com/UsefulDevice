// モジュール定義
pub mod discovery;
pub mod connection;
pub mod service_registration;
pub mod server_init;

// 関数と型のre-export
pub use discovery::{get_devices, DeviceInfo};
pub use connection::connect_to_device;
pub use service_registration::{register_mdns_service, get_local_ip};
pub use server_init::{ServerConfig, ServerSetup, initialize_server, create_tcp_listener};
