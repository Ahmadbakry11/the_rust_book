use crate::traits::Summary;

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {

  fn state(&self) -> bool {
      self.retweet || self.reply
  }

  fn summarize_author(&self) -> &String {
      &self.username
  }
  
  fn summarize(&self) -> String {
    format!("Tweet Summary: {}: {}", self.username, self.content)
  }
}
