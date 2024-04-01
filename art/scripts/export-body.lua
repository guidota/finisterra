dofile('find-pixel.lua')
dofile('export-metadata.lua')
local err = dofile("fs.lua")
if err ~= 0 then return err end

local body_offsets_group = spr.layers['body_offsets']
if (body_offsets_group == nil) then return end
if (not body_offsets_group.isGroup) then return end

local skins_group = spr.layers["skins"]
if (skins_group == nil) then return end
if (not skins_group.isGroup) then return end

local function extract_body(frame) 
  local base = find_pixel(body_offsets_group.layers, 'base', frame)
  local head = find_pixel(body_offsets_group.layers, 'head', frame)
  local left_hand = find_pixel(body_offsets_group.layers, 'left_hand', frame)
  local right_hand = find_pixel(body_offsets_group.layers, 'right_hand', frame)
  local left_foot = find_pixel(body_offsets_group.layers, 'left_foot', frame)
  local right_foot = find_pixel(body_offsets_group.layers, 'right_foot', frame)
  local frame = string.format(
    '(%s, %s, %s, %s, %s, %s),', 
    base, head, left_hand, right_hand, left_foot, right_foot
  )
  return frame
end

local function extract_skin(frame) 
  local priority = 10
  local offset = find_pixel_unwrapped(spr.layers['body_offsets'].layers, 'base', frame)
  local frame = string.format('(image: %d, priority: %d, offset: %s),', frame, priority, offset)

  return frame
end

local output = app.params["output"]
if not (output == nil) then
  local directory = Dirname(output)
  os.execute("mkdir \"" .. directory .. "\"")
  local metadata_file = directory .. Sep .. 'body.ron' 
  export_metadata(metadata_file, extract_body)

  local metadata_file = directory .. Sep .. 'skin.ron' 
  export_metadata(metadata_file, extract_skin)

  local layers_visibility_data = HideLayers(spr)
  local images_directory = directory .. Sep .. 'images'
  export_images(spr.layers["skins"], images_directory)
  RestoreLayersVisibility(spr, layers_visibility_data)

  return 0
end
