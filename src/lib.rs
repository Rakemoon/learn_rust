pub mod game {
    pub mod trivia;
}

pub mod util {
    pub fn escape_html(string: &str) -> String {
        string
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#039;", "'")
            .replace("&amp;", "&")
    }
}
