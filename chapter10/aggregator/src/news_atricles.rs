use crate::traits::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
    pub published: bool,
}

impl Summary for NewsArticle {
  fn state(&self) -> bool {
      self.published
  }
  
  fn summarize_author(&self) -> &String {
      &self.author
  }
}