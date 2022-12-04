#[macro_export]
macro_rules! aoc_main {
    ($p1:ident $(, $p2:ident)?) => {
        use std::fs;

        let filename = format!("input/{}.txt", module_path!().replace("day", ""));
        let contents = fs::read_to_string(filename).unwrap();
        let input = contents.as_str().trim();

        let parts = [$p1 $(, $p2) ?];
        for (i, part) in parts.iter().enumerate() {
            let id = i + 1;
            let solution = part(input);
            println!("Part {id}: {solution}");
        }
    };
}
