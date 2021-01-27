use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::{From, TryFrom};

use thiserror::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub id: Uuid,
    pub name: String,
    pub parents: HashSet<Uuid>,
    pub children: HashSet<Uuid>,
}

impl Part {
    pub fn new(name: &str) -> Part {
        Part {
            id: Uuid::new_v3(&Uuid::NAMESPACE_URL, name.as_bytes()),
            name: String::from(name),
            parents: HashSet::new(),
            children: HashSet::new(),
        }
    }
}

impl Clone for Part {
    fn clone(&self) -> Self {
        Part {
            id: self.id.clone(),
            name: self.name.clone(),
            parents: self.parents.clone(),
            children: self.children.clone(),
        }
    }
}

impl Ord for Part {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Part {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Part {}

pub enum PartsListFilter {
    All,
    TopLevel,
    Assembly,
    Component,
    Subassembly,
    Orphan
}

impl TryFrom<&str> for PartsListFilter { 
    type Error = PartsListError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "" => Ok(PartsListFilter::All),
            "all" => Ok(PartsListFilter::All),
            "top_level" => Ok(PartsListFilter::TopLevel),
            "assembly" => Ok(PartsListFilter::Assembly),
            "component" => Ok(PartsListFilter::Component),
            "subassembly" => Ok(PartsListFilter::Subassembly),
            "orphan" => Ok(PartsListFilter::Orphan),
            _ => Err(PartsListError::InvalidFilterString{s: s.into()})
        }
    }
}

impl From<PartsListFilter> for &str { 
    fn from(f: PartsListFilter) -> &'static str {
        match f {
            PartsListFilter::All => "all",
            PartsListFilter::TopLevel => "top_level",
            PartsListFilter::Assembly => "assembly",
            PartsListFilter::Component => "component",
            PartsListFilter::Subassembly => "subassembly",
            PartsListFilter::Orphan => "orphan"
        }
    }
}

#[derive(Error, Debug)]
pub enum PartsListError {
    /// Error occuring when attempting to retrieve non-existant part
    #[error("Part does not exist (id: {id:?})")]
    PartDoesNotExist {
        id: Uuid,
    },

    /// Error occuring when attempting to add a part that already exists
    #[error("Hash collision, part already exists (name: {name:?}, id: {id:?})")]
    PartExists {
        name: String,
        id: Uuid,
    },

    /// Error occuring when attempting to add a child who is already a parent of the part
    #[error("Cycle detected, part has child in its parental line (parent: {parent:?}, child: {child:?})")]
    AddChildCyclicalRelative {
        parent: Uuid,
        child: Uuid,
    },

    /// Failure to parse string into valid PartsListFilter
    #[error("Invalid string: {s:?}, unable to convert into PartsListFilter")]
    InvalidFilterString {
        s: String,
    },

    /// Failure to parse string into valid PartsListUpdate
    #[error("Invalid string: {s:?}, unable to convert into PartsListUpdate")]
    InvalidUpdateString {
        s: String,
    },

    /// Unhandled filter type for operation
    #[error("Invalid filter operation {s:?} for {s:?}, unable to execute")]
    InvalidFilterChoice {
        s: String,
        f: String,
    },

    /// Unknown error related to parts list
    #[error("unknown parts list error")]
    Unknown,
}

pub enum PartsListUpdate {
    Add,
    Remove,
    Replace,
}

impl TryFrom<&str> for PartsListUpdate { 
    type Error = PartsListError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "" => Ok(PartsListUpdate::Add),
            "add" => Ok(PartsListUpdate::Add),
            "remove" => Ok(PartsListUpdate::Remove),
            "replace" => Ok(PartsListUpdate::Replace),
            _ => Err(PartsListError::InvalidUpdateString{s: s.into()})
        }
    }
}

impl From<PartsListUpdate> for &str { 
    fn from(f: PartsListUpdate) -> &'static str {
        match f {
            PartsListUpdate::Add => "add",
            PartsListUpdate::Remove => "remove",
            PartsListUpdate::Replace => "replace",
        }
    }
}

#[derive(Serialize)]
pub struct PartsList(HashMap<Uuid, Part>);

impl PartsList {
    pub fn new() -> PartsList {
        PartsList(HashMap::new())
    }

    pub fn get(&self, id: &Uuid) -> Result<&Part, PartsListError> {
        if let Some(part) = self.0.get(id) {
            Ok(part)
        } else {
            Err(PartsListError::PartDoesNotExist {
                id: id.clone()
            })
        }
    }

    pub fn get_mut(&mut self, id: &Uuid) -> Result<&mut Part, PartsListError> {
        if let Some(part) = self.0.get_mut(id) {
            Ok(part)
        } else {
            Err(PartsListError::PartDoesNotExist {
                id: id.clone()
            })
        }
    }

    pub fn add(&mut self, new_part: Part) -> Result<&Part, PartsListError> {
        let id = new_part.id.clone();
        // Check for part id collision based on name
        if !self.0.contains_key(&id) {
            self.0.insert(id.clone(), new_part);
            if let Some(part) = self.0.get(&id) {
                Ok(part)
            } else {
                Err(PartsListError::Unknown)
            }
        } else {
            Err(PartsListError::PartExists {
                name: new_part.name.into(),
                id: new_part.id
            })
        }
    }

    pub fn delete(&mut self, id: &Uuid) -> Result<(), PartsListError> {
        // Make sure part exists
        if let Some(part) = self.0.remove(id) {
            // Remove part from all parents and children
            for parent in part.parents {
                self.get_mut(&parent).unwrap().children.remove(id);
            }
            for child in part.children {
                self.get_mut(&child).unwrap().parents.remove(id);
            }
            // Finally remove actual part
            Ok(())
        } else {
            Err(PartsListError::PartDoesNotExist {
                id: id.clone()
            })
        }
    }

    fn recurse_parts_list<'a, T, V>(&'a self, id: &Uuid, accumulate: &mut HashMap<&'a Uuid, &'a Part>, next_set: &T, test: &V) -> Result<(), PartsListError>
        where T: Fn(&Part) -> Vec<&Uuid>,
              V: Fn(&Part) -> bool
    {
        let part = self.get(id)?;
        for child in next_set(part) {
            let child = self.get(child)?;
            if test(child) {
                accumulate.insert(&child.id, child);
            }
            self.recurse_parts_list(&child.id, accumulate, next_set, test)?;
        }
        Ok(())
    }

    fn recurse_match(&self, next_set: fn(&Part) -> Vec<&Uuid>, parts: Vec<&Uuid>, candidate: &Uuid) -> Result<bool, PartsListError> {
        for part in parts {
            // found a match
            if part == candidate {
                return Ok(true);
            }
            let part = self.get(part)?;
            // recurse for further matches
            if self.recurse_match(next_set, next_set(&part), candidate)? {
                return Ok(true);
            }
        }
        // no matches found after exhausting the list
        Ok(false)
    }

    fn get_part_children(part: &Part) -> Vec<&Uuid> {
        part.children.iter().collect()
    }

    fn get_part_parents(part: &Part) -> Vec<&Uuid> {
        part.parents.iter().collect()
    }

    fn is_ancestor(&self, part: &Uuid, candidate: &Uuid) -> Result<bool, PartsListError> {
        self.recurse_match(PartsList::get_part_parents, PartsList::get_part_parents(self.get(part)?), candidate)
    }

    pub fn get_children(&self, id: &Uuid, filter: PartsListFilter) -> Result<Vec<&Part>, PartsListError> {
        match filter {
            PartsListFilter::All => {
                let mut acc = HashMap::new();
                self.recurse_parts_list(id, &mut acc, &PartsList::get_part_children, &|_| {true})?;
                Ok(acc.values().copied().collect())
            },
            PartsListFilter::TopLevel => {
                let children = self.get(id)?.children.iter().map(|x| {self.get(x)}).collect::<Result<Vec<_>, _>>()?;
                Ok(children)
            },
            PartsListFilter::Component => {
                let mut acc = HashMap::new();
                let test = |x: &Part| {!x.parents.is_empty() && x.children.is_empty()};
                self.recurse_parts_list(id, &mut acc, &PartsList::get_part_children, &test)?;
                Ok(acc.values().copied().collect())
            },
            PartsListFilter::Assembly => {
                let mut acc = HashMap::new();
                self.recurse_parts_list(id, &mut acc, &PartsList::get_part_parents, &|_| {true})?;
                Ok(acc.values().copied().collect())
            },
            _ => { return Err(PartsListError::InvalidFilterChoice{ s: "get_children".into(), f: String::from(Into::<&str>::into(filter)) }); }
        }
    }

    fn add_children(&mut self, parent: &Uuid, children: &Vec<&Uuid>) -> Result<(), PartsListError> {
        // add each child one at a time
        for child in children {
            // can't add itself as a child
            if parent == *child {
                return Err(PartsListError::AddChildCyclicalRelative{ parent: *parent, child: **child });
            }
            // check child does not have parent in tree
            if self.is_ancestor(parent, child)? {
                return Err(PartsListError::AddChildCyclicalRelative{ parent: *parent, child: **child });
            } else {
                // actually add child and update parents
                {
                    let parent_ref = self.get_mut(parent)?;
                    parent_ref.children.insert(*child.clone());
                }
                let child_ref = self.get_mut(child)?;
                child_ref.parents.insert(parent.clone());
            }
        
        }
        Ok(())
    }

    fn remove_children(&mut self, parent: &Uuid, children: &Vec<&Uuid>) -> Result<(), PartsListError> {
        // add each child one at a time
        for child in children {
            let child_ref = self.get_mut(child)?;
            assert!(child_ref.parents.remove(parent));
            // remove child from parent and update child to remove parent
            {
                let parent_ref = self.get_mut(parent)?;
                assert!(parent_ref.children.remove(child));
            }
        }
        Ok(())
    }

    pub fn update(&mut self,
                  id: &Uuid,
                  children: &Vec<&Uuid>,
                  op: PartsListUpdate) -> Result<(), PartsListError> {
        match op {
            PartsListUpdate::Add => { self.add_children(&id, children) },
            PartsListUpdate::Remove => { self.remove_children(&id, children) },
            PartsListUpdate::Replace => {
                let part = self.get(&id)?.clone();
                let old_children = &PartsList::get_part_children(&part);
                self.remove_children(&id, old_children)?;
                self.add_children(&id, children)
            }
        }
    }

    pub fn list(&self, filter: PartsListFilter) -> Vec<&Part> {
        match filter {
            PartsListFilter::All => self.0.values().collect(),
            PartsListFilter::TopLevel => self.0.values().filter(|x| {x.parents.is_empty()}).collect(),
            PartsListFilter::Assembly => self.0.values().filter(|x| {!x.children.is_empty()}).collect(),
            PartsListFilter::Component => {
               self.0.values().filter(|x| {!x.parents.is_empty() && x.children.is_empty()}).collect()
            },
            PartsListFilter::Subassembly => {
               self.0.values().filter(|x| {!x.parents.is_empty() && !x.children.is_empty()}).collect()
            },
            PartsListFilter::Orphan => {
               self.0.values().filter(|x| {x.parents.is_empty() && x.children.is_empty()}).collect()
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_contains_part<'a, 'b, T: Iterator<Item = &'a &'a Part>>(list_iter: &mut T, part: &'b Part) -> bool 
    {
        list_iter.find(|x| {x.id == part.id}).is_none() == false
    }

    fn list_compare(list: &[&Part], other: &[&Part]) -> ()
    {
        let mut list: Vec<&Part> = list.iter().copied().collect();
        let mut other: Vec<&Part> = other.iter().copied().collect();
        assert_eq!(list.sort(), other.sort());
    }

    #[test]
    fn create_part() {
        let name = "my part";
        let part = Part::new(name);
        assert_eq!(name, part.name);
        assert!(part.parents.is_empty());
        assert!(part.children.is_empty());
    }

    #[test]
    fn basic_parts_add() {
        let mut parts = PartsList::new();
        let name = "my part";
        let part = Part::new(name);
        let ret_part = parts.add(part.clone()).unwrap();
        assert_eq!(part, *ret_part);
        assert_eq!(part.name, *ret_part.name);
    }

    #[test]
    fn add_duplicate_part() {
        let mut parts = PartsList::new();
        let name = "my part";
        let part = Part::new(name);
        let ret_part = parts.add(part.clone()).unwrap();
        assert_eq!(part, *ret_part);
        let result = parts.add(part.clone());
        assert_matches!(result, Err(e) => {
            assert_matches!(e, PartsListError::PartExists{..});
        });
    }


    #[test]
    fn basic_parts_add_and_list() {
        let mut parts = PartsList::new();
        let name1 = "my part";
        let name2 = "other part";
        let part1 = Part::new(name1);
        let part2 = Part::new(name2);
        let _ = parts.add(part1.clone()).unwrap();
        let _ = parts.add(part2.clone()).unwrap();
        let list = parts.list(PartsListFilter::All);
        assert!(list_contains_part(&mut list.iter(), &part1));
        assert!(list_contains_part(&mut list.iter(), &part2));
    }

    #[test]
    fn create_parts_and_add_child() {
        let mut parts = PartsList::new();
        let name1 = "my part";
        let name2 = "other part";
        let part1 = Part::new(name1);
        let part2 = Part::new(name2);
        let _ = parts.add(part1.clone()).unwrap();
        let _ = parts.add(part2.clone()).unwrap();
        parts.update(&part1.id, &vec![&part2.id],  PartsListUpdate::Add).unwrap();
        assert!(parts.get(&part1.id).unwrap().children.contains(&part2.id));
        assert!(parts.get(&part2.id).unwrap().parents.contains(&part1.id));
    }

    #[test]
    fn remove_child_of_part() {
        let mut parts = PartsList::new();
        let name1 = "my part";
        let name2 = "other part";
        let part1 = Part::new(name1);
        let part2 = Part::new(name2);
        let _ = parts.add(part1.clone()).unwrap();
        let _ = parts.add(part2.clone()).unwrap();
        parts.update(&part1.id, &vec![&part2.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part1.id, &vec![&part2.id],  PartsListUpdate::Remove).unwrap();
        assert!(parts.get(&part1.id).unwrap().children.contains(&part2.id) == false);
        assert!(parts.get(&part2.id).unwrap().parents.contains(&part1.id) == false);
    }

    #[test]
    fn list_top_level_parts() {
        let mut parts = PartsList::new();
        let part1 = parts.add(Part::new("my part")).unwrap().clone();
        let part2 = parts.add(Part::new("other part")).unwrap().clone();
        parts.update(&part1.id, &vec![&part2.id],  PartsListUpdate::Add).unwrap();
        let list = parts.list(PartsListFilter::TopLevel);
        assert_eq!(list_contains_part(&mut list.iter(), &part1), true);
        assert_eq!(list_contains_part(&mut list.iter(), &part2), false);
    }

    #[test]
    fn list_orphan_parts() {
        let mut parts = PartsList::new();
        let part1 = parts.add(Part::new("my part")).unwrap().clone();
        let part2 = parts.add(Part::new("other part")).unwrap().clone();
        let part3 = parts.add(Part::new("orphan part")).unwrap().clone();
        parts.update(&part1.id, &vec![&part2.id],  PartsListUpdate::Add).unwrap();
        let list = parts.list(PartsListFilter::Orphan);
        assert_eq!(list_contains_part(&mut list.iter(), &part1), false);
        assert_eq!(list_contains_part(&mut list.iter(), &part2), false);
        assert_eq!(list_contains_part(&mut list.iter(), &part3), true);
    }

    #[test]
    fn test_get_children() {
        let mut parts = PartsList::new();
        let part1 = parts.add(Part::new("my part")).unwrap().clone();
        let part2 = parts.add(Part::new("other part")).unwrap().clone();
        let part3 = parts.add(Part::new("subassy")).unwrap().clone();
        let part4 = parts.add(Part::new("deep component")).unwrap().clone();

        parts.update(&part1.id, &vec![&part3.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part2.id, &vec![&part3.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part3.id, &vec![&part4.id],  PartsListUpdate::Add).unwrap();

        let list = parts.get_children(&part1.id, PartsListFilter::TopLevel).unwrap();
        list_compare(&list, &vec![&part3]);

        let list = parts.get_children(&part1.id, PartsListFilter::Component).unwrap();
        list_compare(&list, &vec![&part4]);

        let list = parts.get_children(&part1.id, PartsListFilter::All).unwrap();
        list_compare(&list, &vec![&part3, &part4]);

        let list = parts.get_children(&part4.id, PartsListFilter::Assembly).unwrap();
        list_compare(&list, &vec![&part1, &part2, &part3]);
    }

    #[test]
    fn test_update_children() {
        let mut parts = PartsList::new();
        let part1 = parts.add(Part::new("my part")).unwrap().clone();
        let part2 = parts.add(Part::new("other part")).unwrap().clone();
        let part3 = parts.add(Part::new("subassy")).unwrap().clone();
        let part4 = parts.add(Part::new("deep component")).unwrap().clone();

        parts.update(&part1.id, &vec![&part3.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part2.id, &vec![&part3.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part3.id, &vec![&part4.id],  PartsListUpdate::Add).unwrap();

        let list = parts.get_children(&part1.id, PartsListFilter::All).unwrap();
        list_compare(&list, &vec![&part3, &part4]);

        parts.update(&part1.id, &vec![&part2.id, &part3.id, &part4.id],  PartsListUpdate::Replace).unwrap();

        let list = parts.get_children(&part1.id, PartsListFilter::All).unwrap();
        list_compare(&list, &vec![&part2, &part3, &part4]);
    }

    #[test]
    fn test_delete_part() {
        let mut parts = PartsList::new();
        let part1 = parts.add(Part::new("my part")).unwrap().clone();
        let part2 = parts.add(Part::new("other part")).unwrap().clone();
        let part3 = parts.add(Part::new("subassy")).unwrap().clone();
        let part4 = parts.add(Part::new("deep component")).unwrap().clone();

        parts.update(&part1.id, &vec![&part3.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part2.id, &vec![&part3.id],  PartsListUpdate::Add).unwrap();
        parts.update(&part3.id, &vec![&part4.id],  PartsListUpdate::Add).unwrap();

        let list = parts.list(PartsListFilter::All);
        list_compare(&list, &vec![&part1, &part2, &part3, &part4]);
        parts.delete(&part3.id).unwrap();
        let list = parts.list(PartsListFilter::All);
        list_compare(&list, &vec![&part1, &part2, &part4]);
        assert_eq!(parts.get(&part1.id).unwrap().children.contains(&part3.id), false);
        assert_eq!(parts.get(&part1.id).unwrap().parents.contains(&part3.id), false);
        assert_eq!(parts.get(&part2.id).unwrap().children.contains(&part3.id), false);
        assert_eq!(parts.get(&part2.id).unwrap().parents.contains(&part3.id), false);
        assert_eq!(parts.get(&part4.id).unwrap().children.contains(&part3.id), false);
        assert_eq!(parts.get(&part4.id).unwrap().parents.contains(&part3.id), false);
    }

}
