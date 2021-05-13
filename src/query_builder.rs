use postgres::types::ToSql;
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug, PartialEq, Eq, Hash)]
enum EQueryFunction {
    Operation,
    TableName,
    Schema,
    Location, 
    DataIdentifier,
}

#[derive(Debug)]
struct QueryBuilder{
    operations: HashMap<EQueryFunction, String>,
    values: Vec<&'static (dyn ToSql + Sync)>,
    payload_size: u8,
}

#[allow(dead_code)]
impl QueryBuilder {

    pub fn data(&mut self, data: &'static [&(dyn ToSql + Sync)]) -> &mut Self {
        let data_vec = data.to_vec();
       self.values = data_vec;
       self
    }

    pub fn data_identifiers(&mut self, identifiers: &[&str]) -> &mut Self {
        self.payload_size = identifiers.len() as u8;
        self.operations.insert(EQueryFunction::DataIdentifier, identifiers.join(", "));
        
        self
    }

    pub fn table_name(&mut self, table_name: &str) -> &mut Self {
        self.operations.insert(EQueryFunction::TableName, table_name.to_string());
        self
    }

    pub fn location(&mut self, location: &str) -> &mut Self {
        self.operations.insert(EQueryFunction::Location, location.to_string());
        self
    }

    pub fn schema(&mut self, schema: &str) -> &mut Self {
        self.operations.insert(EQueryFunction::Schema, schema.to_string());
        self
    }
   
    pub fn operation(&mut self, operation_type: &str) -> &mut Self {
        self.operations.insert(EQueryFunction::Operation, operation_type.to_string());
        self
    }

    pub fn new() -> Self {
        let tes = QueryBuilder{operations: std::collections::HashMap::new(), values: Vec::new(), payload_size: 0};
        tes
    }
}

impl ToString for QueryBuilder {
    fn to_string(&self) -> String {
        let hash = &self.operations;

        let returnable;
        match hash.get(&EQueryFunction::Operation).unwrap().to_lowercase().as_ref() {
            "insert" => {
                let location_index = [hash.get(&EQueryFunction::Schema).unwrap().to_string(), hash.get(&EQueryFunction::Location).unwrap().to_string()].join(".");
                let values_names = hash.get(&EQueryFunction::DataIdentifier).unwrap();
                let mut values_template = String::new();
                
                for i in 1 ..= self.payload_size {
                    if values_template.is_empty() {
                        values_template = format!("${}", i);
                        continue;
                    }

                    values_template = [values_template, format!("${}", i)].join(", ").to_string();
                }

                returnable = format!("INSERT INTO {} ({}) VALUES ({});", location_index, values_names, values_template);
            },
            "create" => {
                let schema_name = hash.get(&EQueryFunction::Schema).unwrap().to_string();
                let table_name = hash.get(&EQueryFunction::TableName).unwrap().to_string();
                let values_names = hash.get(&EQueryFunction::DataIdentifier).unwrap().to_string();
                returnable = format!("CREATE TABLE {} ({});", [schema_name, table_name].join("."), values_names);
            }
            _ => unimplemented!()
        }

        returnable
    }
}

#[test]
pub fn insert_test() {
    let qb = QueryBuilder::new()
    .operation("INSERT")
    .schema("public")
    .location("location_data")
    .data_identifiers(&["identifier", "owner", "previous_hash", "next_hash", "lat", "long", "speed", "creation_date"]).to_string();


    assert_eq!("INSERT INTO public.location_data (identifier, owner, previous_hash, next_hash, lat, long, speed, creation_date) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
    qb)
}
#[test]
pub fn create_test() {
    let qb = QueryBuilder::new()
    .operation("CREATE")
    .schema("private")
    .table_name("notes")
    .data_identifiers(&["note STRING"]).to_string();

    assert_eq!("CREATE TABLE private.notes (note STRING);",
    qb) 
}