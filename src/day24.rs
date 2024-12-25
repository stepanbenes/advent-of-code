use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Or,
    And,
    Xor,
}

#[derive(Debug)]
enum WireValue {
    Literal {
        name: &'static str,
        value: bool,
    },
    Operation {
        name: &'static str,
        operator: Operator,
        left: &'static str,
        right: &'static str,
    },
}

pub fn decimal_output_on_wires() -> u64 {
    let input_wires = get_input_wires_values();
    let wire_operations = get_wires_operations();
    let all_wires = input_wires.iter().chain(wire_operations.iter());
    println!("{:?}", all_wires);
    let mut wire_map: HashMap<&str, WireValue> = HashMap::new();
    for wire in all_wires {
        match wire {
            WireValue::Literal { name, value } => {
                wire_map.insert(
                    name,
                    WireValue::Literal {
                        name,
                        value: *value,
                    },
                );
            }
            WireValue::Operation {
                name,
                operator,
                left,
                right,
            } => {
                wire_map.insert(
                    name,
                    WireValue::Operation {
                        name,
                        operator: *operator,
                        left,
                        right,
                    },
                );
            }
        }
    }
    let mut z_wires: Vec<_> = wire_map.keys().filter(|key| key.starts_with('z')).collect();
    // sort descending
    z_wires.sort_by(|a, b| b.cmp(a));
    let mut z_value = 0;
    for wire in z_wires {
        z_value *= 2;
        let value = evaluate_wire_value(&wire_map, wire);
        z_value += value as u64;
        println!("{}: {}", wire, value);
    }
    z_value
}

fn get_input_wires_values() -> Vec<WireValue> {
    let input = include_str!("../input/day24_literals.txt");
    //     let input = r"x00: 1
    // x01: 0
    // x02: 1
    // x03: 1
    // x04: 0
    // y00: 1
    // y01: 1
    // y02: 1
    // y03: 1
    // y04: 1";
    let mut wires = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].trim_end_matches(':');
        let value = parts[1].parse::<u64>().unwrap();
        let wire = WireValue::Literal {
            name,
            value: value == 1,
        };
        wires.push(wire);
    }
    wires
}

fn get_wires_operations() -> Vec<WireValue> {
    let input = include_str!("../input/day24_operations.txt");
    //     let input = r"ntg XOR fgs -> mjb
    // y02 OR x01 -> tnw
    // kwq OR kpj -> z05
    // x00 OR x03 -> fst
    // tgd XOR rvg -> z01
    // vdt OR tnw -> bfw
    // bfw AND frj -> z10
    // ffh OR nrd -> bqk
    // y00 AND y03 -> djm
    // y03 OR y00 -> psh
    // bqk OR frj -> z08
    // tnw OR fst -> frj
    // gnj AND tgd -> z11
    // bfw XOR mjb -> z00
    // x03 OR x00 -> vdt
    // gnj AND wpb -> z02
    // x04 AND y00 -> kjc
    // djm OR pbm -> qhw
    // nrd AND vdt -> hwm
    // kjc AND fst -> rvg
    // y04 OR y02 -> fgs
    // y01 AND x02 -> pbm
    // ntg OR kjc -> kwq
    // psh XOR fgs -> tgd
    // qhw XOR tgd -> z09
    // pbm OR djm -> kpj
    // x03 XOR y03 -> ffh
    // x00 XOR y04 -> ntg
    // bfw OR bqk -> z06
    // nrd XOR fgs -> wpb
    // frj XOR qhw -> z04
    // bqk OR frj -> z07
    // y03 OR x01 -> nrd
    // hwm AND bqk -> z03
    // tgd XOR rvg -> z12
    // tnw OR pbm -> gnj";
    let mut wires = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[parts.len() - 1];
        let operator = match parts[1] {
            "OR" => Operator::Or,
            "AND" => Operator::And,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operator"),
        };
        let left = parts[0];
        let right = parts[2];
        let wire = WireValue::Operation {
            name,
            operator,
            left,
            right,
        };
        wires.push(wire);
    }
    wires
}

fn evaluate_wire_value(wire_map: &HashMap<&str, WireValue>, wire_name: &str) -> u8 {
    match wire_map.get(wire_name).unwrap() {
        WireValue::Literal { name: _, value } => {
            if *value {
                1
            } else {
                0
            }
        }
        WireValue::Operation {
            name: _,
            operator,
            left,
            right,
        } => {
            let left_value = evaluate_wire_value(wire_map, left);
            let right_value = evaluate_wire_value(wire_map, right);
            match operator {
                Operator::Or => left_value | right_value,
                Operator::And => left_value & right_value,
                Operator::Xor => left_value ^ right_value,
            }
        }
    }
}
