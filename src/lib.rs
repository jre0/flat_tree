// The organizational structure of a company is provided as a tree. The tree is structured as an object keyed by employee name. The value for a given key is a list of names of people who report to that employee. The list of reports may be empty.

// Example input:
/*
const tree = {
    "Jane Mayer": ["Baraka Tumuti", "Sarah Lee", "David Heinsburg"],
    "Baraka Tumuti": ["Abida Begum"],
    "Sarah Lee": ["David Gibbly", "Kelsey Hamming"],
    "David Heinsburg": [],
    "Abida Begum": ["Dave Bunt", "James Ray"],
    "David Gibbly": [],
    "Kelsey Hamming": [],
    "Dave Bunt": [],
    "James Ray": [],
  }
  */

// Given the name of an employee, print all of the names in the part of the organization they lead.

// Examples:

// Input: "Jane Mayer"
// Output (in any order):
//  Jane Mayer
//  Baraka Tumuti
//  Sarah Lee
//  David Heinsburg
//  Abida Begum
//  David Gibbly
//  Kelsey Hamming
//  Dave Bunt
//  James Ray

// Input: "Abida Begum"
// Output (in any order):
//  Abida Begum
//  Dave Bunt
//  James Ray

// Input: "James Ray"
// Output (in any order):
//  James Ray

///////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

/// Tree Error (catch-all)
pub type Error = Box<dyn std::error::Error>;

/// This `Organization` structure accurately reflects the JS Obj / JSON input which is not a regular tree.
/// It is a flat form of tree by intention.
/// The previus `Node` structure was already a tree, circumventing a big part of the
/// challenge by mistake.
#[derive(Default)]
pub struct Organization {
    pub people: HashMap<String, Vec<String>>,
}

impl Organization {
    /// Import serial json data into new Organization instance
    /// This method needs to be broken into smaller methods
    pub fn import(serial: &str) -> Result<Self, Error> {
        // Parse the string into a recursive JSON data structure as seen at https://github.com/serde-rs/json
        let json: serde_json::Value = serde_json::from_str(serial)?;
        let mut org = Self::default();
        // Traverse the `serde_json::Value` to build our `Organization`
        if let serde_json::Value::Object(obj) = json {
            for (key, value) in obj {
                if let serde_json::Value::Array(array) = value {
                    let mut child_names = vec![];
                    for element in array {
                        if let serde_json::Value::String(name) = element {
                            child_names.push(name);
                        } else {
                            Err("Parse error: array element not a string")?
                        }
                    }
                    org.people.insert(key, child_names);
                } else {
                    Err("Parse error: obj entry not an array")?
                }
            }
            // return the organization
            Ok(org)
        } else {
            Err("Parse error: top not obj")?
        }
    }

    /// Flatten the structure into a list of names found under the given current name.
    /// This is similar to the original `Node` struct method but it traverses the tree
    /// where nodes are kept in the HashMap (vary similar to JS Object).
    pub fn flat(&self, current: &String, output: &mut Vec<String>) -> Result<Vec<String>, Error> {
        output.push(current.clone());
        let names = self.people.get(current).ok_or("entry not found")?;
        for name in names {
            self.flat(name, output)?;
        }
        Ok(output.clone())
    }
}

// run with
#[cfg(test)]
mod test {
    use super::*;

    /// This data can be obtained from a js object
    /// I used a JSON formatter to make it clean for serde_json (deserialization into serde_json::Value)
    const SERIAL_DATA: &str = r#"{"Jane Mayer":["Baraka Tumuti","Sarah Lee","David Heinsburg"],"Baraka Tumuti":["Abida Begum"],"Sarah Lee":["David Gibbly","Kelsey Hamming"],"David Heinsburg":[],"Abida Begum":["Dave Bunt","James Ray"],"David Gibbly":[],"Kelsey Hamming":[],"Dave Bunt":[],"James Ray":[]}"#;

    #[test]
    fn import_json_and_flatten_to_name_list() -> Result<(), Error> {
        let org = Organization::import(SERIAL_DATA)?;
        let mut names: Vec<String> = vec![];
        org.flat(&"Jane Mayer".into(), &mut names)?;
        println!("names: {:?}", names);
        assert_eq!(
            names,
            vec![
                "Jane Mayer",
                "Baraka Tumuti",
                "Abida Begum",
                "Dave Bunt",
                "James Ray",
                "Sarah Lee",
                "David Gibbly",
                "Kelsey Hamming",
                "David Heinsburg"
            ]
        );
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////
// Old code that did not correctly model the input structure:

/// name tree
#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub nodes: Vec<Box<Node>>,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            nodes: vec![],
        }
    }
    pub fn push(&mut self, node: Node) -> &mut Self {
        self.nodes.push(Box::new(node));
        self
    }
    pub fn flat(&self, output: &mut Vec<String>) -> Vec<String> {
        output.push(self.name.clone());
        for child in &self.nodes {
            child.flat(output);
        }
        output.clone()
    }
}

fn main() {
    let mut dave_bunt = Node::new("Dave Bunt".into());
    let mut james_ray = Node::new("James Ray".into());
    let mut kelsey_hamming = Node::new("Kelsey Hamming".into());
    let mut david_gibbly = Node::new("David Gibbly".into());

    let mut jane_mayer = Node::new("Jane Mayer".into());
    let mut baraka_tumuti = Node::new("Baraka Tumuti".into());
    let mut sarah_lee = Node::new("Sarah Lee".into());
    sarah_lee.push(david_gibbly);
    sarah_lee.push(kelsey_hamming);
    let mut david_heinsburg = Node::new("David Heinsburg".into());

    let mut abida_begum = Node::new("Abida Begum".into());
    abida_begum.push(dave_bunt);
    abida_begum.push(james_ray);
    baraka_tumuti.push(abida_begum);

    jane_mayer.push(baraka_tumuti);
    jane_mayer.push(sarah_lee);
    jane_mayer.push(david_heinsburg);

    let mut names: Vec<String> = vec![];
    jane_mayer.flat(&mut names);

    println!("Names: {:?}", names);
}
