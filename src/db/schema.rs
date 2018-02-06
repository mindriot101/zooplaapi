table! {
    houses (id) {
        id -> Int4,
        price -> Int4,
        first_published_date -> Varchar,
        last_published_date -> Varchar,
        listing_id -> Int4,
    }
}

table! {
    houses_agents (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
        house_id -> Nullable<Int4>,
    }
}

table! {
    houses_categories (id) {
        id -> Int4,
        property_type -> Varchar,
        category -> Varchar,
        price_modifier -> Nullable<Varchar>,
        house_id -> Nullable<Int4>,
    }
}

table! {
    houses_locations (id) {
        id -> Int4,
        longitude -> Float4,
        latitude -> Float4,
        street_name -> Varchar,
        displayable_address -> Varchar,
        house_id -> Nullable<Int4>,
    }
}

table! {
    houses_properties (id) {
        id -> Int4,
        description -> Text,
        num_bathrooms -> Int4,
        num_bedrooms -> Int4,
        num_floors -> Int4,
        house_id -> Nullable<Int4>,
    }
}

table! {
    houses_urls (id) {
        id -> Int4,
        details -> Varchar,
        property_report -> Varchar,
        house_id -> Nullable<Int4>,
    }
}

joinable!(houses_agents -> houses (house_id));
joinable!(houses_categories -> houses (house_id));
joinable!(houses_locations -> houses (house_id));
joinable!(houses_properties -> houses (house_id));
joinable!(houses_urls -> houses (house_id));

allow_tables_to_appear_in_same_query!(
    houses,
    houses_agents,
    houses_categories,
    houses_locations,
    houses_properties,
    houses_urls,
);
