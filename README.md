# env-map

env-map is a Rust crate designed to simplify the process of reading environment variables into a structured format. By deriving `EnvMap`, you can easily map environment variables to a Rust struct, supporting both variables from a `.env` file and system environment variables.

## Features

- Easy mapping of environment variables to a Rust struct.
- Supports reading variables from a `.env` file or system environment variables.
- Simplifies configuration management in Rust applications.

## Getting Started

To use env-map in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
env-map = "0.1.0"
```

## Usage

Define your configuration struct and derive `EnvMap`. This will automatically map environment variables to the struct fields.

Example:

```rust
#[derive(EnvMap)]
struct Config {
    api_key: String,
    path_to_save: String,
}

fn main() {
    // This will read the API_KEY and PATH_TO_SAVE variables from the environment
    let config = Config::get_config().get_or_init(Config::default);

    // Use your config as needed
    println!("API Key: {}", config.api_key);
    println!("Path to Save: {}", config.path_to_save);
}
```

## Environment Variables

env-map maps environment variables to corresponding fields in your struct. For the `Config` struct defined in the example, it will look for environment variables with the same names as the struct's fields.

In the provided example, the struct fields are `api_key` and `path_to_save`, so env-map will search for `API_KEY` and `PATH_TO_SAVE` in the environment. However, the actual environment variables required by env-map depend on the fields you define in your struct.

Example environment variables for the given struct:

- `API_KEY`: Corresponds to `api_key` in `Config`.
- `PATH_TO_SAVE`: Corresponds to `path_to_save` in `Config`.

These variables can be set in a `.env` file in the same directory as your application or as system environment variables. The names of the environment variables are automatically derived from the struct's field names, converted to uppercase.

## Contributing

Contributions to env-map are welcome! Please feel free to submit pull requests, report bugs, or suggest features.

## License

This crate is licensed under the Mozilla Public License 2.0 (MPL 2.0).

### Overview of MPL 2.0 for this Crate:

- The MPL 2.0 license applies to all parts of this crate.
- Any modifications made to files in this crate must be made available under the same MPL 2.0 license.
- Use of the crate's code, either in whole or in part, in other projects does not require the entire project to be MPL 2.0 licensed, only the parts that originate from this crate.

For more details, please refer to the [LICENSE](LICENSE) file in this repository. The full text of the MPL 2.0 can also be found at [Mozilla's official site](https://www.mozilla.org/en-US/MPL/2.0/).

By using, contributing to, or distributing this crate, you agree to the terms and conditions of the MPL 2.0 license.
