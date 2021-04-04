pub mod introspection_query {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "IntrospectionQuery";
    pub const QUERY: &'static str = "query IntrospectionQuery {\n  __schema {\n    queryType {\n      name\n    }\n    mutationType {\n      name\n    }\n    subscriptionType {\n      name\n    }\n    types {\n      ...FullType\n    }\n    directives {\n      name\n      description\n      locations\n      args {\n        ...InputValue\n      }\n    }\n  }\n}\n\nfragment FullType on __Type {\n  kind\n  name\n  description\n  fields(includeDeprecated: true) {\n    name\n    description\n    args {\n      ...InputValue\n    }\n    type {\n      ...TypeRef\n    }\n    isDeprecated\n    deprecationReason\n  }\n  inputFields {\n    ...InputValue\n  }\n  interfaces {\n    ...TypeRef\n  }\n  enumValues(includeDeprecated: true) {\n    name\n    description\n    isDeprecated\n    deprecationReason\n  }\n  possibleTypes {\n    ...TypeRef\n  }\n}\n\nfragment InputValue on __InputValue {\n  name\n  description\n  type {\n    ...TypeRef\n  }\n  defaultValue\n}\n\nfragment TypeRef on __Type {\n  kind\n  name\n  ofType {\n    kind\n    name\n    ofType {\n      kind\n      name\n      ofType {\n        kind\n        name\n        ofType {\n          kind\n          name\n          ofType {\n            kind\n            name\n            ofType {\n              kind\n              name\n              ofType {\n                kind\n                name\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum __DirectiveLocation {
        QUERY,
        MUTATION,
        SUBSCRIPTION,
        FIELD,
        FRAGMENT_DEFINITION,
        FRAGMENT_SPREAD,
        INLINE_FRAGMENT,
        SCHEMA,
        SCALAR,
        OBJECT,
        FIELD_DEFINITION,
        ARGUMENT_DEFINITION,
        INTERFACE,
        UNION,
        ENUM,
        ENUM_VALUE,
        INPUT_OBJECT,
        INPUT_FIELD_DEFINITION,
        Other(String),
    }
    impl ::serde::Serialize for __DirectiveLocation {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                __DirectiveLocation::QUERY => "QUERY",
                __DirectiveLocation::MUTATION => "MUTATION",
                __DirectiveLocation::SUBSCRIPTION => "SUBSCRIPTION",
                __DirectiveLocation::FIELD => "FIELD",
                __DirectiveLocation::FRAGMENT_DEFINITION => "FRAGMENT_DEFINITION",
                __DirectiveLocation::FRAGMENT_SPREAD => "FRAGMENT_SPREAD",
                __DirectiveLocation::INLINE_FRAGMENT => "INLINE_FRAGMENT",
                __DirectiveLocation::SCHEMA => "SCHEMA",
                __DirectiveLocation::SCALAR => "SCALAR",
                __DirectiveLocation::OBJECT => "OBJECT",
                __DirectiveLocation::FIELD_DEFINITION => "FIELD_DEFINITION",
                __DirectiveLocation::ARGUMENT_DEFINITION => "ARGUMENT_DEFINITION",
                __DirectiveLocation::INTERFACE => "INTERFACE",
                __DirectiveLocation::UNION => "UNION",
                __DirectiveLocation::ENUM => "ENUM",
                __DirectiveLocation::ENUM_VALUE => "ENUM_VALUE",
                __DirectiveLocation::INPUT_OBJECT => "INPUT_OBJECT",
                __DirectiveLocation::INPUT_FIELD_DEFINITION => "INPUT_FIELD_DEFINITION",
                __DirectiveLocation::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for __DirectiveLocation {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "QUERY" => Ok(__DirectiveLocation::QUERY),
                "MUTATION" => Ok(__DirectiveLocation::MUTATION),
                "SUBSCRIPTION" => Ok(__DirectiveLocation::SUBSCRIPTION),
                "FIELD" => Ok(__DirectiveLocation::FIELD),
                "FRAGMENT_DEFINITION" => Ok(__DirectiveLocation::FRAGMENT_DEFINITION),
                "FRAGMENT_SPREAD" => Ok(__DirectiveLocation::FRAGMENT_SPREAD),
                "INLINE_FRAGMENT" => Ok(__DirectiveLocation::INLINE_FRAGMENT),
                "SCHEMA" => Ok(__DirectiveLocation::SCHEMA),
                "SCALAR" => Ok(__DirectiveLocation::SCALAR),
                "OBJECT" => Ok(__DirectiveLocation::OBJECT),
                "FIELD_DEFINITION" => Ok(__DirectiveLocation::FIELD_DEFINITION),
                "ARGUMENT_DEFINITION" => Ok(__DirectiveLocation::ARGUMENT_DEFINITION),
                "INTERFACE" => Ok(__DirectiveLocation::INTERFACE),
                "UNION" => Ok(__DirectiveLocation::UNION),
                "ENUM" => Ok(__DirectiveLocation::ENUM),
                "ENUM_VALUE" => Ok(__DirectiveLocation::ENUM_VALUE),
                "INPUT_OBJECT" => Ok(__DirectiveLocation::INPUT_OBJECT),
                "INPUT_FIELD_DEFINITION" => Ok(__DirectiveLocation::INPUT_FIELD_DEFINITION),
                _ => Ok(__DirectiveLocation::Other(s)),
            }
        }
    }
    #[derive(Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum __TypeKind {
        SCALAR,
        OBJECT,
        INTERFACE,
        UNION,
        ENUM,
        INPUT_OBJECT,
        LIST,
        NON_NULL,
        Other(String),
    }
    impl ::serde::Serialize for __TypeKind {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                __TypeKind::SCALAR => "SCALAR",
                __TypeKind::OBJECT => "OBJECT",
                __TypeKind::INTERFACE => "INTERFACE",
                __TypeKind::UNION => "UNION",
                __TypeKind::ENUM => "ENUM",
                __TypeKind::INPUT_OBJECT => "INPUT_OBJECT",
                __TypeKind::LIST => "LIST",
                __TypeKind::NON_NULL => "NON_NULL",
                __TypeKind::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for __TypeKind {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "SCALAR" => Ok(__TypeKind::SCALAR),
                "OBJECT" => Ok(__TypeKind::OBJECT),
                "INTERFACE" => Ok(__TypeKind::INTERFACE),
                "UNION" => Ok(__TypeKind::UNION),
                "ENUM" => Ok(__TypeKind::ENUM),
                "INPUT_OBJECT" => Ok(__TypeKind::INPUT_OBJECT),
                "LIST" => Ok(__TypeKind::LIST),
                "NON_NULL" => Ok(__TypeKind::NON_NULL),
                _ => Ok(__TypeKind::Other(s)),
            }
        }
    }
    #[derive(Deserialize)]
    pub struct FullTypeFieldsArgs {
        #[serde(flatten)]
        pub input_value: InputValue,
    }
    #[derive(Deserialize)]
    pub struct FullTypeFieldsType {
        #[serde(flatten)]
        pub type_ref: TypeRef,
    }
    #[derive(Deserialize)]
    pub struct FullTypeFields {
        pub name: String,
        pub description: Option<String>,
        pub args: Vec<FullTypeFieldsArgs>,
        #[serde(rename = "type")]
        pub type_: FullTypeFieldsType,
        #[serde(rename = "isDeprecated")]
        pub is_deprecated: Boolean,
        #[serde(rename = "deprecationReason")]
        pub deprecation_reason: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct FullTypeInputFields {
        #[serde(flatten)]
        pub input_value: InputValue,
    }
    #[derive(Deserialize)]
    pub struct FullTypeInterfaces {
        #[serde(flatten)]
        pub type_ref: TypeRef,
    }
    #[derive(Deserialize)]
    pub struct FullTypeEnumValues {
        pub name: String,
        pub description: Option<String>,
        #[serde(rename = "isDeprecated")]
        pub is_deprecated: Boolean,
        #[serde(rename = "deprecationReason")]
        pub deprecation_reason: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct FullTypePossibleTypes {
        #[serde(flatten)]
        pub type_ref: TypeRef,
    }
    #[derive(Deserialize)]
    pub struct FullType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        pub description: Option<String>,
        pub fields: Option<Vec<FullTypeFields>>,
        #[serde(rename = "inputFields")]
        pub input_fields: Option<Vec<FullTypeInputFields>>,
        pub interfaces: Option<Vec<FullTypeInterfaces>>,
        #[serde(rename = "enumValues")]
        pub enum_values: Option<Vec<FullTypeEnumValues>>,
        #[serde(rename = "possibleTypes")]
        pub possible_types: Option<Vec<FullTypePossibleTypes>>,
    }
    #[derive(Deserialize)]
    pub struct InputValueType {
        #[serde(flatten)]
        pub type_ref: TypeRef,
    }
    #[derive(Deserialize)]
    pub struct InputValue {
        pub name: String,
        pub description: Option<String>,
        #[serde(rename = "type")]
        pub type_: InputValueType,
        #[serde(rename = "defaultValue")]
        pub default_value: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfTypeOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfTypeOfType>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfType>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfTypeOfTypeOfTypeOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfType>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfTypeOfTypeOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfType>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfTypeOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfTypeOfTypeOfType>,
    }
    #[derive(Deserialize)]
    pub struct TypeRefOfType {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfTypeOfType>,
    }
    #[derive(Deserialize)]
    pub struct TypeRef {
        pub kind: __TypeKind,
        pub name: Option<String>,
        #[serde(rename = "ofType")]
        pub of_type: Option<TypeRefOfType>,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchemaQueryType {
        pub name: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchemaMutationType {
        pub name: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchemaSubscriptionType {
        pub name: Option<String>,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchemaTypes {
        #[serde(flatten)]
        pub full_type: FullType,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchemaDirectivesArgs {
        #[serde(flatten)]
        pub input_value: InputValue,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchemaDirectives {
        pub name: String,
        pub description: Option<String>,
        pub locations: Vec<__DirectiveLocation>,
        pub args: Vec<IntrospectionQuerySchemaDirectivesArgs>,
    }
    #[derive(Deserialize)]
    pub struct IntrospectionQuerySchema {
        #[serde(rename = "queryType")]
        pub query_type: IntrospectionQuerySchemaQueryType,
        #[serde(rename = "mutationType")]
        pub mutation_type: Option<IntrospectionQuerySchemaMutationType>,
        #[serde(rename = "subscriptionType")]
        pub subscription_type: Option<IntrospectionQuerySchemaSubscriptionType>,
        pub types: Vec<IntrospectionQuerySchemaTypes>,
        pub directives: Vec<IntrospectionQuerySchemaDirectives>,
    }
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct IntrospectionQuery {
        #[serde(rename = "__schema")]
        pub schema: Option<IntrospectionQuerySchema>,
    }
}
pub use introspection_query::IntrospectionQuery;

impl graphql_client::GraphQLRequest for IntrospectionQuery {
    type Variables = introspection_query::Variables;
    
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: introspection_query::QUERY,
            operation_name: introspection_query::OPERATION_NAME,
        }
    }
}
