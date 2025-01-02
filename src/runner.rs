use crate::*;

pub fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl) {
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        // Implementation for the Add query
        Query::Add(desc, tags) => {
            let item = tl.push(desc, tags); // Add the item to the TodoList
            Ok(QueryResult::Added(item)) // Return the result
        }
        // Implementation for the Done query
        Query::Done(idx) => {
            if tl.done_with_index(idx).is_some() {
                Ok(QueryResult::Done) // Mark item as done and return success
            } else {
                Err(QueryError(format!("No item found with index {}", idx))) // Error if index not found
            }
        }
        // Implementation for the Search query
        Query::Search(params) => {
            let results = tl.search(params); // Perform the search
            let items = results.into_iter().cloned().collect(); // Collect results
            Ok(QueryResult::Found(items)) // Return the found items
        }
    }
}
