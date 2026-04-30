use std::collections::HashMap;

fn main() {
    // vectors
    let mut ppl: Vec<String> = Vec::new();
    ppl.push(String::from("Sibi"));
    ppl.push(String::from("Joshua"));
    ppl.push(String::from("Ned"));
    println!("ppl is {:?}", ppl);

    let last = ppl.pop();
    println!("last: {:?}", last);

    let third = ppl.get(2);
    println!("third: {:?}", third);

    let countdown = vec![5, 4, 3, 2, 1];

    // hash maps
    let mut classes = HashMap::new();
    classes.insert("trm", 2);
    classes.insert("qa", 3);
    classes.insert("css2", 2);
    classes.insert("graph", 1);
    println!("classes: {:?}", classes);

    classes.insert("trm", 3);
    classes.entry("trm").or_insert(2);

    let trm = classes.get("trm");
    println!("trm: {:?}", trm);

    let trm = classes.entry("trm").or_insert(0);
    *trm -= 1;

    let trm = classes.get("trm");
    println!("trm: {:?}", trm);
}
