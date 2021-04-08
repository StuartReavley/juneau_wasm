use std::collections::HashMap;
use regex::Regex;


mod test;


pub struct Template {}

impl Template {

    pub fn render(template:&str, values:HashMap<String, String>) -> String {
        let regex = Regex::new(r"\$\{([^}]*)\}").unwrap();
        let matches:Vec<(usize, usize)> = regex
            .find_iter(template)
            .map(|m| (m.start(), m.end()))
            .collect();

        let mut last = 0;
        let mut result = String::with_capacity(template.len());
        for (start, end) in matches {
            let arg = &template[(start + 2)..(end - 1)];
            result.push_str(&template[last..start]);
            result.push_str(values.get(arg).expect(&format!("Cannot find template argument [{}]", arg)));
            last = end;
        }
        result.push_str(&template[last..]);
        result
    }
}