use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn setup(project_path: &Path) {
    println!("seting up project in directory {:?}", project_path);

    let project_name = project_path.file_name().unwrap();

    let toml_content = include_str!("../assets/turkey.toml");
    let toml_content = toml_content.replace("<name>", project_name.to_str().unwrap());

    let toml_path = project_path.join("turkey.toml");
    let mut toml_file = File::create(toml_path).unwrap();
    toml_file.write_all(toml_content.as_bytes()).unwrap();

    let src_dir = project_path.join("src");
    fs::create_dir(&src_dir).unwrap();

    let main_content = include_str!("../assets/main.tky");
    let main_path = src_dir.join("main.tky");

    let mut main_file = File::create(main_path).unwrap();
    main_file.write_all(main_content.as_bytes()).unwrap();

    println!("project has been made!\n have fun using my lang");
}
