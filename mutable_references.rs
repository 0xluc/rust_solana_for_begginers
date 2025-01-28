struct Config {
    port: u16,
}
let mut config: Config = Config {
    port: 9090
};

let config_reference: &mut Config = &mut config;
config_reference.port = 4000;

println!("Using port {}", config.port);
