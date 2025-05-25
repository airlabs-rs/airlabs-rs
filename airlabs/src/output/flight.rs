use super::*;

impl Output for api::Flight {
    fn output(&self) -> String {
        format!("{self:#?}")
    }
}

impl Output for api::FlightFree {
    fn output(&self) -> String {
        format!("{self:#?}")
    }
}
