-- simple counter demonstrating state persistence

local count = babu.state.get("count") or 0
count = count + 1
babu.state.set("count", count)

local lang = babu.context.language
local msg = lang == "ru"
    and "Счётчик: " .. count
    or "Counter: " .. count

babu.log("info", msg)
babu.system.notify("Counter", tostring(count))
babu.audio.play_ok()

return { chain = true }