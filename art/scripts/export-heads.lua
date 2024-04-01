dofile('find-pixel.lua')
dofile('export-metadata.lua')
local err = dofile("fs.lua")
if err ~= 0 then return err end

local function extract_head(frame) 
  local image = frame
  local hair_priority = 10 + find_priority(spr.layers["hairs"].layers, "hair_1", frame)
  local eyes_priority = 10 + find_priority(spr.layers["eyes"].layers, "eyes_1", frame)
  local face_priority = 10 + find_priority(spr.layers["faces"].layers, "face_1", frame)
  local offset = find_pixel(spr.layers, 'offset', frame)
  
  local frame = string.format('(image: %d, hair_priority: %d, eyes_priority: %d, face_priority: %d, %s),', image, hair_priority, eyes_priority, face_priority, offset)

  return frame
end

local output = app.params["output"]
if not (output == nil) then
  local directory = Dirname(output)
  os.execute("mkdir \"" .. directory .. "\"")
  local metadata_file = directory .. Sep .. 'head.ron' 
  export_metadata(metadata_file, extract_head)

  local layers_visibility_data = HideLayers(spr)
  local faces_directory = directory .. Sep .. 'faces'
  export_images(spr.layers["faces"], faces_directory, SpriteSheetType.COLUMNS)
  local eyes_directory = directory .. Sep .. 'eyes'
  export_images(spr.layers["eyes"], eyes_directory, SpriteSheetType.COLUMNS)
  local hairs_directory = directory .. Sep .. 'hairs'
  export_images(spr.layers["hairs"], hairs_directory, SpriteSheetType.COLUMNS)
  RestoreLayersVisibility(spr, layers_visibility_data)

  return 0
end
