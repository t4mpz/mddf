pub mod screens{
  use crate::argscommands::argscommands;
  use indicatif::ProgressBar;
  use crate::structures::{structures::SearchResult, structures::ResultDisplayOptions};
  
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


  /*
    Indexes the query search results and returns it as a text to be 
    implemented in a std::io::out
    it shows the results data accordinly with the display options.
    :param results: The Search Results in a vector
    :param options: The display options for said results
    :returns: The parsed String for all of those results
  */
  pub fn result_renderer(results: Vec<SearchResult>, options: &ResultDisplayOptions) -> String{
    let parsed_string_results: Vec<String> = results.iter().enumerate().map(| key_result | {
      let key = key_result.0;
      let result = key_result.1;
      let result_str = generate_result_string_by_options(&result, options);
      format!("[{}] {}", key, result_str).to_string()
    }).collect();
    return parsed_string_results.join("\n");
  }

  /*
    It generates a new progress bar to be used and returns it so the using function can
    iterate it. 
    :returns: The Progress Bar 
   */
  // pub fn pg_while() -> ProgressBar {
    // TODO
  // }
}