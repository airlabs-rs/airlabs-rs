use super::*;

impl FlightRequest {
    pub fn iata(token: impl ToString, code: impl ToString) -> Self {
        let api_key = token.to_string();
        let code = FlightCode::Iata(code.to_string());
        Self { api_key, code }
    }

    pub fn icao(token: impl ToString, code: impl ToString) -> Self {
        let api_key = token.to_string();
        let code = FlightCode::Icao(code.to_string());
        Self { api_key, code }
    }
}
