pub mod screens{
  use crate::structures::structures::SearchResult;
  use crate::options::search_results::{ResultDisplayOptions, generate_result_string_by_options};


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
   Renders an image at the terminal using the kitty icat image kitten
   for more info on that https://sw.kovidgoyal.net/kitty/kittens/icat/
   it renders the kitten image using pre-defined image configurations
   using the raw command curl -SsL <img href> | kitten icat
   */
  pub fn img_renderer(url_img: &String){

  }

}