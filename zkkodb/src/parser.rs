use std::collections::HashMap;

use serde::Deserialize;

// read json string, reading the command string and match it
#[derive(Debug, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    #[serde(rename = "create")]
    Create(CreateCommand),
     
    #[serde(rename = "read")]
    Read(ReadCommand),

    
    #[serde(rename = "update")]
    Update(UpdateCommand),
    
     
    #[serde(rename = "insert")]
    Insert(InsertCommand),

    /*
    #[serde(rename = "delete")]
    Delete(DeleteCommand),

    Unknown(String)
    */
}

// differentiates a User create from a table create
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum CreateCommand {
    #[serde(rename = "user")]
    User {
        username: String,
        password: String,
        role: String,
    },

    #[serde(rename = "table")]
    Table {
        table: String,
        primary_key: String,
        rows: std::collections::HashMap<String, ColumnDefinition>,
    }
}

#[derive(Debug, Deserialize)]
pub struct ReadCommand {
    pub table: String,
    #[serde(default)]
    pub filter: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub limit: Option<usize>,
}
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum UpdateCommand {
  #[serde(rename = "rows")]
  Rows {
    table: String, 
    add: std::collections::HashMap<String, ColumnDefinition>,
  },

  #[serde(rename = "content")]
  Content {
    table: String,
    filter: String,
    rows: std::collections::HashMap<String, serde_json::Value>
  }
}
#[derive(Debug, Deserialize)]
pub struct  InsertCommand {
    pub table: String,
    pub filter: String,
    pub rows: std::collections::HashMap<String, serde_json::Value>
}

pub enum DeleteCommand {
    //TODO
}

#[derive(Debug, Deserialize)]
pub struct ColumnDefinition {
    #[serde(rename = "type")]
    pub col_type: String,

    #[serde(default)]
    pub not_null: bool,

    #[serde(default)]
    pub unique: bool,

    #[serde(default)]
    pub default: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create_table() {
        let input = r#"
        {
          "command": "create",
          "type": "table",
          "table": "products",
          "primary_key": "id",
          "rows": {
            "id": {
              "type": "int",
              "not_null": true,
              "unique": true
            },
            "product": {
              "type": "string",
              "default": "Unnamed"
            },
            "price": {
              "type": "float"
            }
          }
        }
        "#;

        let parsed: Command = serde_json::from_str(input).unwrap();
        println!("{:?}", parsed);
    }

    #[test]
    fn test_parse_create_user() {
        let input = r#"
        {
          "command": "create",
          "type": "user",
          "username": "zkko",
          "password": "pwd",
          "role": "admin"
        }
        "#;

        let parsed: Command = serde_json::from_str(input).unwrap();
        println!("{:?}", parsed); 
    }

    #[test]
    fn test_parse_read_table() {
        let input = r#"
        {
          "command": "read",
          "table": "products",
          "filter": {
            "price": "20"
          },
          "limit": 5
        }
        "#;

        let parsed: Command = serde_json::from_str(input).unwrap();
        println!("{:?}", parsed);
        match parsed {
            Command::Read(cmd) => {
                assert_eq!(cmd.table, "products");
                assert_eq!(cmd.filter.get("price").unwrap(), "20");
                assert_eq!(cmd.limit, Some(5));
            }
            _ => panic!("Expected read command"),
        }
    }
    
    #[test]
    fn test_parse_read_table_minimal() {
        let input = r#"
        {
          "command": "read",
          "table": "products"
        }
        "#;
    
        let parsed: Command = serde_json::from_str(input).unwrap();
        match parsed {
            Command::Read(cmd) => {
                assert_eq!(cmd.table, "products");
                assert!(cmd.filter.is_empty());
                assert_eq!(cmd.limit, None);
            },
            _ => panic!("Expected Command::Read"),
        }
    }
    #[test]
    fn test_parse_update_rows() {
        let input = r#"
        {
            "command": "update",
            "type": "rows",
            "table": "products",
            "add": {
                "category": {
                    "type": "string"
                }
            }
        }
        "#;
    
        let parsed: Command = serde_json::from_str(input).unwrap();
    
        match parsed {
            Command::Update(UpdateCommand::Rows { table, add }) => {
                assert_eq!(table, "products");
                assert_eq!(add.get("category").unwrap().col_type, "string");
            }
            _ => panic!("Expected Command::Update::Rows"),
        }
    }

    #[test]
    fn test_parse_update_content() {
        let input = r#"
        {
          "command": "update",
          "type": "content",
          "table": "products",
          "filter": "id = 1",
          "rows": {
            "price": 2.30
          }
        }
        "#;
        
        let parsed: Command = serde_json::from_str(input).unwrap();
    
        match parsed {
            Command::Update(UpdateCommand::Content { table, filter, rows }) => {
                assert_eq!(table, "products");
                assert_eq!(filter, "id = 1");
    
                let price = rows.get("price").unwrap().as_f64().unwrap();
                assert_eq!(price, 2.30);
            }
            _ => panic!("Expected Command::Update::Content"),
        }
    }
    
    
}
