use regex::Regex;

fn main() {
    let text = "hel o . o o . oo o";
    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(text, "%20");
    println!("input: {}", text);
    println!("result: {}", result);
}
