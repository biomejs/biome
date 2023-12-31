use rustc_hash::FxHashMap;

fn get_utilities_match(spec: &String, class_name: &str) -> Option<bool> {
    if spec.ends_with('$') && class_name == &spec[..spec.len() - 1] {
        return Some(true);
    }
    if class_name.starts_with(spec) && class_name != spec.as_str() {
        return Some(false);
    }
    None
}

fn find_utilities_index(utilities: &[String], class_name: &str) -> Option<usize> {
    let mut matched = false;
    let mut match_index: usize = 0;
    let mut last_size: usize = 0;
    for (i, spec) in utilities.iter().enumerate() {
        match get_utilities_match(spec, class_name) {
            Some(true) => return Some(i),
            Some(false) => {
                let spec_size = spec.chars().count();
                if spec_size > last_size {
                    match_index = i;
                    last_size = spec_size;
                    matched = true;
                }
            }
            _ => {}
        }
    }
    if matched {
        Some(match_index)
    } else {
        None
    }
}

// TODO: detect arbitrary css (e.g. [background:red]), put at the end
pub fn sort_class_name(class_name: &str, utilities: &Vec<String>) -> String {
    let classes = class_name.split_whitespace().collect::<Vec<&str>>();
    let mut unordered_classes: Vec<&str> = Vec::new();
    let mut utilities_map: FxHashMap<usize, Vec<&str>> = FxHashMap::default();
    for class in classes {
        match find_utilities_index(utilities, class) {
            Some(index) => {
                utilities_map.entry(index).or_default().push(class);
            }
            None => {
                unordered_classes.push(class);
            }
        }
    }
    let mut sorted_classes: Vec<&str> = unordered_classes;
    for i in 0..utilities.len() {
        if let Some(classes) = utilities_map.get(&i) {
            let mut abc_classes = classes.clone();
            abc_classes.sort_unstable();
            sorted_classes.extend(abc_classes);
        }
    }
    sorted_classes.join(" ")
}
