local utils = {}

function utils.world_to_isometric(x, y, z)
  local iso_x = x - y
  local iso_y = (x + y) / 2 - z
  return { x = iso_x, y = iso_y }
end

return utils
