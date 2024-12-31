use ndarray::prelude::*;
use serde_json::Value;
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
    let mut part1: usize = 0;
    let mut part2: usize = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        chars.next_back();
        chars.next();
        let mut enc = 6;
        let mut dec = 0;
        while let Some(c) = chars.next() {
            match c {
                '\\' => match chars.next().unwrap() {
                    '\\' | '"' => {
                        dec += 1;
                        enc += 4;
                    }
                    'x' => {
                        chars.next();
                        chars.next();
                        dec += 1;
                        enc += 5;
                    }
                    _ => panic!("unexpected escape: {}", c),
                },
                _ => {
                    dec += 1;
                    enc += 1;
                }
            }
        }
        part1 += line.len() - dec;
        part2 += enc - line.len();
    }

    log(&format!("y15d08p1: {}", part1));
    log(&format!("y15d08p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d09(input: &str) {
    let mut graph: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    for line in input.lines() {
        let fields: Vec<&str> = line.split(" = ").collect();
        let cities: Vec<&str> = fields[0].split(" to ").collect();
        let dist: i32 = fields[1].parse().unwrap();

        let city = graph.entry(cities[0]).or_insert(vec![]);
        city.push((cities[1], dist));

        let city = graph.entry(cities[1]).or_insert(vec![]);
        city.push((cities[0], dist));
    }

    let root = &mut vec![];
    for city in graph.keys().cloned() {
        root.push((city, 0));
    }
    graph.insert("root", root.clone());

    fn travel<'a>(
        graph: &HashMap<&str, Vec<(&'a str, i32)>>,
        path: &mut Vec<&'a str>,
        dist: i32,
    ) -> (i32, i32) {
        if path.len() == graph.len() {
            return (dist, dist);
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        let city = path.last().unwrap();
        for (next, edge) in graph.get(city).unwrap() {
            if !path.contains(&next) {
                path.push(next);
                let (mindist, maxdist) = travel(graph, path, dist + edge);
                path.pop();
                min = min.min(mindist);
                max = max.max(maxdist);
            }
        }

        (min, max)
    }

    let mut path = vec!["root"];
    let (part1, part2) = travel(&graph, &mut path, 0);

    log(&format!("y15d09p1: {}", part1));
    log(&format!("y15d09p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d10(input: &str) {
    fn look_say(input: &str) -> String {
        let mut next = String::new();
        let mut chars = input.chars();
        let mut count = 1;
        let mut digit = chars.next().unwrap();
        for c in chars {
            if c == digit {
                count += 1;
            } else {
                next.push_str(&format!("{}{}", count, digit));
                count = 1;
                digit = c;
            }
        }

        next.push_str(&format!("{}{}", count, digit));
        next
    }

    let mut input = input.trim().to_string();
    for _ in 0..40 {
        input = look_say(&input);
    }
    let part1 = input.len() as i32;
    for _ in 0..10 {
        input = look_say(&input);
    }
    let part2 = input.len() as i32;

    log(&format!("y15d10p1: {}", part1));
    log(&format!("y15d10p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d11(input: &str) {
    fn valid(password: &str) -> bool {
        let mut straight = false;
        let mut pairs = false;
        let mut pair = 0;
        let mut b = 0;
        let mut a = 0;
        for char in password.chars() {
            let c = char as u32;
            if char <= 'z' && c - b == 1 && b - a == 1 {
                straight = true;
            }
            if c == b && c != a && pair != c {
                pairs = pair != 0;
                pair = c;
            }
            a = b;
            b = c;
        }
        straight && pairs
    }

    fn next(password: &str) -> String {
        let chars = password.chars().rev();
        let mut next = String::new();
        let mut carry = true;
        for c in chars {
            if carry {
                let nextc = match c {
                    'h' => 'j',
                    'k' => 'm',
                    'n' => 'p',
                    'z' => 'a',
                    _ => (c as u8 + 1) as char,
                };
                next.push(nextc);
                carry = nextc == 'a';
            } else {
                next.push(c);
            }
        }
        next.chars().rev().collect()
    }

    let mut part1 = input.trim().to_string();
    while !valid(&part1) {
        part1 = next(&part1);
    }
    let mut part2 = next(&part1);
    while !valid(&part2) {
        part2 = next(&part2);
    }

    log(&format!("y15d11p1: {}", part1));
    log(&format!("y15d11p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d12(input: &str) {
    let json: Value = serde_json::from_str(input).unwrap();

    fn sum(json: &Value, skip: bool) -> i32 {
        match json {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(n) => n.as_i64().unwrap() as i32,
            Value::String(_) => 0,
            Value::Array(a) => a.iter().map(|x| sum(x, skip)).sum(),
            Value::Object(o) => {
                if skip && o.values().any(|x| x == "red") {
                    return 0;
                }
                o.values().map(|x| sum(x, skip)).sum()
            }
        }
    }

    log(&format!("y15d12p1: {}", sum(&json, false)));
    log(&format!("y15d12p2: {}", sum(&json, true)));
}

#[wasm_bindgen]
pub fn y15d13(input: &str) {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let fields = line.split(" ").collect::<Vec<&str>>();
        let person1 = fields[0];
        let action = fields[2];
        let value = fields[3].parse::<i32>().unwrap();
        let person2 = fields[10].trim_end_matches('.');
        let value = if action == "gain" { value } else { -value };

        let entry = graph.entry(person1).or_insert(HashMap::new());
        entry.insert(person2, value);
    }

    // add reverse edges
    for (person, edges) in graph.clone() {
        for (next, edge) in edges {
            let entry = graph.get_mut(next).unwrap().get_mut(person).unwrap();
            *entry += edge;
        }
    }

    fn visit<'a>(
        graph: &HashMap<&str, HashMap<&'a str, i32>>,
        path: &mut Vec<&'a str>,
        happ: i32,
    ) -> i32 {
        if path.len() == graph.len() {
            let first = path.first().unwrap();
            let last = path.last().unwrap();
            let happ = happ + graph.get(first).unwrap().get(last).unwrap();
            return happ;
        }

        let mut max = i32::MIN;
        let person = path.last().unwrap();
        for (next, edge) in graph.get(person).unwrap() {
            if !path.contains(&next) {
                path.push(next);
                let maxhapp = visit(graph, path, happ + edge);
                path.pop();
                max = max.max(maxhapp);
            }
        }
        max
    }

    let root = *graph.keys().next().unwrap();
    let part1 = visit(&graph, &mut vec![root], 0);

    // add myself
    let mut graph = graph;
    let mut me = HashMap::new();
    for (&person, edges) in graph.iter_mut() {
        me.insert(person, 0);
        edges.insert("me", 0);
    }
    graph.insert("me", me);

    let part2 = visit(&graph, &mut vec!["me"], 0);

    log(&format!("y15d13p1: {}", part1));
    log(&format!("y15d13p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d14(input: &str) {
    let mut reindeers = Vec::new();
    for line in input.lines() {
        let fields = line.split(" ").collect::<Vec<&str>>();
        let speed = fields[3].parse::<i32>().unwrap();
        let fly = fields[6].parse::<i32>().unwrap();
        let rest = fields[13].parse::<i32>().unwrap();
        reindeers.push((0, 0, true, 0, speed, fly, rest));
    }

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            let (mut dist, score, mut flying, mut time, speed, fly, rest) = reindeer;
            time += 1;
            if flying {
                dist += *speed;
                if time == *fly {
                    time = 0;
                    flying = false;
                }
            } else {
                if time == *rest {
                    time = 0;
                    flying = true;
                }
            }
            *reindeer = (dist, *score, flying, time, *speed, *fly, *rest);
        }

        let max = reindeers.iter().map(|x| x.0).max().unwrap();
        for reindeer in &mut reindeers {
            if reindeer.0 == max {
                reindeer.1 += 1;
            }
        }
    }
    let part1 = reindeers.iter().map(|x| x.0).max().unwrap();
    let part2 = reindeers.iter().map(|x| x.1).max().unwrap();

    log(&format!("y15d14p1: {}", part1));
    log(&format!("y15d14p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d15(input: &str) {
    let mut part2: i32 = 0;

    let n = input.lines().count();
    let mut ingredients = Array2::<i64>::zeros((5, n));
    for (i, line) in input.lines().enumerate() {
        let props = line.split(" ").collect::<Vec<&str>>();
        for (j, prop) in props.iter().enumerate().skip(2).step_by(2) {
            let value = prop.trim_end_matches(',').parse::<i64>().unwrap();
            ingredients[[j / 2 - 1, i]] = value;
        }
    }

    log(&format!("ing: {}", ingredients));
    let amounts = Array1::<i64>::from_vec(vec![44, 56]);
    log(&format!("amt: {}", amounts));
    log(&format!("score: {}", ingredients.dot(&amounts)));

    let part1: i64 = ingredients
        .dot(&amounts)
        .slice(s![..-1])
        .iter()
        .map(|&x| x.max(0))
        .product();

    log(&format!("y15d15p1: {}", part1));
    log(&format!("y15d15p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d16(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d16p1: {}", part1));
    log(&format!("y15d16p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d17(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d17p1: {}", part1));
    log(&format!("y15d17p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d18(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d18p1: {}", part1));
    log(&format!("y15d18p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d19(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d19p1: {}", part1));
    log(&format!("y15d19p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d20(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d20p1: {}", part1));
    log(&format!("y15d20p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d21(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d21p1: {}", part1));
    log(&format!("y15d21p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d22(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d22p1: {}", part1));
    log(&format!("y15d22p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d23(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d23p1: {}", part1));
    log(&format!("y15d23p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d24(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d24p1: {}", part1));
    log(&format!("y15d24p2: {}", part2));
}

#[wasm_bindgen]
pub fn y15d25(input: &str) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    log(&format!("y15d25p1: {}", part1));
    log(&format!("y15d25p2: {}", part2));
}
