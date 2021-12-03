/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdIndustryJobs200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdIndustryJobs200Ok {
    /// Job activity ID
    #[serde(rename = "activity_id")]
    pub activity_id: i32,
    /// blueprint_id integer
    #[serde(rename = "blueprint_id")]
    pub blueprint_id: i64,
    /// Location ID of the location from which the blueprint was installed. Normally a station ID, but can also be an asset (e.g. container) or corporation facility
    #[serde(rename = "blueprint_location_id")]
    pub blueprint_location_id: i64,
    /// blueprint_type_id integer
    #[serde(rename = "blueprint_type_id")]
    pub blueprint_type_id: i32,
    /// ID of the character which completed this job
    #[serde(
        rename = "completed_character_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_character_id: Option<i32>,
    /// Date and time when this job was completed
    #[serde(rename = "completed_date", skip_serializing_if = "Option::is_none")]
    pub completed_date: Option<String>,
    /// The sume of job installation fee and industry facility tax
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    /// Job duration in seconds
    #[serde(rename = "duration")]
    pub duration: i32,
    /// Date and time when this job finished
    #[serde(rename = "end_date")]
    pub end_date: String,
    /// ID of the facility where this job is running
    #[serde(rename = "facility_id")]
    pub facility_id: i64,
    /// ID of the character which installed this job
    #[serde(rename = "installer_id")]
    pub installer_id: i32,
    /// Unique job ID
    #[serde(rename = "job_id")]
    pub job_id: i32,
    /// Number of runs blueprint is licensed for
    #[serde(rename = "licensed_runs", skip_serializing_if = "Option::is_none")]
    pub licensed_runs: Option<i32>,
    /// ID of the location for the industry facility
    #[serde(rename = "location_id")]
    pub location_id: i64,
    /// Location ID of the location to which the output of the job will be delivered. Normally a station ID, but can also be a corporation facility
    #[serde(rename = "output_location_id")]
    pub output_location_id: i64,
    /// Date and time when this job was paused (i.e. time when the facility where this job was installed went offline)
    #[serde(rename = "pause_date", skip_serializing_if = "Option::is_none")]
    pub pause_date: Option<String>,
    /// Chance of success for invention
    #[serde(rename = "probability", skip_serializing_if = "Option::is_none")]
    pub probability: Option<f32>,
    /// Type ID of product (manufactured, copied or invented)
    #[serde(rename = "product_type_id", skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<i32>,
    /// Number of runs for a manufacturing job, or number of copies to make for a blueprint copy
    #[serde(rename = "runs")]
    pub runs: i32,
    /// Date and time when this job started
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// status string
    #[serde(rename = "status")]
    pub status: Status,
    /// Number of successful runs for this job. Equal to runs unless this is an invention job
    #[serde(rename = "successful_runs", skip_serializing_if = "Option::is_none")]
    pub successful_runs: Option<i32>,
}

impl GetCorporationsCorporationIdIndustryJobs200Ok {
    /// 200 ok object
    pub fn new(
        activity_id: i32,
        blueprint_id: i64,
        blueprint_location_id: i64,
        blueprint_type_id: i32,
        duration: i32,
        end_date: String,
        facility_id: i64,
        installer_id: i32,
        job_id: i32,
        location_id: i64,
        output_location_id: i64,
        runs: i32,
        start_date: String,
        status: Status,
    ) -> GetCorporationsCorporationIdIndustryJobs200Ok {
        GetCorporationsCorporationIdIndustryJobs200Ok {
            activity_id,
            blueprint_id,
            blueprint_location_id,
            blueprint_type_id,
            completed_character_id: None,
            completed_date: None,
            cost: None,
            duration,
            end_date,
            facility_id,
            installer_id,
            job_id,
            licensed_runs: None,
            location_id,
            output_location_id,
            pause_date: None,
            probability: None,
            product_type_id: None,
            runs,
            start_date,
            status,
            successful_runs: None,
        }
    }
}

/// status string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "reverted")]
    Reverted,
}
