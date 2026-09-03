// get_weather.rhai

fn endpoint_name() {
    "get-weather"
}

fn http_method() {
    "POST"
}

fn input_schema() {
    #{
        type: "object",
        required: [
            "latitude",
            "longitude",
            "start_time_ms",
            "end_time_ms"
        ],
        properties: #{
            latitude: #{
                type: "number",
                format: "double",
                minimum: -90.0,
                maximum: 90.0,
                description: "Latitude in degrees"
            },
            longitude: #{
                type: "number",
                format: "double",
                minimum: -180.0,
                maximum: 180.0,
                description: "Longitude in degrees"
            },
            start_time_ms: #{
                type: "integer",
                format: "int64",
                description: "Start of time range (milliseconds since GMT Unix Epoch)"
            },
            end_time_ms: #{
                type: "integer",
                format: "int64",
                description: "End of time range (milliseconds since GMT Unix Epoch)"
            }
        }
    }
}

fn output_schema() {
    #{
        type: "array",
        description: "Array of daily bucket bins spanning the requested time span",
        items: #{
            type: "object",
            required: [
                "barometer",
                "precipitation_chance",
                "temperature",
                "heat_index"
            ],
            properties: #{
                barometer: #{
                    type: "number",
                    format: "float",
                    description: "Barometric pressure reading"
                },
                precipitation_chance: #{
                    type: "number",
                    format: "float",
                    minimum: 0.0,
                    maximum: 100.0,
                    description: "Precipitation probability percentage (0–100%)"
                },
                temperature: #{
                    type: "number",
                    format: "float",
                    description: "Ambient temperature reading"
                },
                heat_index: #{
                    type: "number",
                    format: "float",
                    description: "Calculated heat index temperature"
                }
            }
        }
    }
}

// In Rhai, input is an object map passed dynamically by the caller
fn handle(input) {
    let latitude = input.latitude;
    let longitude = input.longitude;
    let start_time_ms = input.start_time_ms;
    let end_time_ms = input.end_time_ms;

    get_weather(latitude, longitude, start_time_ms, end_time_ms)
}

// This is a placeholder for the actual weather data retrieval logic.
fn get_weather(latitude, longitude, start_time_ms, end_time_ms) {
    [#{
        barometer: 1013.25,
        precipitation_chance: 0.0,
        temperature: 22.0,
        heat_index: 22.0
    }]
}