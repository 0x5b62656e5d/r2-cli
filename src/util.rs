pub async fn pretty_print(vec: Vec<String>) {
    vec.iter().for_each(|s| println!("{}", s.replace("\"", "")));
}
