struct Config {
    port: u16
}

let config: Config = Config {
    port: 8080
};

let config_reference: &Config = &config;

println!("Using port {}", config_reference.port);


let val = 10;
let r1 = &val;
let r2 = &val;
println!("{r1} should be the same as {r2}");
