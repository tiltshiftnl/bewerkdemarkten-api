table! {
    afwijzing (id) {
        id -> Int4,
        marktId -> Nullable<Int4>,
        marktDate -> Nullable<Date>,
        reasonCode -> Nullable<Int4>,
        erkenningsNummer -> Nullable<Varchar>,
        plaatsvoorkeuren -> Nullable<Array<Text>>,
        anywhere -> Nullable<Bool>,
        minimum -> Nullable<Int4>,
        maximum -> Nullable<Int4>,
        bak -> Nullable<Bool>,
        eigenMaterieel -> Nullable<Bool>,
        brancheId -> Nullable<Int4>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    allocation (id) {
        id -> Int4,
        marktId -> Nullable<Int4>,
        marktDate -> Nullable<Date>,
        plaatsId -> Nullable<Varchar>,
        erkenningsNummer -> Nullable<Varchar>,
        plaatsvoorkeuren -> Nullable<Array<Text>>,
        anywhere -> Nullable<Bool>,
        minimum -> Nullable<Int4>,
        maximum -> Nullable<Int4>,
        bak -> Nullable<Bool>,
        eigenMaterieel -> Nullable<Bool>,
        brancheId -> Nullable<Int4>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    log (id) {
        id -> Int4,
        ts -> Timestamptz,
        level -> Varchar,
        msg -> Text,
        meta -> Nullable<Json>,
    }
}

table! {
    markets (id) {
        id -> Int4,
        name -> Varchar,
        abbreviation -> Varchar,
        //createdAt -> Timestamptz,
        //updatedAt -> Timestamptz,
    }
}

table! {
    plaatsvoorkeur (id) {
        id -> Int4,
        marktId -> Nullable<Int4>,
        erkenningsNummer -> Nullable<Varchar>,
        plaatsId -> Nullable<Varchar>,
        priority -> Nullable<Int4>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    rsvp (id) {
        id -> Int4,
        marktId -> Nullable<Int4>,
        marktDate -> Nullable<Date>,
        erkenningsNummer -> Nullable<Varchar>,
        attending -> Nullable<Bool>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    session (sid) {
        sid -> Varchar,
        sess -> Nullable<Json>,
        expire -> Nullable<Timestamptz>,
    }
}

table! {
    voorkeur (id) {
        id -> Int4,
        erkenningsNummer -> Varchar,
        marktId -> Nullable<Int4>,
        marktDate -> Nullable<Date>,
        anywhere -> Nullable<Bool>,
        minimum -> Nullable<Int4>,
        maximum -> Nullable<Int4>,
        brancheId -> Nullable<Varchar>,
        parentBrancheId -> Nullable<Varchar>,
        inrichting -> Nullable<Varchar>,
        absentFrom -> Nullable<Date>,
        absentUntil -> Nullable<Date>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    afwijzing,
    allocation,
    log,
    markets,
    plaatsvoorkeur,
    rsvp,
    session,
    voorkeur,
);
