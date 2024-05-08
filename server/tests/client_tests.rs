use bevy::app;
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode,
    connection::{ConnectionConfiguration, ConnectionEvent, ConnectionLostEvent},
    Client, QuinnetClientPlugin,
};

fn start_server() {
    let mut app = app::App::new();

    // client starten
}

// #[test]
// fn hello_world() {
//     println!("Hello, world!");
//     assert_eq!(2 + 2, 4);
// }
