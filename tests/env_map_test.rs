use env_map::EnvMap;

#[derive(EnvMap)]
struct Config {
    name: String,
}

#[test]
fn it_works() {
    assert_eq!(
        Config::get_config().get_or_init(Config::default).name,
        "Rodrigo".to_string()
    );
}
