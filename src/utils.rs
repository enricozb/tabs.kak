use std::collections::HashMap;

/// Returns a minified buflist.
///
/// Specifically, returns the vector of smallest unique suffixes of buflist.
pub fn minified_buflist(buflist: &[String]) -> Vec<String> {
  let mut paths = HashMap::<String, (Vec<&str>, usize)>::new();
  let mut minified = Vec::new();

  for bufname in buflist {
    let mut parts = bufname.split('/').collect::<Vec<_>>();
    let mut candidate = parts.pop().unwrap().to_string();

    if let Some((mut other_parts, index)) = paths.remove(&candidate) {
      let mut other_candidate = candidate.clone();
      while candidate == other_candidate {
        assert!(!parts.is_empty() || !other_parts.is_empty(), "identical buffers");

        if let Some(parent) = parts.pop() {
          candidate = [parent, &candidate].join("/");
        }
        if let Some(parent) = other_parts.pop() {
          other_candidate = [parent, &other_candidate].join("/");
        }
      }

      // replace previously conflicting candidate path
      minified[index] = other_candidate.clone();

      paths.insert(other_candidate, (other_parts, index));
    } else {
    }

    paths.insert(candidate.clone(), (parts, minified.len()));
    minified.push(candidate);
  }

  minified
}
