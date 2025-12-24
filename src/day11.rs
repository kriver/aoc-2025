use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::util::load;

type Devices = HashMap<String, Vec<String>>;
type DeviceLevels = HashMap<String, usize>; // level of a device

pub fn input() -> Devices {
    let lines: Vec<String> = load("data/day11.txt");
    lines
        .into_iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap()[0..3].to_string();
            let connections = parts.map(|p| p.to_string()).collect::<Vec<String>>();
            (name, connections)
        })
        .collect()
}

fn all_paths(devices: &Devices, start: &str, finish: &str, levels: &DeviceLevels) -> usize {
    let finish_level = levels.get(finish).unwrap();
    let mut todo: Vec<_> = vec![start.to_string()];
    let mut cnt = 0;
    loop {
        if todo.is_empty() {
            break;
        }
        let current = todo.pop().unwrap();
        let current_level = levels.get(&current).unwrap();
        if current_level >= finish_level {
            continue;
        }
        let nexts = devices.get(&current);
        for next in nexts.unwrap_or(&vec![]) {
            if next == finish {
                cnt += 1;
            } else {
                todo.push(next.clone());
            }
        }
    }
    cnt
}

fn build_levels(devices: &Devices) -> DeviceLevels {
    let start = "svr".to_owned();
    let mut lvl = 0;
    let mut levels: DeviceLevels = HashMap::from([(start.clone(), lvl)]);
    let mut todo = vec![start];
    while !todo.is_empty() {
        let mut ld_next = HashSet::new();
        lvl += 1;
        for device in todo.drain(..) {
            let next_devices = devices.get(&device);
            for next in next_devices.unwrap_or(&vec![]) {
                levels.insert(next.clone(), lvl);
                ld_next.insert(next.clone());
            }
        }
        todo = ld_next.iter().cloned().collect();
    }
    levels
}

fn all_paths_multi(
    devices: &Devices,
    start: Vec<&str>,
    finish: Vec<&str>,
    levels: &DeviceLevels,
    res: Vec<usize>,
) -> Vec<usize> {
    finish
        .into_iter()
        .map(|f| {
            start
                .iter()
                .enumerate()
                .map(|(i, s)| res[i] * all_paths(devices, s, f, levels))
                .sum()
        })
        .collect()
}

pub fn part1(devices: Devices) -> usize {
    let levels = build_levels(&devices);
    all_paths(&devices, "you", "out", &levels)
}

pub fn part2(devices: Devices) -> usize {
    // Turn data into dot:
    // gawk '
    //   BEGIN { FS=":? "; print "digraph "test" {"};
    //   END {print "}"};
    //   {for (i=2;i<=NF;i++) {print $1" -> "$i";"}}'

    let levels = build_levels(&devices);
    let mut res = vec![1];
    res = all_paths_multi(&devices, vec!["svr"], vec!["fft"], &levels, res);
    res = all_paths_multi(
        &devices,
        vec!["fft"],
        vec!["amt", "kmc", "fxz", "njx", "opq"],
        &levels,
        res,
    );
    res = all_paths_multi(
        &devices,
        vec!["amt", "kmc", "fxz", "njx", "opq"],
        vec!["hnh", "zle", "tyl", "vic", "ibk"],
        &levels,
        res,
    );
    res = all_paths_multi(
        &devices,
        vec!["hnh", "zle", "tyl", "vic", "ibk"],
        vec!["fkd", "xba", "ekg", "mmm" /*"hud"*/],
        &levels,
        res,
    );
    res = all_paths_multi(
        &devices,
        vec!["fkd", "xba", "ekg", "mmm" /*"hud"*/],
        vec!["dac"],
        &levels,
        res,
    );
    res = all_paths_multi(&devices, vec!["dac"], vec!["out"], &levels, res);
    res[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 652);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 362956369749210);
    }
}
