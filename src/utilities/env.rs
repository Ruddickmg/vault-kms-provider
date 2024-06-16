pub fn get_env(var_name: &str, default: &str) -> String {
  std::env::var(var_name).unwrap_or_else(|_| {
    println!("Environment variable: \"{}\" has not been defined, defaulting to: {}", var_name, default);
    default.to_string()
  })
}
