pub mod structures{
  pub struct ImagePage{
    pub src: String,
    pub title: String,
    pub alt: String
  }

  #[derive(Clone)]
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
}