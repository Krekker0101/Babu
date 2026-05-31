-- weather command using wttr.in API

local lang = babu.context.language

-- get saved city or use default
local city = babu.state.get("city") or "Moscow"

babu.log("info", "Fetching weather for: " .. city)

-- build URL
local url = "https://wttr.in/" .. city .. "?format=3&lang=" .. lang

-- make request
local response = babu.http.get(url)

if response.ok then
    babu.log("info", "Weather: " .. response.body)
    
    -- show notification
    local title = lang == "ru" and "Погода" or "Weather"
    babu.system.notify(title, response.body)
    
    babu.audio.play_ok()
else
    babu.log("error", "Failed to fetch weather: " .. (response.error or "unknown error"))
    babu.audio.play_error()
end

return { chain = false }