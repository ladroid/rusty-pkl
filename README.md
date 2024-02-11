# pkl-rs

pkl-rs is a Rust library for parsing Pkl configuration files. Pkl is a simple configuration language that supports hierarchical structures and various data types.

## Features

- Parse Pkl files into a structured data representation in Rust.
- Support for basic Pkl syntax, including key-value pairs, nested objects, and common data types such as strings, integers, floats, and booleans.

## Usage

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your-username/rust-pkl-parser.git
   ```

2. Navigate to the project directory:

   ```bash
   cd rust-pkl-parser
   ```

3. Run the parser with the desired Pkl file:

   ```bash
   cargo run --example path/to/your/file.pkl
   ```

   Replace `path/to/your/file.pkl` with the path to your Pkl file.

## Example

Suppose you have a Pkl file named `example.pkl` with the following content:

```pkl
name = "Pkl: Configure your Systems in New Ways"
attendants = 100
isInteractive = true
amountLearned = 13.37

bird {
  name = "Common wood pigeon"
  diet = "Seeds"
  taxonomy {
    species = "Columba palumbus"
  }
}
```

Running the parser with this file will produce structured output representing the parsed Pkl values.

## Advanced Usage

You can also access specific parameters programmatically and assign them to variables using the provided functions in the library. For example, to access the `name` parameter:

```rust
use pkl_rs::*;

fn main() {
    match parse_pkl("examples\\config.pkl") {
        Ok(pkl_value) => {
            println!("Parsed Pkl value: {:?}", pkl_value);
            // You can further process the parsed Pkl value as needed
            if let Some(value) = find_parameter(&pkl_value, "name") {
                println!("Found name parameter: {:?}", value);
            } else {
                println!("Parameter 'name' not found.");
            }
        }
        Err(err) => {
            eprintln!("Error parsing Pkl file: {}", err);
        }
    }
}
```

## Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Comment on issues that require feedback.
  4. Contribute code via [pull requests].

[issue]: https://github.com/ladroid/pkl-rs/issues
[pull requests]: https://github.com/ladroid/pkl-rs/pulls

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.