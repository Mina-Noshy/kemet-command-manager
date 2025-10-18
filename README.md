# Kemet CLI - Command Saver & Runner

A powerful command-line interface (CLI) tool designed to help you save, manage, and execute frequently used terminal commands using custom aliases. Perfect for developers, system administrators, and anyone looking to streamline their terminal workflow.

## 🚀 Features

*   **Save & Alias Commands:** Store complex or frequently used commands under simple, custom aliases.
*   **Quick Execution:** Run saved commands instantly using their assigned alias.
*   **Command Status:** Enable or disable commands without permanently deleting them.
*   **Persistent Storage:** Saved commands are stored securely using an embedded database.
*   **Cross-Platform:** Compatible with Windows, macOS, and Linux operating systems.

## 📦 Installation

### Prerequisites

*   [Rust Compiler and Cargo](https://www.rust-lang.org/tools/install) (version 1.70 or higher)

### Build from Source

1.  **Clone the Repository:**
    ```bash
    git clone https://github.com/Mina-Noshy/kemet-command-manager.git
    cd kemet-command-manager
    ```

2.  **Build the Project (Optional if executable already exists):**
    If you have already generated the executable file (e.g., obtained it from a release or built it previously), this step can be skipped.
    ```bash
    cargo build --release
    ```
    The executable will be created in `target/release/`.

3.  **Install the Executable:**
    *   **Linux/macOS:** Copy the binary to a directory already in your system's `PATH` (e.g., `/usr/local/bin`).
        ```bash
        # Example using /usr/local/bin (may require sudo)
        sudo cp target/release/kemet /usr/local/bin/
        # Or copy to your user bin directory if it's in PATH (e.g., ~/bin)
        # cp target/release/kemet ~/bin/
        ```
    *   **Windows:** Copy the `.exe` file to a dedicated folder (e.g., `C:\tools`) and add that folder to your system's `PATH` environment variable.
        ```batch
        # Example commands in Command Prompt or PowerShell (requires administrative privileges)
        # mkdir C:\tools
        # copy target\release\kemet.exe C:\tools\
        # [Manually add C:\tools to PATH via System Properties or use setx]
        # setx PATH "%PATH%;C:\tools"
        ```

4.  **Verify Installation:** Close and reopen your terminal, then run:
    ```bash
    kemet --help
    ```
    If the help message appears, the installation was successful.

## 🛠️ Usage

### Managing Saved Commands

*   **Add a Command:**
    ```bash
    # Long form
    kemet --add --name "alias_name" --command "the actual command to run"
    # Short form
    kemet -a -n "alias_name" -c "the actual command to run"
    ```
    *Example:*
    ```bash
    kemet --add --name "list-all" --command "ls -la"
    kemet --add --name "git-status" --command "git status"
    kemet --add --name "run-tests" --command "cargo test"
    ```

*   **List All Commands:**
    ```bash
    kemet --list
    ```

*   **Run a Saved Command:**
    ```bash
    kemet <alias_name>
    ```
    *Example:*
    ```bash
    kemet list-all
    kemet git-status
    kemet run-tests
    ```

*   **Remove a Command (by Index from `--list`):**
    ```bash
    kemet --remove <index>
    ```

*   **Clear All Commands:**
    ```bash
    kemet --clear
    ```

*   **Set Command Status (by Index from `--list`):**
    *   **Activate:**
        ```bash
        kemet --active <index>
        ```
    *   **Deactivate:**
        ```bash
        kemet --inactive <index>
        ```
    *Deactivated commands will not run when called by their alias.*

### Example Workflows

*   **Development Commands:**
    ```bash
    kemet --add --name "start-server" --command "npm run dev"
    kemet --add --name "build-app" --command "npm run build"
    kemet --add --name "git-sync" --command "git pull origin main && git add . && git commit -m 'Sync'"
    # Execute
    kemet start-server
    kemet build-app
    ```

*   **Docker Commands:**
    ```bash
    kemet --add --name "docker-build" --command "docker build -t myapp ."
    kemet --add --name "docker-run" --command "docker run -p 8080:80 myapp"
    # Execute
    kemet docker-build
    kemet docker-run
    ```
