function find_pixel(layers, layer_name, frame)
  local empty = string.format('%s: (x: 0, y: 0)', layer_name)

  local layer = layers[layer_name]
  if (layer == nil) then return empty end

  local cel = layer:cel(frame)
  if (cel == nil) then return empty end

  local cel_x = cel.bounds.x
  local cel_y = cel.bounds.y

  local image = cel.image
  if (image == nil) then return empty end

  local sprite = app.activeSprite

  for it in image:pixels() do
    local x = cel_x + it.x
    local y = sprite.height - (cel_y + it.y)
    return string.format('%s: (x: %d, y: %d)', layer_name, x, y)
  end
  return empty
end

function find_pixel_unwrapped(layers, layer_name, frame)
  local empty = '(x: 0, y: 0)'

  local layer = layers[layer_name]
  if (layer == nil) then return empty end

  local cel = layer:cel(frame)
  if (cel == nil) then return empty end

  local cel_x = cel.bounds.x
  local cel_y = cel.bounds.y

  local image = cel.image
  if (image == nil) then return empty end

  local sprite = app.activeSprite

  for it in image:pixels() do
    local x = cel_x + it.x
    local y = sprite.height - (cel_y + it.y)
    return string.format('(x: %d, y: %d)', x, y)
  end
  return empty
end

function find_priority(layers, layer_name, frame)
  local layer = layers[layer_name]
  if (layer == nil) then return 0 end

  local cel = layer:cel(frame)
  if (cel == nil) then return 0 end
  return cel.zIndex 
end
