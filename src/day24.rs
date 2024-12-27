use itertools::Itertools;
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

    let x_value = get_decimal_wire_value("x", &wire_map);
    let y_value = get_decimal_wire_value("y", &wire_map);
    let z_value = get_decimal_wire_value("z", &wire_map);
    println!(
        "x: {}, y: {}, expected z: {}, real z: {}",
        x_value,
        y_value,
        x_value + y_value,
        z_value
    );

    println!(
        "x: {:b}, y: {:b}, expected z: {:b}, real z: {:b}",
        x_value,
        y_value,
        x_value + y_value,
        z_value
    );

    let (_, part2) = solve();
    println!("Part 2: {}", part2);

    z_value
}

fn get_decimal_wire_value(prefix: &str, wire_map: &HashMap<&str, WireValue>) -> u64 {
    let mut filtered_wires: Vec<_> = wire_map
        .keys()
        .filter(|key| key.starts_with(prefix))
        .collect();
    // sort descending
    filtered_wires.sort_by(|a, b| b.cmp(a));
    let mut value = 0;
    for wire in filtered_wires {
        value *= 2;
        value += evaluate_wire_value(wire_map, wire) as u64;
    }
    value
}

fn get_input_wires_values() -> Vec<WireValue> {
    //let input = include_str!("../input/day24_literals.txt");
    let input = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1";
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
    //let input = include_str!("../input/day24_operations.txt");
    let input = r"ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
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

use std::fmt::Debug;

#[derive(Eq, PartialEq)]
enum OperationType {
    Xor,
    Or,
    And,
}

trait Operation: Debug {
    fn op(&self, values: &mut std::collections::HashMap<&'static str, u8>) -> bool;
    fn get_inputs(&self) -> (&'static str, &'static str);
    fn output(&self) -> &'static str;
    fn get_type(&self) -> OperationType;
}

#[derive(Debug, Clone)]
struct And {
    inp1: &'static str,
    inp2: &'static str,
    out: &'static str,
}

impl Operation for And {
    fn op(&self, values: &mut std::collections::HashMap<&'static str, u8>) -> bool {
        if (*values.get(self.inp1).unwrap() == u8::MAX)
            || (*values.get(self.inp2).unwrap() == u8::MAX)
        {
            return false;
        }

        *values.get_mut(self.out).unwrap() =
            values.get(self.inp1).unwrap() & values.get(self.inp2).unwrap();
        true
    }

    fn get_inputs(&self) -> (&'static str, &'static str) {
        (
            std::cmp::min(self.inp1, self.inp2),
            std::cmp::max(self.inp1, self.inp2),
        )
    }

    fn output(&self) -> &'static str {
        self.out
    }

    fn get_type(&self) -> OperationType {
        OperationType::And
    }
}

#[derive(Debug, Clone)]
struct Xor {
    inp1: &'static str,
    inp2: &'static str,
    out: &'static str,
}

impl Operation for Xor {
    fn op(&self, values: &mut std::collections::HashMap<&'static str, u8>) -> bool {
        if (*values.get(self.inp1).unwrap() == u8::MAX)
            || (*values.get(self.inp2).unwrap() == u8::MAX)
        {
            return false;
        }

        *values.get_mut(self.out).unwrap() =
            values.get(self.inp1).unwrap() ^ values.get(self.inp2).unwrap();
        true
    }
    fn get_inputs(&self) -> (&'static str, &'static str) {
        (
            std::cmp::min(self.inp1, self.inp2),
            std::cmp::max(self.inp1, self.inp2),
        )
    }

    fn output(&self) -> &'static str {
        self.out
    }

    fn get_type(&self) -> OperationType {
        OperationType::Xor
    }
}

#[derive(Debug, Clone)]
struct Or {
    inp1: &'static str,
    inp2: &'static str,
    out: &'static str,
}

impl Operation for Or {
    fn op(&self, values: &mut std::collections::HashMap<&'static str, u8>) -> bool {
        if (*values.get(self.inp1).unwrap() == u8::MAX)
            || (*values.get(self.inp2).unwrap() == u8::MAX)
        {
            return false;
        }

        *values.get_mut(self.out).unwrap() =
            values.get(self.inp1).unwrap() | values.get(self.inp2).unwrap();
        true
    }

    fn get_inputs(&self) -> (&'static str, &'static str) {
        (
            std::cmp::min(self.inp1, self.inp2),
            std::cmp::max(self.inp1, self.inp2),
        )
    }

    fn output(&self) -> &'static str {
        self.out
    }
    fn get_type(&self) -> OperationType {
        OperationType::Or
    }
}

fn create_operation(
    inp1: &'static str,
    inp2: &'static str,
    op: &'static str,
    out: &'static str,
) -> Box<dyn Operation> {
    match op {
        "AND" => Box::new(And { inp1, inp2, out }),
        "OR" => Box::new(Or { inp1, inp2, out }),
        _ => Box::new(Xor { inp1, inp2, out }),
    }
}

pub fn solve() -> (usize, String) {
    // General setup
    let input = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
x05: 0
x06: 1
x07: 1
x08: 0
x09: 1
x10: 1
x11: 1
x12: 1
x13: 1
x14: 0
x15: 1
x16: 0
x17: 1
x18: 0
x19: 1
x20: 1
x21: 0
x22: 0
x23: 0
x24: 1
x25: 1
x26: 1
x27: 0
x28: 1
x29: 1
x30: 1
x31: 0
x32: 0
x33: 1
x34: 1
x35: 0
x36: 0
x37: 0
x38: 1
x39: 0
x40: 0
x41: 0
x42: 0
x43: 1
x44: 1
y00: 1
y01: 0
y02: 0
y03: 1
y04: 1
y05: 0
y06: 0
y07: 0
y08: 0
y09: 0
y10: 0
y11: 1
y12: 0
y13: 0
y14: 0
y15: 0
y16: 1
y17: 0
y18: 0
y19: 1
y20: 0
y21: 1
y22: 0
y23: 1
y24: 0
y25: 0
y26: 1
y27: 0
y28: 0
y29: 1
y30: 0
y31: 0
y32: 0
y33: 1
y34: 0
y35: 0
y36: 0
y37: 1
y38: 0
y39: 0
y40: 1
y41: 0
y42: 1
y43: 1
y44: 1

x44 XOR y44 -> drc
phq OR frm -> hjs
vdh AND nwn -> gqd
y40 XOR x40 -> vkn
x21 XOR y21 -> cnb
cnb AND wmb -> vvk
dfb XOR bfn -> hbk
bhd XOR mmh -> z06
fkc XOR bwj -> z22
gnj AND jfw -> spq
dwh AND kqn -> fhp
x19 AND y19 -> kkg
drc XOR qqt -> z44
fkc AND bwj -> jbb
y32 XOR x32 -> qwt
y37 AND x37 -> kgg
x07 AND y07 -> dqn
dsp AND bvp -> hff
pmv OR pkn -> wmt
cjf OR pfk -> z45
hgq XOR phb -> z27
qnq OR dpc -> djp
x32 AND y32 -> nbb
qwt AND jqm -> fdk
x18 XOR y18 -> grp
vkf OR hdm -> kqn
cqv AND jss -> bwd
x00 AND y00 -> jfw
cjb XOR srm -> z19
jss XOR cqv -> z35
ntt OR spq -> ndd
cqm XOR qqj -> z43
x01 AND y01 -> ntt
y14 XOR x14 -> dfb
nbk XOR wrk -> z05
dvw AND rpg -> z23
vvc OR kcv -> qqj
bqc XOR fwr -> z26
dwh XOR kqn -> z41
x15 XOR y15 -> bkb
rjm XOR gjr -> z24
x22 XOR y22 -> bwj
y22 AND x22 -> hpj
x08 XOR y08 -> hnf
y27 AND x27 -> frm
wrw OR swr -> fds
gtm AND rmt -> mkv
kdh AND qvq -> ghr
fgv AND mfm -> kcv
hnf AND gqs -> pmv
kkg OR qvs -> vdh
fdk OR nbb -> rmt
y29 AND x29 -> cwd
hjk OR bts -> vkg
vtk AND npm -> tqb
dvw XOR rpg -> dbb
y39 XOR x39 -> mnm
y05 XOR x05 -> wrk
djd AND fds -> dqt
tvh OR sqm -> npm
cdr XOR cmt -> z10
x28 XOR y28 -> hgd
x33 XOR y33 -> gtm
mnm XOR vhv -> z39
fbv XOR bwg -> z38
hqs AND nhr -> vbt
kth OR qcp -> hgq
wjj OR scs -> bfn
bkv OR vvk -> fkc
cmt AND cdr -> pph
dqt OR gqb -> bqc
y35 XOR x35 -> jss
vkb OR krd -> cdr
mqf AND cvh -> trj
x36 XOR y36 -> dsp
y41 XOR x41 -> dwh
y38 XOR x38 -> bwg
ghr OR tpc -> jqm
ckn XOR hqq -> z07
vkn AND vkg -> vkf
y28 AND x28 -> wkc
x31 XOR y31 -> qvq
rjm AND gjr -> swr
gcb OR dbb -> rjm
y18 AND x18 -> z18
y24 AND x24 -> wrw
x17 XOR y17 -> kbh
y24 XOR x24 -> gjr
x26 XOR y26 -> fwr
y43 AND x43 -> bvm
y15 AND x15 -> sbt
y12 XOR x12 -> fvh
kvn OR ffb -> cjb
y31 AND x31 -> tpc
y37 XOR x37 -> bnh
y11 AND x11 -> scv
hgd AND hjs -> phr
jfw XOR gnj -> z01
fvh XOR sfk -> z12
fds XOR djd -> z25
qwt XOR jqm -> z32
bvp XOR dsp -> z36
phr OR wkc -> jdq
y07 XOR x07 -> hqq
y43 XOR x43 -> cqm
bnh XOR kss -> z37
trg OR vbt -> kdh
cwd OR pkc -> nhr
y19 XOR x19 -> srm
dkd AND jdq -> pkc
wrk AND nbk -> fnk
hjs XOR hgd -> z28
rnt AND qbs -> rcp
djp XOR mft -> z03
cht OR mkv -> mqf
hbk XOR bkb -> z15
x44 AND y44 -> pfk
x26 AND y26 -> qcp
ndd XOR jgw -> z02
x06 AND y06 -> dhs
ckn AND hqq -> cpt
y13 AND x13 -> wjj
x10 AND y10 -> tvr
ffr OR gqd -> wmb
y09 XOR x09 -> tjb
nhr XOR hqs -> z30
hgq AND phb -> phq
x00 XOR y00 -> z00
y16 AND x16 -> prt
dqn OR cpt -> gqs
x23 AND y23 -> gcb
mft AND djp -> tvh
bkb AND hbk -> qtw
kfk AND chk -> vgb
vhv AND mnm -> hjk
y42 XOR x42 -> mfm
x25 XOR y25 -> djd
fgv XOR mfm -> z42
grp XOR fgr -> kvn
x16 XOR y16 -> chk
x09 AND y09 -> krd
cqb OR rmg -> mqm
y30 XOR x30 -> hqs
kss AND bnh -> dvf
y11 XOR x11 -> rnt
x04 AND y04 -> ptm
y13 XOR x13 -> cbr
rnt XOR qbs -> z11
hff OR fjf -> kss
x25 AND y25 -> gqb
kdh XOR qvq -> z31
y06 XOR x06 -> mmh
cnb XOR wmb -> z21
y02 XOR x02 -> jgw
x17 AND y17 -> hkn
y34 AND x34 -> cvh
x27 XOR y27 -> phb
x42 AND y42 -> vvc
x03 XOR y03 -> mft
y35 AND x35 -> nvg
x10 XOR y10 -> cmt
y20 AND x20 -> ffr
x30 AND y30 -> trg
ptm OR tqb -> nbk
bfn AND dfb -> sjr
jgw AND ndd -> qnq
y39 AND x39 -> bts
y38 AND x38 -> knq
npm XOR vtk -> z04
prt OR vgb -> jqc
kfk XOR chk -> z16
rvd OR dhs -> ckn
fnk OR wkv -> bhd
y03 AND x03 -> sqm
x08 AND y08 -> pkn
y02 AND x02 -> dpc
bwg AND fbv -> pdw
mqf XOR cvh -> z34
tjb AND wmt -> vkb
jdq XOR dkd -> z29
x34 XOR y34 -> tfn
y21 AND x21 -> bkv
tfn OR trj -> cqv
fgr AND grp -> ffb
bwd OR nvg -> bvp
cjv OR hkn -> fgr
fhp OR rnc -> fgv
sjr OR tck -> z14
vkn XOR vkg -> z40
kbh XOR jqc -> z17
kgg OR dvf -> fbv
tjb XOR wmt -> z09
tvr OR pph -> qbs
qqj AND cqm -> gsg
pdw OR knq -> vhv
gqs XOR hnf -> z08
y20 XOR x20 -> nwn
kbh AND jqc -> cjv
bqc AND fwr -> kth
rcp OR scv -> sfk
sbt OR qtw -> kfk
bhd AND mmh -> rvd
nwn XOR vdh -> z20
y29 XOR x29 -> dkd
y23 XOR x23 -> rpg
jbb OR hpj -> dvw
cbr XOR mqm -> z13
y33 AND x33 -> cht
y12 AND x12 -> rmg
mqm AND cbr -> scs
sfk AND fvh -> cqb
y14 AND x14 -> tck
x04 XOR y04 -> vtk
y05 AND x05 -> wkv
rmt XOR gtm -> z33
y01 XOR x01 -> gnj
srm AND cjb -> qvs
x36 AND y36 -> fjf
drc AND qqt -> cjf
y40 AND x40 -> hdm
y41 AND x41 -> rnc
gsg OR bvm -> qqt";
    let mut parts = input.split("\n\n");
    let initial = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split(": ");
        (
            parts.next().unwrap(),
            parts.next().unwrap().parse::<u8>().unwrap(),
        )
    });

    let mut gates = std::collections::HashMap::<&'static str, u8>::new();
    let mut connections_reversed =
        std::collections::HashMap::<&'static str, (&'static str, &'static str)>::new();

    for gate in initial {
        gates.insert(gate.0, gate.1);
    }

    let operations = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let inp1 = parts.next().unwrap();
            let op = parts.next().unwrap();
            let inp2 = parts.next().unwrap();
            let out = parts.nth(1).unwrap();
            connections_reversed.insert(out, (inp1, inp2));
            [inp1, inp2, out].iter().for_each(|gate| {
                if !gates.contains_key(gate) {
                    gates.insert(gate, u8::MAX);
                }
            });
            create_operation(inp1, inp2, op, out)
        })
        .collect::<Vec<_>>();

    // Part 1 Solution, just simulate all the gates until we're finished
    let mut still_to_produce = operations.iter().collect::<Vec<_>>();

    while !still_to_produce.is_empty() {
        still_to_produce = still_to_produce
            .iter()
            .filter(|op| !op.op(&mut gates))
            .copied()
            .collect::<Vec<_>>();
    }

    // get all binary values And convert to a number
    let mut output_gates = gates
        .iter()
        .filter(|(key, _val)| key.starts_with("z"))
        .collect::<Vec<_>>();
    output_gates.sort();
    output_gates.reverse();

    let output = output_gates
        .iter()
        .map(|(_name, val)| val.to_string())
        .collect::<String>();
    let sol1 = usize::from_str_radix(&output, 2).unwrap();

    // Part2, also based on the insights from : https://www.reddit.com/r/adventofcode/comments/1hla5ql/2024_day_24_part_2_a_guide_on_the_idea_behind_the/
    let mut faulty = vec![];
    for operation in operations.iter() {
        // Find all nodes feeding an output that are not Xor (only exception would be z45)
        if operation.output().starts_with("z")
            && operation.get_type() != OperationType::Xor
            && operation.output() != "z45"
        {
            let inputs = operation.get_inputs();

            // ignore gates fed by inputs directly
            if inputs.0.starts_with("x") || inputs.0.starts_with("y") {
                continue;
            }
            faulty.push(operation.output());
        }

        // Find all nodes that could only feed outputs, but are feeding intermediate gates instead
        if operation.get_type() == OperationType::Xor && !operation.output().starts_with("z") {
            let inputs = operation.get_inputs();
            // again ignore gates fed by inputs directly
            if inputs.0.starts_with("x") || inputs.0.starts_with("y") {
                continue;
            }
            faulty.push(operation.output());
        }
    }

    // now check for special cases
    // First: Every Xor not fed by X00/Y00 needs to feed into another Xor
    // by design of Carry Ripple Adders
    // If there is none, we have a misconfiguration
    for operation in operations.iter() {
        if operation.get_type() == OperationType::Xor
            && (operation.get_inputs().0.starts_with("x")
                || operation.get_inputs().0.starts_with("y"))
            && !operation.get_inputs().0[1..].starts_with("00")
        {
            if !operations.iter().any(|op| {
                op.get_type() == OperationType::Xor
                    && (op.get_inputs().0 == operation.output()
                        || op.get_inputs().1 == operation.output())
            }) {
                faulty.push(operation.output());
            }
        }
        // Next find ANDs that do not feed into ORs
        // again, must be true for Ripple Carry adders
        else if operation.get_type() == OperationType::And
            && (operation.get_inputs().0.starts_with("x")
                || operation.get_inputs().0.starts_with("y"))
            && !(operation.get_inputs().0.starts_with("x00"))
            && !operations.iter().any(|op| {
                op.get_type() == OperationType::Or
                    && (op.get_inputs().0 == operation.output()
                        || op.get_inputs().1 == operation.output())
            })
        {
            faulty.push(operation.output());
        }
    }
    faulty.sort();

    let sol2 = faulty.iter().join(",");

    (sol1, sol2) //.iter().join(",")))
}
