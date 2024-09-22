use crate::task::TASK_PRIORITIES;

pub struct Util;

impl Util {
    pub fn get_spaced_title(title: &str) -> String {
        format!(" {} ", title)
    }

    pub fn get_priority_indicator(value: u8) -> String {
        // Priority value is in ascending order
        // but in the visualization the order is reversed to be more intuitive
        // priority: 1 => !!!
        // priority: 2 => !!
        // priority: 3 => !
        let priority_value = if value == 0 {
            0
        } else {
            TASK_PRIORITIES
                .into_iter()
                .rev()
                .position(|t| t == value)
                .unwrap_or(0)
        };

        "!!!".chars().take((priority_value).into()).collect()
    }
}
