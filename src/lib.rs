mod unix;

pub fn get_printer_list() -> Vec<String> {
    if cfg!(unix) {
        match unix::get_printer_list() {
            Ok(printer_list) => return printer_list,
            Err(e) => panic!("{}", e),
        };
    } else {
        panic!("Unsupported platform");
    }
}
