use std::process::Command;
use nix::unistd::getuid;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    let is_root = getuid().is_root();

    let themes = get_themes();
    println!("[available themes]");
    for theme in themes.iter() {
        println!("{}", theme);
    }
    print!("\n");
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => println!("failed to flush stdout")
    };

    let theme: String;
    loop {
        print!("what theme do you want> ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_) => println!("failed to flush stdout")
        };

        let input = input();

        if themes.iter().any(|i| i==&input) {
            theme = String::from(input.trim());
            break;
        } else {
            println!("invalid theme");
        }
    }
    print!("\n");
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => println!("failed to flush stdout")
    };

    if is_root {
        println!("settings themes for every user...")
    } else {
        println!("setting themes for current user only (try running as root)")
    }
    print!("\n");
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => println!("failed to flush stdout")
    };

    let command =
        Command::new("/usr/bin/flatpak")
            .arg("override")
            .arg(  if is_root {"--system"} else {"--user"} )
            .arg("--env=GTK_THEME=".to_owned() + &theme)
            .output()
            .expect("failed to execute /usr/bin/flatpak");

    let error = String::from_utf8_lossy(&command.stderr);
    if error.len() <= 0 {
        println!("successfully applied theme");
    } else {
        println!("an error occurred: {}", error);
    }
}

fn get_themes() -> Vec<String> {
    let command =
        Command::new("/usr/bin/flatpak")
            .arg("list")
            .arg("--runtime")
            .output()
            .expect("failed to execute /usr/bin/flatpak");

    let mut themes: Vec<String> = Vec::new();
    let output = String::from_utf8_lossy(&command.stdout);

    let mut raw_appdata: Vec<String> = Vec::new();
    for app in output.trim().split("\n") {
        raw_appdata.push(String::from(app));
    }

    for theme in raw_appdata.iter() {
        let theme_name: String = String::from(theme.split("\t").collect::<Vec<&str>>()[1]);

        let name_part: Vec<&str> = theme_name.split(".").collect();
        let name_full = name_part[0].to_owned() + name_part[1] + name_part[2];
        if name_full == "orggtkGtk3theme" || name_full == "orgkdeKStyle" {
            themes.push(String::from(name_part[3].trim()));
        }
    }

    themes
}

fn input() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
        return input_string.trim().to_string();
}
