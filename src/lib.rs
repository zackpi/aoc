use std::collections::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn y15d01(input: &str) {
    let mut floor: i32 = 0;
    let mut basement: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected character: {}", c),
        }
        if floor == -1 && basement == 0 {
            basement = (i as i32) + 1;
        }
    }
    log(&format!("y15d01p1: {}", floor));
    log(&format!("y15d01p2: {}", basement));
}

#[wasm_bindgen]
pub fn y15d02(input: &str) {
    let mut paper: i32 = 0;
    let mut ribbon: i32 = 0;
    for line in input.lines() {
        let dims: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];
        let slack = l * w.min(w * h).min(h * l);
        paper += 2 * (l * w + w * h + h * l) + slack;

        let longest = dims.iter().max().unwrap();
        ribbon += l * w * h + 2 * (l + w + h - longest);
    }

    log(&format!("y15d02p1: {}", paper));
    log(&format!("y15d02p2: {}", ribbon));
}

#[wasm_bindgen]
pub fn y15d03(input: &str) {
    let mut houses = HashSet::new();
    let mut santa = (0, 0);
    for c in input.chars() {
        match c {
            '^' => santa.1 += 1,
            'v' => santa.1 -= 1,
            '>' => santa.0 += 1,
            '<' => santa.0 -= 1,
            _ => panic!("unexpected character: {}", c),
        }
        houses.insert(santa);
    }
    log(&format!("y15d03p1: {}", houses.len()));

    let mut houses = HashSet::new();
    let mut santa = (0, 0);
    let mut robo = (0, 0);
    for (i, c) in input.chars().enumerate() {
        let mut step = (0, 0);
        match c {
            '^' => step.1 += 1,
            'v' => step.1 -= 1,
            '>' => step.0 += 1,
            '<' => step.0 -= 1,
            _ => panic!("unexpected character: {}", c),
        }
        let current = if i % 2 == 0 {
            santa.0 += step.0;
            santa.1 += step.1;
            santa
        } else {
            robo.0 += step.0;
            robo.1 += step.1;
            robo
        };
        houses.insert(current);
    }

    log(&format!("y15d03p2: {}", houses.len()));
}

#[wasm_bindgen]
pub fn y15d04(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    let key = input.trim();
    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{}", key, i));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            if part1 == 0 {
                part1 = i;
            }
            if hash[2] == 0 {
                part2 = i;
                break;
            }
        }
        i += 1;
    }

    log(&format!("y15d04p1: {}", part1));
    log(&format!("y15d04p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d05(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for line in input.lines() {
        let mut vowels = 0;
        let mut double = false;
        let mut naughty = false;
        let mut last = ' ';

        let mut pairs = HashMap::new();
        let mut pair = false;
        let mut sandwich = false;
        let mut last2 = ' ';
        for (i, char) in line.chars().enumerate() {
            // part 1 checks
            match char {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => (),
            }
            if last == char {
                double = true;
            }
            match last {
                'a' => {
                    if char == 'b' {
                        naughty = true;
                    }
                }
                'c' => {
                    if char == 'd' {
                        naughty = true;
                    }
                }
                'p' => {
                    if char == 'q' {
                        naughty = true;
                    }
                }
                'x' => {
                    if char == 'y' {
                        naughty = true;
                    }
                }
                _ => (),
            }

            // part 2 checks
            if last2 == char {
                sandwich = true;
            }
            if !pair {
                let curr = format!("{}{}", last, char);
                if let Some(&j) = pairs.get(&curr) {
                    if j < i - 1 {
                        pair = true;
                    }
                } else {
                    pairs.insert(curr, i);
                }
            }

            last2 = last;
            last = char;
        }

        if vowels >= 3 && double && !naughty {
            part1 += 1;
        }
        if sandwich && pair {
            part2 += 1;
        }
    }

    log(&format!("y15d05p1: {}", part1));
    log(&format!("y15d05p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d06(input: &str) {
    let mut lights1 = vec![vec![false; 1000]; 1000];
    let mut lights2 = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        let fields: Vec<&str> = line.rsplitn(4, " ").collect();
        let from: Vec<usize> = fields[2].split(',').map(|x| x.parse().unwrap()).collect();
        let to: Vec<usize> = fields[0].split(',').map(|x| x.parse().unwrap()).collect();
        let action = fields[3];
        for x in from[0]..=to[0] {
            for y in from[1]..=to[1] {
                match action {
                    "turn on" => {
                        lights1[x][y] = true;
                        lights2[x][y] += 1;
                    }
                    "turn off" => {
                        lights1[x][y] = false;
                        lights2[x][y] = (lights2[x][y] - 1).max(0);
                    }
                    "toggle" => {
                        lights1[x][y] = !lights1[x][y];
                        lights2[x][y] += 2;
                    }
                    _ => panic!("unexpected action: {}", action),
                }
            }
        }
    }

    let part1 = lights1.iter().flatten().filter(|&&x| x).count();
    let part2 = lights2.iter().flatten().sum::<i32>();

    log(&format!("y15d06p1: {}", part1));
    log(&format!("y15d06p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d07(input: &str) {
    let mut gates = HashMap::new();
    let mut values = HashMap::new();
    for line in input.lines() {
        let fields: Vec<&str> = line.split(" -> ").collect();
        gates.insert(fields[1], fields[0]);
    }

    fn resolve<'a>(
        gates: &HashMap<&'a str, &'a str>,
        values: &mut HashMap<&'a str, u16>,
        wire: &'a str,
    ) -> u16 {
        if let Ok(value) = wire.parse() {
            return value;
        } else if let Some(&value) = values.get(wire) {
            return value;
        }

        let gate = gates.get(wire).unwrap();
        let fields: Vec<&str> = gate.split(' ').collect();
        let value = match fields.len() {
            1 => resolve(gates, values, fields[0]),
            2 => match fields[0] {
                "NOT" => !resolve(gates, values, fields[1]),
                _ => panic!("unexpected op: {}", fields[0]),
            },
            3 => {
                let wire1 = resolve(gates, values, fields[0]);
                let wire2 = resolve(gates, values, fields[2]);
                match fields[1] {
                    "AND" => wire1 & wire2,
                    "OR" => wire1 | wire2,
                    "LSHIFT" => wire1 << wire2,
                    "RSHIFT" => wire1 >> wire2,
                    _ => panic!("unexpected op: {}", fields[1]),
                }
            }
            _ => panic!("unexpected gate: {}", gate),
        };

        values.insert(wire, value);
        value
    }

    let part1 = resolve(&gates, &mut values, "a");
    values.clear();
    values.insert("b", part1);
    let part2 = resolve(&gates, &mut values, "a");

    log(&format!("y15d07p1: {}", part1));
    log(&format!("y15d07p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d08(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d08p1: {}", part1));
    log(&format!("y15d08p2: {}", part2));
}

// #[wasm_bindgen]
// pub fn y15d04(input: &str) {
//     let mut part1: i32 = 0;
//     let mut part2: i32 = 0;
//     log(&format!("y15d04p1: {}", part1));
//     log(&format!("y15d04p2: {}", part2));
// }
