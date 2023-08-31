# CICD Commit Helper

![Version](https://img.shields.io/badge/version-0.1-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

CICD Commit Helper is a command-line tool designed to facilitate good commit policies in a CI/CD environment. It checks the number of changed files and lines in a commit and prompts the user to confirm or abort overly large commits.

## Features

- Checks the number of changed files and lines before committing.
- Prompts the user to confirm or abort if the commit exceeds a certain size.
- Automatically commits and pushes changes to the repository.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/JarlDue/cicd-commit-tool.git
    ```

2. Navigate to the project directory:

    ```bash
    cd cicd-commit-tool
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

## Usage

Run the tool with the following command:

```bash
cicd-commit-tool commit [commit-message]
```

### Options

- `commit-message`: The message for the commit. Optional.

## Environment Variables

- `GIT_TOKEN`: Your GitHub personal access token. This is required for pushing changes.

## Contributing

1. Fork the project.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a pull request.

## Future Plans

We have several enhancements and features planned to make CICD Commit Helper even more robust and user-friendly:

### Short Term

1. **Interactive CLI Interface**: A step-by-step guide for committing changes.
2. **Config File Support**: Allow users to specify commit rules in a `.cicdconfig` file.
3. **Multi-Branch Support**: Enable checks and operations for branches other than `main`.
4. **Token/Password Scanning**: Scan for sensitive information like tokens or passwords in files about to be committed and abort the commit if any are found.

### Long Term

1. **CI/CD Integration**: Seamlessly integrate with popular CI/CD tools like Jenkins, GitLab CI, and GitHub Actions.
2. **Plugin Architecture**: Allow users to extend functionality through plugins.
3. **Comprehensive Logging**: Detailed logs for debugging and audit purposes.
