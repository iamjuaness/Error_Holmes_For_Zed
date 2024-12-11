# Error Holmes

Error Holmes is an extension for the [Zed editor](https://zed.dev/) that enhances error and warning visibility in your code projects. Inspired by the "Error Lens" extension, Error Holmes integrates seamlessly into your workflow by displaying diagnostics (errors and warnings) directly in the editor for supported programming languages. It also intelligently detects the programming language based on the project's structure and utilizes the corresponding Language Server Protocol (LSP). ✨✨✨

## Features
- **Real-time diagnostics**: Displays errors and warnings directly in the editor, enhancing visibility and reducing debugging time. 🔍🔍🔍
- **Language auto-detection**: Automatically detects the programming language based on project files (e.g., `package.json` for JavaScript/TypeScript). 📁📁📁
- **Customizable diagnostics styling**: Modify the appearance of error and warning highlights to suit your preferences. 🎨🎨🎨
- **Integrated LSP support**: Supports multiple LSPs for different languages, with configurable initialization options. 🔗🔗🔗

## Installation
1. Clone this repository: ✏️✏️✏️
   ```sh
   git clone https://github.com/yourusername/error-holmes.git
   ```
2. Navigate to the project directory: 📂📂📂
   ```sh
   cd error-holmes
   ```
3. Build the extension: 🛠️🛠️🛠️
   ```sh
   cargo build --release
   ```
4. Install the extension into Zed: 🔧🔧🔧
   - Open Zed.
   - Go to **Settings > Extensions**.
   - Add a new extension by pointing to the path of the compiled `error-holmes` binary.

## Usage
1. Open a project in Zed. 🚀🚀🚀
2. Error Holmes will detect the project's language based on common configuration files (e.g., `package.json`, `requirements.txt`, etc.). 📜📜📜
3. Errors and warnings will be highlighted directly in the editor. ⚡⚡⚡

### Supported Languages
- **JavaScript/TypeScript** (via `typescript-language-server`)
- **Python** (via `pyls`)
- **Ruby** (via `solargraph`)
- Add support for more languages by extending the `detect_language_from_project` and `get_language_server_path` functions in `src/lib.rs`. 🛠️🛠️🛠️

## Development
### Prerequisites
- [Rust](https://www.rust-lang.org/) programming language installed. 🔧🔧🔧
- Familiarity with Zed and the Language Server Protocol. 📚📚📚

### Project Structure
- `src/lib.rs`: Contains the main logic for the extension, including language detection, LSP configuration, and diagnostics handling. 🗂️🗂️🗂️
- `Cargo.toml`: Manages dependencies and metadata for the Rust project. 📋📋📋

### Extending the Project
1. **Add support for new languages**: ✏️✏️✏️
   - Extend the `files_to_check` array in `detect_language_from_project` to recognize new project types.
   - Add the corresponding LSP binary in `get_language_server_path`.

2. **Enhance diagnostics handling**: ✨✨✨
   - Modify `language_server_initialization_options` to customize diagnostics features.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve Error Holmes. 🤝🤝🤝

## License
This project is licensed under the [MIT License](LICENSE). 📝📝📝

## Acknowledgments
- Inspired by the "Error Lens" extension for other editors. 🙌🙌🙌
- Thanks to the Zed team for their amazing editor and API support. 🌟🌟🌟

---

Bring the power of Error Holmes to your development workflow and never miss a diagnostic again! 🔍🔍🔍