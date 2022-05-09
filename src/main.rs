use std::env;
use std::process;
use systemstat::Platform;
mod colors;
mod fields;

// Simple system fetch tool written in Rust.
fn main() {

    let args: Vec<String> = env::args().collect();
    let mut show_kern_name = false;
    let ascii_tree: String;

    ascii_tree = format!(

        "   {yellow}                               .o        {reset}
            {yellow}                            °O@@@        {reset}
            {cyan}     RRRRRRRRRRRRRRRR    {yellow}°o@@@@@@        {reset}
            {cyan}     RRRRRRRRRRRRRRRRR. {yellow}O@@@@@@@@°       {reset}
            {cyan}     RRRR         {cyan}RRRR {yellow}*@@@@@@@@@#       {reset}
            {cyan}     RRRR       {yellow}.** {cyan}RRR {yellow}*@@@@@@@@@       {reset}
            {cyan}     RRRR    {yellow}.*@@@@° {cyan}RRR {yellow}@@@@@@@@@@      {reset}
            {cyan}     RRRR {yellow}°o@@@@@@@ {cyan}RRR {yellow}*@@@@@@@@@@      {reset}
            {cyan}     RRRR {yellow}#@@@@@@@ {cyan}RRR {yellow}*.@@@@@@@@@@.     {reset}
            {yellow}  *@ {cyan}RRRRRRRRRRRRRRRR {yellow}*@@@@@@@@@@@@@     {reset}
            {yellow}*o@@ {cyan}RRRRRRRRRRRRRRRR {yellow}.#@@@@@@@@@@@@@.   {reset}
            {yellow} #@@ {cyan}RRRR {yellow}@@@@@@ {cyan}RRRR {yellow}*@@@@@@@@@@@@@@o   {reset}
            {yellow}   @ {cyan}RRRR {yellow}@@@@@@@@ {cyan}RRRR {yellow}#@@@@@@@@@@@@@   {reset}
            {cyan}     RRRR {yellow}o#@@@@@@@ {cyan}RRRR {yellow}O@@@@@@@@@@@@.   {reset}
            {cyan}     RRRR      {yellow}<@@@@ {cyan}RRRR {yellow}.o@@@@@@@@@@O   {reset}
            {cyan}     RRRR            {cyan}RRRR {yellow}°oO@@@@@@@@@@   {reset}
            {cyan}     RRRR            {cyan}RRRR    {yellow}.°*O#@@@@@@   {reset}
            {yellow}                                  .@@@@@.   {reset}
            {yellow}                                      ***   {reset}
        ",
        cyan = colors::cyan,
        yellow = colors::yellow,
        reset = colors::reset,
    );

    // Skip first arg as that is the program command
    for arg in &args[1..] {

        // Convert to string slice for the comparisons
        let arg = &arg[..];

        match arg {
            "--help" | "-h" => {
                help_message()
            }

            "--kernel-name" | "-k" => {
                show_kern_name = true;
            }

            _ => {
                invalid_option(arg.to_string());
            }
        }
    }

    let ascii_tree = split_by_newline(ascii_tree);

    let stat = systemstat::System::new();

    let mut data_list: Vec<String> = Vec::new();
    data_list.push(String::from(""));
    data_list.push(String::from(""));

    if let Ok(value) = fields::get_user_host_name() {
            data_list.push(value.0);
            data_list.push(value.1);
    };


    if let Ok(value) = fields::get_distro_name() {
        data_list.push(value);
    };

    // Kernel name

    if let Ok(value) = fields::get_kernel(show_kern_name) {
        data_list.push(value);
    };

    // Shell

    if let Ok(value) = fields::get_shell() {
        data_list.push(value);
    };

    // Uptime

    if let Ok(value) = stat.uptime() {
        data_list.push(fields::format_uptime(value));
    };

    // Memory

    if let Ok(value) = stat.memory() {
        data_list.push(fields::format_memory(value));
    };

    // Battery

    if let Ok(value) = stat.battery_life() {
        data_list.push(fields::format_battery(value));
    };

    println!();
    print_left_to_right(ascii_tree, data_list);
    println!();
}

// Print two vectors of strings side to side
fn print_left_to_right(left: Vec<String>, right: Vec<String>) {
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {left_len} else {right_len};

    for i in 0..max_len {
        if i < left_len {
            print!(" {}", left[i]);
        }
        if i < right_len {
            print!(" {}", right[i]);
        }
        println!()
    }
}

// Split a multi-line string into several ones separated by the newline
fn split_by_newline(ascii_art: String) -> Vec<String> {
    let mut split: Vec<String> = Vec::new();
    let mut last_index = 0;

    let bytes = ascii_art.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            split.push(ascii_art[last_index..i].trim().to_string());
            last_index = i;
        }
    }

    split
}

fn help_message() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Usage:");
    println!("  {bold}{cyan}risifetch{reset} [options]",
            cyan = colors::cyan,
            reset = colors::reset,
            bold = colors::bold,
            );
    println!();
    println!("OPTIONS");
    println!("  -h, --help     Display this help message");
    println!("  -k, --kernel-name     Display the kernel name");
    println!();
    println!("risifetch {}", version);
    println!("Report bugs to https://github.com/risiOS/risifetch/issues");
    process::exit(1)
}

fn invalid_option(option: String) {
    println!("Unrecognized option '{}'", option);
    println!("Try 'risifetch --help' for more information.");
    process::exit(1)
}
