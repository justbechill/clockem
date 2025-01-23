use weer_api::Realtime;

pub fn parse_weather_string(d: &Realtime, s: &str, daynight_strings: &Vec<String>) -> String {
    let mut f = s.to_string();

    for (i, c) in s.chars().enumerate() {
        if c == '%' {
            let modifier = c.to_string() + s.chars().nth(i + 1).unwrap_or('Z').to_string().as_str();

            let replacement = match modifier.as_str() {
                "%L" => &d.location.name,
                "%l" => &d.location.country,
                "%R" => &d.location.region,
                "%S" => &d.current.condition.text,
                "%F" => &d.current.temp_f.to_string(),
                "%f" => &d.current.feelslike_f.to_string(),
                "%C" => &d.current.temp_c.to_string(),
                "%c" => &d.current.feelslike_c.to_string(),
                "%K" => &(&d.current.temp_c + 273.15).to_string(),
                "%k" => &(&d.current.feelslike_c + 273.15).to_string(),
                "%W" => &d.current.wind_kph.to_string(),
                "%w" => &d.current.wind_mph.to_string(),
                "%D" => &d.current.wind_dir,
                "%d" => &d.current.wind_degree.to_string(),
                "%P" => &d.current.precip_mm.to_string(),
                "%p" => &d.current.precip_in.to_string(),
                "%H" => &d.current.humidity.to_string(),
                "%U" => &d.current.uv.to_string(),
                "%T" => match d.current.is_day() {
                    true => &daynight_strings[0],
                    false => &daynight_strings[1],
                },
                _ => &modifier,
            };

            f = f.replace(&modifier, &replacement);
        }
    }

    f
}
