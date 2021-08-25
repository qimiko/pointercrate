use crate::endpoints::misc;
use pointercrate_core::permission::Permission;
use rocket::{Build, Rocket};

mod endpoints;

pub const LIST_HELPER: Permission = Permission::new("List Helper", 0x2);
pub const LIST_MODERATOR: Permission = Permission::new("List Moderator", 0x4);
pub const LIST_ADMINISTRATOR: Permission = Permission::new("List Administrator", 0x8);

pub fn setup(mut rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/v1/list_information/", rocket::routes![misc::list_information])
        .mount("/api/v1/submitters/", rocket::routes![
            endpoints::submitter::paginate,
            endpoints::submitter::get,
            endpoints::submitter::paginate
        ])
}