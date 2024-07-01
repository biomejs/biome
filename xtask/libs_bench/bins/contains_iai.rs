use regex::Regex;
use std::{collections::HashMap, process::Command};

fn main() {
    let result = Command::new("cargo")
        .args(["bench", "-p", "xtask_libs_bench"])
        .output();
    let re = Regex::new(r"(?P<NAME>\w*?)\s*Instructions:\s*(?P<INST>\d*).*\n\s*L1 Accesses:\s*(?P<L1>\d*).*\n\s*L2 Accesses:\s*(?P<L2>\d*).*\n\s*RAM Accesses:\s*(?P<RAM>\d*).*\n\s*Estimated Cycles:\s*(?P<CYCLES>\d*).*").unwrap();
    let stdout = String::from_utf8(result.unwrap().stdout).unwrap();
    let mut tests: HashMap<&str, (isize, isize, isize, isize, isize)> = HashMap::new();

    for capture in re.captures_iter(stdout.as_str()) {
        let name = capture.name("NAME").unwrap().as_str();
        let inst = capture.name("INST").unwrap().as_str().parse().unwrap();
        let l1 = capture.name("L1").unwrap().as_str().parse().unwrap();
        let l2 = capture.name("L2").unwrap().as_str().parse().unwrap();
        let ram = capture.name("RAM").unwrap().as_str().parse().unwrap();
        let cycles = capture.name("CYCLES").unwrap().as_str().parse().unwrap();
        tests.insert(name, (inst, l1, l2, ram, cycles));
    }

    for (k, v_setup) in tests.iter() {
        if let Some(name) = k.strip_suffix("_setup") {
            let v_all = tests[name];
            println!("{name}");
            println!(
                "\tdiff inst: {} is {} - {}",
                v_all.0 - v_setup.0,
                v_all.0,
                v_setup.0
            );
            println!(
                "\tdiff l1: {} is {} - {}",
                v_all.1 - v_setup.1,
                v_all.1,
                v_setup.1
            );
            println!(
                "\tdiff l2: {} is {} - {}",
                v_all.2 - v_setup.2,
                v_all.2,
                v_setup.2
            );
            println!(
                "\tdiff ram: {} is {} - {}",
                v_all.3 - v_setup.3,
                v_all.3,
                v_setup.3
            );
            println!(
                "\tdiff cycles: {} is {} - {}",
                v_all.4 - v_setup.4,
                v_all.4,
                v_setup.4
            );
        }
    }
}
