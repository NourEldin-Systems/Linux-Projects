local player = require("src.player")
local sti = require("sti")
local map
function love.load()
  love.graphics.setDefaultFilter("nearest", "nearest")

  map = sti("src/assets/maps/map.lua")
  player.load()
end

function love.update(dt)
  player.update(dt)

  -- Update map layers and animations (if any)
  map:update(dt)
end

function love.draw()
  map:draw()
  player.draw()
end
