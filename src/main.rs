use std::process::Command;
use nix::unistd::getuid;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    let is_root = getuid().is_root();

    let themes = get_themes();
    println!("[available themes via org.gtk.Gtk3theme]");
    for theme in themes.iter() {
        println!("{}", theme);
    }
    print!("\n");
    stdout().flush();

    let mut theme: String = String::new();
    loop {
        print!("what theme do you want> ");
        stdout().flush();

        let input = input();

        if themes.iter().any(|i| i==&input) {
            theme = String::from(input.trim());
            break;
        } else {
            println!("invalid theme");
        }
    }
    print!("\n");
    stdout().flush();

    if is_root {
        println!("settings themes for every user...")
    } else {
        println!("setting themes for current user only (try running as root)")
    }
    print!("\n");
    stdout().flush();

    let apps = get_application_names();
    for app in apps.iter() {
        let command =
            Command::new("/usr/bin/flatpak")
                .arg("override")
                .arg(  if is_root {"--system"} else {"--user"} )
                .arg("--env=GTK_THEME=".to_owned() + &theme)
                .arg(app)
                .output()
                .expect("failed to execute /usr/bin/flatpak");

        let error = String::from_utf8_lossy(&command.stderr);
        if error.len() <= 0 {
            println!("set theme for {}", app);
        } else {
            println!("an error occurred: {}", error);
        }
    }
}

fn get_application_names() -> Vec<String> {
    let output =
        Command::new("/usr/bin/flatpak")
            .arg("list")
            .arg("--app")
            .output()
            .expect("failed to execute /usr/bin/flatpak");

    let output = String::from_utf8_lossy(&output.stdout);

    let mut raw_appdata: Vec<String> = Vec::new();
    let mut full_apps_name: Vec<String> = Vec::new();

    for app in output.trim().split("\n") {
        raw_appdata.push(String::from(app));
    }

    for app in raw_appdata.iter() {
        let fullname: String = app
            .split("\t")
            .collect::<Vec<&str>>()[1]
            .trim()
            .to_string();

        full_apps_name.push(fullname);
    }

    full_apps_name
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
        if name_part[0].to_owned() + name_part[1] + name_part[2] == "orggtkGtk3theme" {
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
