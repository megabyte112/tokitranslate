fn main(){
    if std::env::args().len() == 1 {
        println!("toki pona translator");
        println!("type 'q' or 'quit' to exit\n");
        loop {
            println!("write sentence:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("readline failed");
            println!();
            let input = input.trim().to_lowercase();
            if input == "q" || input == "quit" {
                break;
            }
            let words: Vec<&str> = input.split_whitespace().collect();
            let words: Vec<String> = words.iter().map(|&x| x.to_string()).collect();
            output(&words);
            println!();
        }
    }
    else {
        let mut args = std::env::args();
        args.next();
        let words: Vec<String> = args.collect();
        output(&words);
    }
}
fn translateword(word: &str) -> &str {
    let word = word.trim_matches(|c: char| !c.is_alphabetic());
    let wordlist = include_str!("wordlist");
    let wordlist = wordlist.split("\n");
    let wordlist = wordlist.map(|line| line.split("-").collect::<Vec<&str>>());
    let wordlist = wordlist.map(|line| (line[0], line[1])).collect::<std::collections::HashMap<&str, &str>>();
    wordlist.get(&*word).unwrap_or(&"<no word found>")
}
fn output(args: &Vec<String>) {
    for word in args {
        let mut word = word.to_string();
        while word.len() < 12 {
            word.push(' ');
        }
        let word = word.as_str();
        println!("{}:\t{}", word, translateword(word));
    }
}