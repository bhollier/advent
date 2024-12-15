use phf::phf_map;

mod day1;
mod day2;
mod day3;
mod day4;

pub static DAYS: phf::Map<u8, fn(&str)> = phf_map! {
    1_u8 => day1::run,
    2_u8 => day2::run,
    3_u8 => day3::run,
    4_u8 => day4::run,
};
