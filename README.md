To create a README for your `dfx-json-inspector` crate, you can use the existing documentation comments in your code. Here's a draft README.md based on the information you provided:

```markdown
# dfx-json-inspector

`dfx-json-inspector` is a command-line tool for analyzing `dfx.json` files in Internet Computer projects. It counts the number of canisters defined in the project and provides a summary of canister types.

## Features

- Parses and analyzes `dfx.json` files
- Counts total number of canisters
- Categorizes canisters by type
- Supports custom paths for `dfx.json` files

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dfx-json-inspector = "0.1.0"
```

## Usage

Run the tool from the command line:

```
$ dfx-json-inspector
```

Or specify a custom path:

```
$ dfx-json-inspector --path /path/to/project
```

## API

The main functionality is provided by two key functions:

### `read_dfx_json(path: &str) -> Result<Value>`

Reads and parses the dfx.json file from the given path.

### `analyze_canisters(json: &Value) -> Result<(usize, HashMap<String, u32>)>`

Analyzes the canisters defined in the parsed dfx.json, returning the total count and a breakdown by type.

## License

This project is licensed under MIT OR Apache-2.0.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
```

To add this README to your project:

1. Create a file named `README.md` in the root directory of your project.
2. Copy the content above into the file.
3. Customize the content as needed, adding any additional information about your project.
4. Ensure your `Cargo.toml` file includes the README in the package. Add or modify the following lines:

   ```toml
   [package]
   # ... other package information ...
   description = "A command-line tool for analyzing dfx.json files in Internet Computer projects"
   readme = "README.md"
   ```

5. Commit these changes to your repository.
6. Update your crate version (e.g., to 0.1.1) in `Cargo.toml`.
7. Publish the updated crate:

   ```
   cargo publish
   ```

This should resolve the issue of missing documentation on crates.io. The README will now appear on your crate's page, providing users with essential information about your tool.