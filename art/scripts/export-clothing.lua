dofile('find-pixel.lua')
dofile('export-metadata.lua')
local err = dofile("fs.lua")
if err ~= 0 then return err end

local type = 'clothing'
local function extract_metadata(frame) 
  local image = frame
  local priority = 10
  local offset = find_pixel_unwrapped(spr.layers['body_offsets'].layers, 'base', frame)
  local frame = string.format('(image: %d, priority: %d, offset: %s),', image, priority, offset)

  return frame
end

local output = app.params["output"]
if not (output == nil) then
  local directory = Dirname(output)
  os.execute("mkdir \"" .. directory .. "\"")
  local metadata_file = directory .. Sep .. 'cloth.ron' 
  export_metadata(metadata_file, extract_metadata)

  local layers_visibility_data = HideLayers(spr)
  export_images(spr.layers[type], directory .. Sep .. 'images')
  RestoreLayersVisibility(spr, layers_visibility_data)
end

return 0
