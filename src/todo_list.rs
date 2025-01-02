use std::fmt;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}, {:?}", self.index, self.description, self.tags)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        Self::default()
    }

    pub fn push(&mut self, description: Description, tags: Vec<Tag>) -> TodoItem {
        // Start by logging the current state
        println!(
            "Before push: top_index={:?}, items={:?}",
            self.top_index, self.items
        );

        self.top_index = Index::new(self.top_index.0 + 1);
        let item = TodoItem::new(self.top_index, description, tags, false);
        self.items.push(item.clone());

        // Log the new state after adding the item
        println!(
            "After push: top_index={:?}, items={:?}",
            self.top_index, self.items
        );

        item
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        println!("Attempting to mark done: idx={:?}", idx);
        for item in &mut self.items {
            println!("Checking item: {:?}", item.index);
            if item.index == idx {
                item.done = true;
                println!("Marked done: {:?}", item);
                return Some(idx);
            }
        }
        println!("Index not found: {:?}", idx);
        None
    }

    pub fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        println!("Search params: {:?}", sp);
        let results: Vec<&TodoItem> = self
            .items
            .iter()
            .filter(|item| {
                println!("Checking item: {:?}", item);
                sp.words.iter().all(|word| {
                    item.description
                        .0
                        .to_lowercase()
                        .contains(&word.0.to_lowercase())
                }) && sp.tags.iter().all(|tag| {
                    item.tags
                        .iter()
                        .any(|item_tag| item_tag.0.to_lowercase() == tag.0.to_lowercase())
                })
            })
            .collect();
        println!("Search results: {:?}", results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut todo_list = TodoList::new();
        let desc = Description::new("Test Task");
        let tags = vec![Tag::new("urgent")];

        let item = todo_list.push(desc.clone(), tags.clone());
        assert_eq!(item.index, Index(1));
        assert_eq!(item.description, desc);
        assert_eq!(item.tags, tags);
        assert!(!item.done);
    }

    #[test]
    fn test_done_with_index() {
        let mut todo_list = TodoList::new();
        let desc = Description::new("Test Task");
        let tags = vec![Tag::new("urgent")];

        let item = todo_list.push(desc.clone(), tags.clone());
        assert_eq!(item.done, false);

        let done_index = todo_list.done_with_index(item.index);
        assert_eq!(done_index, Some(item.index));
        assert!(
            todo_list
                .items
                .iter()
                .find(|i| i.index == item.index)
                .unwrap()
                .done
        );
    }

    #[test]
    fn test_search() {
        let mut todo_list = TodoList::new();
        todo_list.push(
            Description::new("Buy groceries"),
            vec![Tag::new("shopping")],
        );
        todo_list.push(
            Description::new("Go to the mall"),
            vec![Tag::new("leisure")],
        );
        todo_list.push(
            Description::new("Send message to loved ones"),
            vec![Tag::new("communication")],
        );

        let search_params = SearchParams {
            words: vec![SearchWord::new("mall")],
            tags: vec![],
        };

        let results = todo_list.search(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].description.0, "Go to the mall");
    }

    #[test]
    fn test_search_with_tags() {
        let mut todo_list = TodoList::new();
        todo_list.push(
            Description::new("Buy groceries"),
            vec![Tag::new("shopping"), Tag::new("food")],
        );
        todo_list.push(
            Description::new("Go to the mall"),
            vec![Tag::new("leisure"), Tag::new("shopping")],
        );
        todo_list.push(
            Description::new("Send message to loved ones"),
            vec![Tag::new("communication")],
        );

        let search_params = SearchParams {
            words: vec![SearchWord::new("mall")],
            tags: vec![Tag::new("shopping")],
        };

        let results = todo_list.search(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].description.0, "Go to the mall");
    }

    #[test]
    fn test_done_with_invalid_index() {
        let mut todo_list = TodoList::new();
        let desc = Description::new("Test Task");
        let tags = vec![Tag::new("urgent")];

        todo_list.push(desc.clone(), tags.clone());

        let invalid_index = Index(99);
        let done_result = todo_list.done_with_index(invalid_index);
        assert_eq!(done_result, None);
    }
}
