pub mod layouts {
  use handlebars::Handlebars;
  use serde::Serialize;
  use std::fs;

  #[derive(Serialize)]
  pub struct Contents {
    title: String,
    body: String,
  }

  impl Contents {
    pub fn new(title: String, body: String) -> Contents {
      Contents {
        title: title,
        body: body,
      }
    }
  }

  pub fn index(contents: &Contents) -> Result<String, String> {
    let mut handlebars = Handlebars::new();
    let index_raw: String = index_contents()?;
    register_template("index", &index_raw, &mut handlebars)?;
    match handlebars.render("index", contents) {
      Ok(contents) => Ok(String::from(contents)),
      Err(error) => Err(error.to_string()),
    }
  }

  fn register_template(
    name: &str,
    contents: &String,
    handlebars: &mut Handlebars,
  ) -> Result<String, String> {
    match handlebars.register_template_string(name, contents) {
      Ok(_) => Ok(String::from("Ok")),
      Err(error) => Err(error.to_string()),
    }
  }

  fn index_contents() -> Result<String, String> {
    match fs::read_to_string("./layout/index.html") {
      Ok(contents) => Ok(contents),
      Err(error) => Err(error.to_string()),
    }
  }
}
