
function export_metadata(output, extract_function)
  local identation = '  '
  local f = io.open(output, 'w')
  f:write(string.format('(\n')) -- start

  animations = {}
  -- for each sprite tag
  for i,tag in ipairs(spr.tags) do
    animation, direction = tag.name:match('(%w+)_(%w+)')

    if (animations[animation] == nil) then
      animations[animation] = {}
    end

    if (animations[animation][direction] == nil) then
      animations[animation][direction] = {}
    end


    local animation_frames = animations[animation][direction]
    local frames = tag.frames - 1
    
    for i=0,frames,1 do
      local frame = tag.fromFrame.frameNumber + i
      local frame_metadata = extract_function(frame)

      animation_frames[i + 1] = frame_metadata
    end
  end 

  for animation,dir_animation in pairs(animations) do
    f:write(identation .. animation .. string.format(': (\n')) -- start animation

    for direction,frames in pairs(dir_animation) do
      f:write(identation .. identation .. direction .. ': (\n') -- start direction
      f:write(identation .. identation .. identation .. 'frames: [\n') -- start frames list

      for i,frame in ipairs(frames) do
        f:write(identation .. identation .. identation .. identation .. frame .. '\n') -- append frame
      end
      
      f:write(identation .. identation .. identation .. '],\n') -- end frames
      f:write(identation .. identation .. '),\n') -- end direction
    end
    f:write(identation .. '),\n') -- end animation
  end
    
  f:write(string.format(')\n')) -- end
  f:close()

  return 0
end

