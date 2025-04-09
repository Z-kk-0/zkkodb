use serde::Deserialize;

// read json string, reading the command string and match it
#[derive(Debug, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    #[serde(rename = "create")]
    Create(CreateCommand),
    /* 
    #[serde(rename = "read")]
    Read(ReadCommand),

    #[serde(rename = "update")]
    Update(UpdateCommand),

    #[serde(rename = "insert")]
    Insert(InsertCommand),

    #[serde(rename = "delete")]
    Delete(DeleteCommand),

    Unknown(String)
    */
}

// differenciates a User create from a table create
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
        tablename: String,
        primary_key: String,
        rows: std::collections::HashMap<String, ColumnDefinition>,
    }
}

pub enum ReadCommand {
    //TODO
}

pub enum InsertCommand {
    //TODO
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
          "tablename": "products",
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
}
