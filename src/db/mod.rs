pub mod connection;
pub mod models;
pub mod schema;

use self::models::*;
use super::zoopla::responses::HouseResponse;
use diesel::prelude::*;
use diesel::insert_into;

pub fn create_property<'a>(r: &'a HouseResponse, conn: &PgConnection) {
    use db::schema::*;

    let house: House = insert_into(houses::table)
        .values((
            houses::price.eq(r.price as i32),
            houses::first_published_date.eq(&r.first_published_date),
            houses::last_published_date.eq(&r.last_published_date),
        ))
        .get_result(conn)
        .expect("inserting house");

    insert_into(houses_agents::table)
        .values((
            houses_agents::name.eq(&r.agent_name),
            houses_agents::phone.eq(&r.agent_phone),
            houses_agents::house_id.eq(house.id),
        ))
        .execute(conn)
        .expect("inserting agent");

    insert_into(houses_categories::table)
        .values((
            houses_categories::property_type.eq(&r.property_type),
            houses_categories::category.eq(&r.category),
            houses_categories::price_modifier.eq(&r.price_modifier),
        ))
        .execute(conn)
        .expect("inserting category");

    insert_into(houses_locations::table)
        .values((
            houses_locations::longitude.eq(r.longitude),
            houses_locations::latitude.eq(r.latitude),
            houses_locations::street_name.eq(&r.street_name),
            houses_locations::displayable_address.eq(&r.displayable_address),
            houses_locations::house_id.eq(house.id),
        ))
        .execute(conn)
        .expect("inserting location");

    insert_into(houses_properties::table)
        .values((
            houses_properties::description.eq(&r.description),
            houses_properties::num_bathrooms.eq(r.num_bathrooms as i32),
            houses_properties::num_bedrooms.eq(r.num_bedrooms as i32),
            houses_properties::num_floors.eq(r.num_floors as i32),
            houses_properties::house_id.eq(house.id),
        ))
        .execute(conn)
        .expect("inserting location");

    insert_into(houses_urls::table)
        .values((
            houses_urls::details.eq(&r.details_url),
            houses_urls::property_report.eq(&r.property_report_url),
            houses_urls::house_id.eq(house.id),
        ))
        .execute(conn)
        .expect("inserting location");
}
