-- set city for weather command

local phrase = babu.context.phrase
local lang = babu.context.language

-- try to extract city name from phrase
-- this is a simple example - you might want better parsing
local city = phrase:match("город%s+(.+)") or phrase:match("city%s+(.+)")

if city then
    city = city:gsub("^%s*(.-)%s*$", "%1") -- trim
    
    -- save to state (shared with weather command)
    babu.state.set("city", city)
    
    local msg = lang == "ru" 
        and "Город установлен: " .. city
        or "City set to: " .. city
    
    babu.log("info", msg)
    babu.system.notify("Babu", msg)
    babu.audio.play_ok()
else
    local msg = lang == "ru"
        and "Не удалось определить город"
        or "Could not determine city"
    
    babu.log("warn", msg)
    babu.audio.play_not_found()
end

return { chain = false }