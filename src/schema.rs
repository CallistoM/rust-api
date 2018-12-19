table! {
    olc_external {
        id -> Nullable<Integer>,
        value -> Integer,
        date -> Nullable<Datetime>,
    }
}

table! {
    olc_internal {
        id -> Nullable<Integer>,
        value -> Integer,
        date -> Nullable<Datetime>,
    }
}

table! {
    olc_gps {
        id -> Nullable<Integer>,
        latitude -> Integer,
        longitude -> Integer,
        altitude -> Integer,
        accuracy -> Integer,
        satellites -> Integer,
    }
}