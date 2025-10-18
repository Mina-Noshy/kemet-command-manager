use crate::database::manager::CommandManager;
use crate::errors::app_error::AppError;
use crate::utils::command_runner::{get_input, run_command};
use std::env;

pub fn handle_cli(command_manager: &mut CommandManager) -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--add") | Some("-a") => {
            let mut name: Option<String> = None;
            let mut command_text: Option<String> = None;

            // Iterate over arguments in pairs (flag, value)
            let mut i = 2;
            while i < args.len() {
                match args.get(i).map(|s| s.as_str()) {
                    Some("--name") | Some("-n") => {
                        if let Some(value) = args.get(i + 1) {
                            name = Some(value.to_string());
                            i += 2;
                        } else {
                            println!("Missing value for --name/-n");
                            break;
                        }
                    }
                    Some("--command") | Some("-c") => {
                        if let Some(value) = args.get(i + 1) {
                            command_text = Some(value.to_string());
                            i += 2;
                        } else {
                            println!("Missing value for --command/-c");
                            break;
                        }
                    }
                    Some(flag) => {
                        println!("Unknown flag: {}", flag);
                        i += 1;
                    }
                    None => break,
                }
            }

            // Check if both name and command text are provided
            match (name, command_text) {
                (Some(n), Some(d)) => {
                    command_manager.add_command(n, d)?;
                }
                (None, Some(_)) => println!("Please provide a name using --name or -n"),
                (Some(_), None) => {
                    println!("Please provide a command text using --command or -c")
                }
                (None, None) => {
                    println!("Please provide both name (--name/-n) and command text (--command/-c)")
                }
            }
        }
        Some("--list") | Some("-l") => {
            let commands = command_manager.get_all_commands()?;
            if commands.is_empty() {
                println!("No commands found.");
            } else {
                println!("Commands:");
                for command in commands {
                    let status = if command.is_active { "✓" } else { "○" };
                    println!(
                        "{} [{}] {} - {}",
                        status, command.id, command.name, command.command
                    );
                }
            }
        }
        Some("--active") | Some("-ma") => {
            let id_str = args.get(2).map(|s| s.to_string()).unwrap();
            if let Ok(id) = id_str.trim().parse::<u64>() {
                if command_manager.mark_command_active(id)? {
                    println!("Command {} marked as active.", id);
                } else {
                    println!("Command with ID {} not found.", id);
                }
            } else {
                println!("Invalid command ID format.");
            }
        }
        Some("--inactive") | Some("-mi") => {
            let id_str = args.get(2).map(|s| s.to_string()).unwrap();
            if let Ok(id) = id_str.trim().parse::<u64>() {
                if command_manager.mark_command_inactive(id)? {
                    println!("Command {} marked as inactive.", id);
                } else {
                    println!("Command with ID {} not found.", id);
                }
            } else {
                println!("Invalid command ID format.");
            }
        }
        Some("--remove") | Some("-r") => {
            let id_str = args.get(2).map(|s| s.to_string()).unwrap();
            if let Ok(id) = id_str.trim().parse::<u64>() {
                if command_manager.remove_command(id)? {
                    println!("Command {} removed.", id);
                } else {
                    println!("Command with ID {} not found.", id);
                }
            } else {
                println!("Invalid command ID format.");
            }
        }
        Some("--clear") | Some("-x") => {
            let confirmation = get_input("Are you sure you want to clear all commands? (y/N): ");
            if confirmation.to_lowercase() != "y" {
                println!("Clear operation cancelled.");
                return Ok(());
            }
            command_manager.clear_all_commands()?;
        }
        Some("--help") | Some("-h") | None => {
            print_help();
        }
        _ => {
            if let Some(name) = args.get(1) {
                let cli_command = command_manager.get_command_by_name(name)?;
                if let Some(command) = cli_command {
                    if command.is_active {
                        run_command(command.command);
                    } else {
                        println!("Command not active. Please activate the command and try again!");
                    }
                } else {
                    println!("Command name not found: {}", name);
                }
            } else {
                println!("No command name provided. Use --help for usage.");
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"
Kemet CLI Helper App

📌 Basic Commands:
  --add, -a           Add a new command
  --list, -l          List all saved commands
  --active, -ma       Mark a command as active
  --inactive, -mi     Mark a command as inactive
  --remove, -r        Remove a command
  --clear, -x         Clear all commands
  --help, -h          Show this help message

📌 Adding a Command:
  --name, -n          Specify the command name (required for add)
  --command, -c       Specify the actual command text (required for add)
  Examples:
    kemet --add --name "create-console-app" --command "dotnet new console"
    kemet --add --name "git-status" --command "git status"
    kemet --add --name "list-files" --command "ls -la"
    kemet --add --name "create-dir" --command "mkdir NewFolder"
    kemet --add --name "git-commit" --command "git add . && git commit -m 'Initial commit'"
    kemet --add --name "npm-install" --command "npm install"
    kemet --add --name "docker-build" --command "docker build -t myapp ."
    kemet --add --name "start-server" --command "npm run dev"
    kemet --add --name "backup" --command "cp -r source/ backup/"
    kemet --add --name "update-packages" --command "sudo apt update && sudo apt upgrade"

📌 Executing a Command:
  [command name]      Run a previously added command by its name
  Examples:
    kemet create-console-app
    kemet git-status
    kemet list-files

📌 Notes:
  - Commands have an active/inactive status. Only active commands can be executed.
  - On Windows, commands run in PowerShell; on Linux/macOS, commands run in the default shell.
  - Use -l or --list to see all commands and their current status (✓ active, ○ inactive).

🔹 Status symbols in the list:
  ✓  Active command
  ○  Inactive command
"#
    );
}
