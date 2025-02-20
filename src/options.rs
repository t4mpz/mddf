pub mod search_results{
  use crate::structures::structures::SearchResult;
  pub struct ResultDisplayOptions {
    pub href: bool,
    pub img_href: bool,
    pub story_name: bool,
    pub last_chapter: bool,
    pub additional_info: bool,
    pub counting: bool
  }

  pub fn gen_base_result_display_options() -> ResultDisplayOptions {
    ResultDisplayOptions{
      href: true,
      img_href: false,
      story_name: true,
      last_chapter: false,
      additional_info: false,
      counting: true
    }
  }

   /*
    Transforms a Search Result set into a string, retaining only the allowed fields
    It vectorizes the SearchResult data in a String vector and then compares
    with the vectorized ResultDisplayOptions to see what data will be retrieved
    from the said search result. The two vectors are zipped in order, allowing the
    user to ommit specific data, customizing the results
    :param result: Reference to the SearchResult item
    :param option: Reference to the ResultDisplayOptions item
    :returns: (String) The parsed string from the selected data from the search result
   */
  pub fn generate_result_string_by_options(result: &SearchResult, option: &ResultDisplayOptions) -> String {
    let mut summarized_data: Vec<String> = Vec::new();
    let vectorized_options: Vec<bool> = vec![option.href,
                                             option.img_href,
                                             option.story_name,
                                             option.last_chapter,
                                             option.additional_info];
    let vectorized_result: Vec<String> = vec![result.href.clone(),
                                              result.img_href.clone(),
                                              result.story_name.clone(),
                                              result.last_chapter.clone(),
                                              result.additional_info.join(" | ")];

    for i in 0..vectorized_options.len(){
      if vectorized_options[i] {
        summarized_data.push(vectorized_result[i].clone());
      }
    };
    return summarized_data.join("\t");
  }


}

pub mod download{
  pub struct DownloadOptions {
    pub story_href: String,
    pub chapters_to_download: i32, // 0 for all the chapters
    pub silent: bool
  }

  pub fn gen_download_options(story_href: &String, chapters_to_download: i32, silent: bool) -> DownloadOptions{
    DownloadOptions{
      story_href: story_href.clone(),
      chapters_to_download,
      silent
    }
  }
}