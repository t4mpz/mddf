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
}