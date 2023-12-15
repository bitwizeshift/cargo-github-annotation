use std::io::BufRead;

use serde_json as json;

fn json_string(value: Option<&json::Value>) -> Option<String> {
  value.and_then(|v| v.as_str().map(String::from))
}

fn json_u64(value: Option<&json::Value>) -> Option<u64> {
  value.and_then(|v| v.as_u64())
}

fn print_message(value: &json::Value) {
  if let Some(children) = value.get("children") {
    if let Some(array) = children.as_array() {
      for value in array {
        print_message(value)
      }
    }
  }

  let level = json_string(value.get("level"));
  let message = json_string(value.get("message"));

  if level.is_none() || message.is_none() {
    return;
  }

  let level = match level.unwrap().as_str() {
    "error" => "error",
    "help" => "notice",
    "warning" => "warning",
    _ => return,
  };
  let message = message.unwrap();

  if let Some(spans) = value.get("spans") {
    if let Some(array) = spans.as_array() {
      for span in array {
        let mut output = String::new();

        output.push_str(&format!("::{} ", level));
        if let Some(file_name) = json_string(span.get("file_name")) {
          output.push_str(&format!("file={}", &file_name));
        }
        if let Some(col_start) = json_u64(span.get("column_start")) {
          output.push_str(&format!(",col={}", col_start));
        }
        if let Some(col_end) = json_u64(span.get("column_end")) {
          output.push_str(&format!(",endColumn={}", col_end));
        }
        if let Some(line_start) = json_u64(span.get("line_start")) {
          output.push_str(&format!(",line={}", line_start));
        }
        if let Some(line_end) = json_u64(span.get("line_end")) {
          output.push_str(&format!(",endLine={}", line_end));
        }
        output.push_str("::");
        output.push_str(&message);

        eprintln!("{}", output);
      }
    }
  }
}

fn main() {
  for maybe_line in std::io::stdin().lock().lines() {
    if maybe_line.is_err() {
      continue;
    }
    let line = maybe_line.unwrap();
    let maybe_object = json::from_str(&line);
    if maybe_object.is_err() {
      continue;
    }
    let object: json::Value = maybe_object.unwrap();
    match object {
      json::Value::Object(map) => {
        let maybe_message = map.get("message");
        if maybe_message.is_none() {
          continue;
        }
        let message = maybe_message.unwrap();

        print_message(message);
      }
      _ => continue,
    }
  }
}
