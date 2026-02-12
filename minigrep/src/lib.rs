pub fn search<'a>(query: &'a str, contents: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(contents.lines().filter(move |line| line.contains(&query)))
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    let query = query.to_lowercase();
    Box::new(
        contents
            .lines()
            .filter(move |line| line.to_lowercase().contains(&query)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<&str>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents).collect::<Vec<&str>>()
        );
    }
}
