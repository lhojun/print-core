use std::process::Command;

pub fn get_printer_list() -> Result<Vec<String>, String> {
    let command = match Command::new("lpstat").arg("-e").output() {
        Ok(command) => command,
        Err(e) => return Err(e.to_string()),
    };
    if !command.status.success() {
        let error_message = String::from_utf8_lossy(&command.stderr).to_string();
        return Err(error_message);
    }
    let output_string = String::from_utf8_lossy(&command.stdout);
    let printers: Vec<&str> = output_string.split_inclusive("\n").collect();
    let mut result = Vec::with_capacity(printers.len());
    for printer in printers {
        result.push(printer.to_string());
    }
    Ok(result)
}
