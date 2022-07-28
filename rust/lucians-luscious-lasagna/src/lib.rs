// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const EXPECT_MINUTES: i32 = 40;
const LAYER_MINUTES: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    EXPECT_MINUTES
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    EXPECT_MINUTES - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * LAYER_MINUTES
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
