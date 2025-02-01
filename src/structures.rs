pub mod structures{
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

  pub struct ResultDisplayOptions {
    pub href: bool,
    pub img_href: bool,
    pub story_name: bool,
    pub last_chapter: bool,
    pub additional_info: bool,
    pub counting: bool
  }
}