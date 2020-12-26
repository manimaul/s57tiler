table! {
    charts (id) {
        id -> Int8,
        name -> Text,
        scale -> Int4,
        file_name -> Text,
        updated -> Text,
        issued -> Text,
    }
}

table! {
    use crate::sql_types::*;
    use diesel::sql_types::*;

    features (id) {
        id -> Int8,
        layer -> Varchar,
        geom -> Geometry,
        props -> Jsonb,
        chart_id -> Int8,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    styles (id) {
        id -> Int8,
        name -> Text,
        style -> Jsonb,
    }
}

joinable!(features -> charts (chart_id));

allow_tables_to_appear_in_same_query!(
    charts,
    features,
    spatial_ref_sys,
    styles,
);
