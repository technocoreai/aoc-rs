#[macro_export]
macro_rules! aoc_main {
    ($p1:ident $(, $p2:ident)?) => {
        let filename = format!("input/{}.txt", module_path!().replace("day", ""));
        let contents = std::fs::read_to_string(filename).unwrap();
        let input = contents.as_str().trim_end();

        let parts = [$p1 $(, $p2) ?];
        for (i, part) in parts.iter().enumerate() {
            let id = i + 1;
            let solution = part(input);
            println!("Part {id}: {solution}");
        }
    };
}

pub fn parse<T, F: FnOnce() -> Option<T>>(error_msg: &str, parse_fn: F) -> T {
    parse_fn().unwrap_or_else(|| panic!("{}", error_msg))
}

pub fn parse_obj<T, F: FnOnce() -> Option<T>>(kind: &str, value: &str, parse_fn: F) -> T {
    let error_msg = format!("Invalid {}: {}", kind, value);
    parse(error_msg.as_str(), parse_fn)
}
