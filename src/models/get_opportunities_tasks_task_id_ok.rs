/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOpportunitiesTasksTaskIdOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOpportunitiesTasksTaskIdOk {
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// notification string
    #[serde(rename = "notification")]
    pub notification: String,
    /// task_id integer
    #[serde(rename = "task_id")]
    pub task_id: i32,
}

impl GetOpportunitiesTasksTaskIdOk {
    /// 200 ok object
    pub fn new(
        description: String,
        name: String,
        notification: String,
        task_id: i32,
    ) -> GetOpportunitiesTasksTaskIdOk {
        GetOpportunitiesTasksTaskIdOk {
            description,
            name,
            notification,
            task_id,
        }
    }
}
