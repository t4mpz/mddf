pub mod structures{
  use cli_table::{Table, format::Justify, WithTitle};
  pub struct ImagePage{
    pub src: String,
    pub title: String,
    pub alt: String
  }

  pub struct ListedChapter{
    pub href: String,
    pub title: String,
    pub position: i32,
    pub updated_raw_date: String
  }
  
  pub struct SearchResult{
    pub href: String,
    pub img_href: String,
    pub story_name: String,
    pub last_chapter: String,
    pub additional_info: Vec<String>
  }

  #[derive(Table)]
  pub struct SearchResultTable { 
    #[table(title="href", justify="Justify::Center")]
    pub href: String,
    #[table(title="Image", justify="Justify::Center")]
    pub img_href: String,
    #[table(title="Name", justify="Justify::Center")]
    pub story_name: String,
    #[table(title="Last Chapter", justify="Justify::Center")]
    pub last_chapter: String,
    #[table(title="Additional Info", justify="Justify::Center")]
    pub additional_info: String
  }
}