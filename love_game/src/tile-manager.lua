require("src/utils")

local tile_manager = {
  map = {
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
  },
  tile_size = 32,
  tileset = {}
}

function tile_manager.load_tileset()
  table.insert(tile_manager.tileset, love.graphics.newImage("src/assets/tile.png"))
end

function tile_manager.draw_map()
  for y = 1, #tile_manager.map do
    for x = 1, #tile_manager.map[y] do
      local tile = tile_manager.map[y][x]

      if tile > 0 and tile_manager.tileset[tile] then
        local dx = tile_manager.tile_size * (x - 1)
        local dy = tile_manager.tile_size * (y - 1)

        local iso_pos = world_to_isometric(dx, dy)

        love.graphics.draw(
          tile_manager.tileset[tile],
          iso_pos.x,
          iso_pos.y,
          0, 2, 2
        )
      end
    end
  end
end

return tile_manager
