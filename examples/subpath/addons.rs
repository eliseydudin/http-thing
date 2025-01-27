use std::collections::HashMap;

pub fn parse_query(query: &String) -> HashMap<String, String> {
    let mut parsed_query: HashMap<String, String> = HashMap::new();
    if query.len() < 2 {
        return parsed_query;
    }

    query[1..]
        .split('&')
        .filter(|&c| c != "")
        .for_each(|section| {
            let parts: Vec<&str> = section.split('=').collect::<Vec<&str>>();
            if parts.len() > 1 {
                parsed_query.insert(parts[0].to_string(), parts[1].to_string());
            } else {
                parsed_query.insert(parts[0].to_string(), "".to_string());
            }
        });
    return parsed_query;
}

pub fn query_string_to_string(query_string: &String) -> String {
    return query_string.replace("%20", " ").replace("+", " ");
}