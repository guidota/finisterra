dofile('find-pixel.lua')
dofile('export-metadata.lua')
local err = dofile("fs.lua")
if err ~= 0 then return err end

function do_export(type, layout) 
  local function extract_metadata(frame) 
    local layers = type .. 's'
    local priority = 10 + find_priority(spr.layers[layers].layers, type .. '_1', frame)
    local offset = find_pixel(spr.layers, 'offset', frame)
    local frame = string.format('(image: %d, priority: %d, %s),', frame, priority, offset)

    return frame
  end

  local output = app.params["output"]
  if not (output == nil) then
    local directory = Dirname(output)
    os.execute("mkdir \"" .. directory .. "\"")
    local metadata_file = directory .. Sep .. type .. '.ron' 
    export_metadata(metadata_file, extract_metadata)

    local layers_visibility_data = HideLayers(spr)
    export_images(spr.layers[type .. 's'], directory .. Sep .. 'images', layout)
    RestoreLayersVisibility(spr, layers_visibility_data)
  end

  return 0
end


