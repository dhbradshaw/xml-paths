use xml_paths::paths;

fn main() {
    // Read in path to xml file from the command line
    let path = std::env::args()
        .nth(1)
        .expect("Please provide a path to an xml file");
    let paths = paths(&path).expect("Failed to read xml file");
    for path in paths {
        println!("{}", path);
    }
}
