# YTCrawler

YTCrawler is a powerful web crawler built in Rust, designed specifically for extracting valuable information from YouTube. Leveraging the performance and safety features of Rust, this tool provides a robust foundation for crawling and collecting data from YouTube pages.

## Features
- Efficient web crawling using Rust's concurrency and asynchronous capabilities.
- Seamless integration with the YouTube API for retrieving additional information.
- Configurable settings for customization, including API keys and crawling depth.
- Well-defined data structures for organizing and storing crawled data.
- Extensive error handling to ensure robust and reliable performance.

## Getting Started
1. Clone the repository: `git clone https://github.com/mgajurel/YTCrawler.git`
2. Navigate to the project directory: `cd YTCrawler`
3. Customize configuration settings in the `config.rs` module.
4. Build and run the project: `cargo run`

## Project Structure
The project follows a modular structure for better organization:

- **src/**
  - **main.rs**: Main entry point for the application.
  - **config.rs**: Configuration module for handling settings.
  - **crawler.rs**: Web crawler module for fetching and parsing web pages.
  - **youtube_api.rs**: Module for interacting with the YouTube API.
  - **data_structures.rs**: Module defining data structures used in the project.
  - **utils.rs**: Module containing utility functions.
  - **tests.rs**: File for writing tests.

- **Cargo.toml**: Configuration file specifying project metadata and dependencies.

## Dependencies
YTCrawler relies on the following Rust dependencies:
- `reqwest` for making HTTP requests.
- `scraper` for HTML parsing.

Make sure to check the `Cargo.toml` file for the most up-to-date list of dependencies.

## Contribution
Contributions are welcome! Feel free to open issues, submit pull requests, or suggest improvements. Let's make YTCrawler even better together.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
