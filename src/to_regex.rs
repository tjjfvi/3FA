use crate::*;

pub fn to_regex<'a, D: Dfa<&'a u8>>(
  dfa: D,
  alphabet: impl Clone + IntoIterator<Item = &'a u8>,
) -> String
where
  D::State: Ord + Clone,
{
  let mut paths = [(
    None,
    [(Some(dfa.initial()), String::new())].into_iter().collect(),
  )]
  .into_iter()
  .collect();

  visit(&dfa, alphabet, &mut paths, dfa.initial());

  while let Some(state) = paths.keys().find_map(|x| x.clone()) {
    let key = Some(state);
    let mut map = paths.remove(&key).unwrap();
    let self_path = map.remove(&key);
    for map2 in &mut paths.values_mut() {
      if let Some(initial_path) = map2.remove(&key) {
        for (key, end_path) in &map {
          let path = format!(
            "({}){}({})",
            &initial_path,
            self_path
              .as_ref()
              .map(|s| format!("({})*", s))
              .unwrap_or_default(),
            end_path
          );
          map2
            .entry(key.clone())
            .and_modify(|x| {
              *x += "|";
              *x += &path;
            })
            .or_insert(path);
        }
      }
    }
  }

  return paths.remove(&None).unwrap().remove(&None).unwrap();

  fn visit<'a, D: Dfa<&'a u8>>(
    dfa: &D,
    alphabet: impl Clone + IntoIterator<Item = &'a u8>,
    paths: &mut BTreeMap<Option<D::State>, BTreeMap<Option<D::State>, String>>,
    state: D::State,
  ) where
    D::State: Ord + Clone,
  {
    let mut todo = BTreeSet::new();
    paths.entry(Some(state.clone())).or_insert_with(|| {
      let mut map = BTreeMap::new();
      if dfa.accept(&state) {
        map.insert(None, "".to_owned());
      }
      for char in alphabet.clone() {
        if let Some(next) = dfa.next(state.clone(), char) {
          todo.insert(next.clone());
          let str = if (*char as char).is_ascii() {
            (*char as char).to_string()
          } else {
            format!("\\{:02x}", *char)
          };
          map
            .entry(Some(next))
            .and_modify(|o| {
              *o += "|";
              *o += &str;
            })
            .or_insert(str);
        }
      }
      map
    });
    for state in todo {
      visit(dfa, alphabet.clone(), paths, state)
    }
  }
}
