use phf::phf_map;

mod day1;
mod day2;
mod day3;

pub static DAYS: phf::Map<u8, fn(&String)> = phf_map! {
    1_u8 => day1::run,
    2_u8 => day2::run,
    3_u8 => day3::run,
};
