use std::collections::HashMap;
use serde::Deserialize;

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

    
    #[serde(rename = "delete")]
    Delete(DeleteCommand),

    /*
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
    pub rows: std::collections::HashMap<String, serde_json::Value>
}
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum DeleteCommand {
    #[serde(rename = "table")]
    Table {
      table: String
    },
    #[serde(rename = "content")]
    Content {
      table: String,
      filter: String
    }
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
