use super::scraper::SelectorData;

pub fn get_data() -> SelectorData {
    SelectorData {
        index_selector: "ul.chapter-list > a.chapter-item".to_string(),
        title_selector: "div.book-name".to_string(),
        status_selector: "div.book-state".to_string(),
        author_selector: "div.author".to_string(),
        text_selector: "div.chapter-entity".to_string(),
    }
}
