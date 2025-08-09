use super::*;

impl FlightRequest {
    pub fn iata(code: impl ToString) -> Self {
        let code = FlightCode::Iata(code.to_string());
        Self { code }
    }

    pub fn icao(code: impl ToString) -> Self {
        let code = FlightCode::Icao(code.to_string());
        Self { code }
    }
}
