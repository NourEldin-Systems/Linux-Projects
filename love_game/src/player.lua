local utils = require("src.utils")
local player = {
  x = 0,
  y = 0,
  z = 0,
  speed = 100,
  height = 100,
  width = 50,
  movement = {
    isRight = false,
    isLeft = false,
    isTop = false,
    isBottom = false,
  },
  direction = { x = 0, y = 0, z = 0 }
}

function player.load()
  -- player.sprite = love.graphics.newImage("")
end

function player.update(dt)
  player.calculate_direction()

  player.move(dt)
end

function player.draw()
  -- Draw green player
  love.graphics.setColor(0.2, 0.8, 0.2)
  local drawPos = utils.world_to_isometric(player.x, player.y, player.z);
  love.graphics.rectangle("fill", drawPos.x, drawPos.y, player.width, player.height)

  -- Reset color to white
  love.graphics.setColor(1, 1, 1)
end

-- setting the player's direction to one of the values 1, 0 or -1 to use
-- in movement-related calculations like when calculating wheather a movement
-- should increase or decrease the x or the y axis
function player.calculate_direction()
  -- calculating horizantal direction
  local r = love.keyboard.isDown("d") and 1 or 0
  local l = love.keyboard.isDown("a") and 1 or 0
  local s = love.keyboard.isDown("s") and 1 or 0
  local w = love.keyboard.isDown("w") and 1 or 0
  local space = love.keyboard.isDown("space") and 1 or 0

  player.direction.x = r - l
  player.direction.y = s - w
  player.direction.z = space
end

function player.move(dt)
  if player.speed == 0 or player.speed == nil then
    return
  elseif player.direction == nil then
    return
  end

  local dx = player.direction.x
  local dy = player.direction.y
  local dz = player.direction.z

  if dx == 0 and dy == 0 then return end

  local length = math.sqrt(dx * dx + dy * dy)

  if length > 0 then
    dx = dx / length
    dy = dy / length
  end

  player.x = player.x + player.speed * dx * dt
  player.y = player.y + player.speed * dy * dt
end

return player
