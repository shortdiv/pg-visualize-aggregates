use serde::Serialize;

#[derive(Serialize)]
pub struct Climbs {
    pub id: i32,
    pub route_name: String,
    pub parent_sector: String,
    pub sector_id: i32,
    pub route_id: i32,
    pub type_string: String,
    pub fa: String,
    pub yds: String,
    pub vermin: String,
    pub nopm_yds: String,
    pub nopm_vermin: String,
    pub yds_rank: String,
    pub vermin_rank: String,
    pub safety: String,
    pub parent_loc: String,
    pub description: String,
    pub location: String,
    pub protection: String,
    pub corrected_users_ratings: String,
}
