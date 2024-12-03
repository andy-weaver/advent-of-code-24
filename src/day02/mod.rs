pub mod count_safe_reports;
pub mod is_safe;
pub mod has_at_most_one_bad_level;

pub use count_safe_reports::count_safe_reports;
pub use is_safe::is_safe;
pub use has_at_most_one_bad_level::has_at_most_one_bad_level;

pub const INPUT_FILENAME: &str = "inputs/day02.input";
