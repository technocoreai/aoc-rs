#[macro_export]
macro_rules! aoc_main {
    ($p1:ident $(, $p2:ident)?) => {
        use std::fs;

        let filename = format!("input/{}.txt", module_path!().replace("day", ""));
        let input = fs::read_to_string(filename).unwrap();

        let parts = [$p1 $(, $p2) ?];
        for (i, part) in parts.iter().enumerate() {
            let id = i + 1;
            let solution = part(input.as_str());
            println!("Part {id}: {solution}");
        }
    };
}
