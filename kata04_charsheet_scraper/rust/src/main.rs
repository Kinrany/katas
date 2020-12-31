use headless_chrome::Browser;

fn browse_wikipedia() -> Result<(), failure::Error> {
  let browser = Browser::default()?;

  let tab = browser.wait_for_initial_tab()?;

  // Navigate to dndbeyond
  tab.navigate_to("https://www.dndbeyond.com/characters/31859887/HUQLxj")?;

  // Wait for network/javascript/dom to make the skill list available
  let element = tab.wait_for_element("div.ct-skills")?;
  let outer_html = element
    .call_js_fn("function() { return this.outerHTML; }", true)?
    .value
    .unwrap_or_default()
    .to_string();

  println!("{}", outer_html);

  // // Type in a query and press `Enter`
  // tab.type_str("WebKit")?.press_key("Enter")?;

  // // We should end up on the WebKit-page once navigated
  // tab.wait_for_element("#firstHeading")?;
  // assert!(tab.get_url().ends_with("WebKit"));

  // // Take a screenshot of the entire browser window
  // let _jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;

  // // Take a screenshot of just the WebKit-Infobox
  // let _png_data = tab
  //   .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
  //   .capture_screenshot(ScreenshotFormat::PNG)?;

  Ok(())
}

fn main() -> Result<(), failure::Error> {
  browse_wikipedia()
}
