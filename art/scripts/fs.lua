function Dirname(str)
   return str:match("(.*" .. Sep .. ")")
end

-- Hides all layers and sub-layers inside a group, returning a list with all
-- initial states of each layer's visibility.
function HideLayers(sprite)
   local data = {} -- Save visibility status of each layer here.
   for i,layer in ipairs(sprite.layers) do
      if layer.isGroup then
         -- Recursive for groups.
         data[i] = HideLayers(layer)
      else
         data[i] = layer.isVisible
         layer.isVisible = false
      end
   end
   return data
end

-- Restore layers visibility.
function RestoreLayersVisibility(sprite, data)
   for i,layer in ipairs(sprite.layers) do
      if layer.isGroup then
         -- Recursive for groups.
         RestoreLayersVisibility(layer, data[i])
      else
         layer.isVisible = data[i]
      end
   end
end

-- Dialog
function MsgDialog(title, msg)
   local dlg = Dialog(title)
   dlg:label{
      id = "msg",
      text = msg
   }
   dlg:newrow()
   dlg:button{id = "close", text = "Close", onclick = function() dlg:close() end }
   return dlg
end

function export_images(group, directory, type) 
  for _, item in ipairs(group.layers) do
    local filename = directory.. Sep .. item.name .. '.png'
    item.isVisible = true
    local layer = item
    if (item.isGroup) then
      for _, layer in ipairs(item.layers) do
        layer.isVisible = true
      end
    end
     
    app.command.ExportSpriteSheet {
      ui=false,
      askOverwrite=false,
      type=type or SpriteSheetType.ROWS,
      columns=0,
      rows=0,
      width=0,
      height=0,
      bestFit=false,
      textureFilename=filename,
      dataFilename="",
      dataFormat=SpriteSheetDataFormat.JSON_HASH,
      filenameFormat="{layer}.{extension}",
      borderPadding=0,
      shapePadding=0,
      innerPadding=0,
      trimspr=false,
      trim=false,
      trimByGrid=false,
      extrude=false,
      ignoreEmpty=false,
      mergeDuplicates=false,
      openGenerated=false,
      layer="",
      tag="",
      splitLayers=false,
      splitTags=true,
      splitGrid=false,
      listLayers=false,
      listTags=false,
      listSlices=false,
      fromTilesets=false,
    }
    layer.isVisible = false
    if (item.isGroup) then
      for _, layer in ipairs(item.layers) do
        layer.isVisible = false 
      end
    end
  end
end


local filename = app.params["filename"]
if filename ~= nil then
   spr = app.open(filename)
else 
   spr = app.activeSprite
end

if spr == nil then
   -- Show error, no sprite active.
   local dlg = MsgDialog("Error", "No sprite is currently active. Please, open a sprite first and run again.")
   dlg:show()
   return 1
end

-- Identify operative system.
Sep = string.sub(spr.filename, 1, 1) == "/" and "/" or "\\"

if Dirname(spr.filename) == nil then
   -- Error, can't identify OS when the sprite isn't saved somewhere.
   local dlg = MsgDialog("Error", "Current sprite is not associated to a file. Please, save your sprite and run again.")
   dlg:show()
   return 1
end

return 0
