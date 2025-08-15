-- Resolve relative image paths against the source file's directory

local path = pandoc.path
local pipe = pandoc.pipe

local function file_exists(p)
  local f = io.open(p, 'rb')
  if f then f:close() return true end
  return false
end

local function resolve_relative(src)
  -- leave remote URLs untouched
  if src:match('^https?://') then
    return src
  end
  -- leave data URIs untouched
  if src:match('^data:') then
    return src
  end
  local inputs = PANDOC_STATE and PANDOC_STATE.input_files or {}
  local input = inputs and inputs[1] or nil
  if not input or input == '' then
    return src
  end
  local base_dir = path.directory(input)
  local candidate = path.normalize(path.join({ base_dir, src }))
  if file_exists(candidate) then
    return candidate
  end
  -- Try hyphen/underscore variants in filename
  local dir = path.directory(candidate)
  local fname = path.filename(candidate)
  if fname then
    local alt1 = fname:gsub('%-', '_')
    if alt1 ~= fname then
      local c1 = path.normalize(path.join({ dir, alt1 }))
      if file_exists(c1) then return c1 end
    end
    local alt2 = fname:gsub('%_', '-')
    if alt2 ~= fname then
      local c2 = path.normalize(path.join({ dir, alt2 }))
      if file_exists(c2) then return c2 end
    end
  end
  -- Fallback: search in repository 'chapters/**/media' for the filename
  if fname and fname ~= '' then
    local cmd = string.format("find chapters -type f -name %q -print -quit", fname)
    local ok, out = pcall(pipe, 'sh', {'-lc', cmd}, '')
    if ok and out and out ~= '' then
      local found = out:gsub('%s+$','')
      local book_path = path.normalize(path.join({'book', found}))
      if file_exists(book_path) then
        return book_path
      end
      if file_exists(found) then
        return found
      end
    else
      -- Try hyphen/underscore variant in search
      local alt = fname:gsub('%-', '_')
      if alt ~= fname then
        local cmd2 = string.format("find chapters -type f -name %q -print -quit", alt)
        local ok2, out2 = pcall(pipe, 'sh', {'-lc', cmd2}, '')
        if ok2 and out2 and out2 ~= '' then
          local found2 = out2:gsub('%s+$','')
          local book_path2 = path.normalize(path.join({'book', found2}))
          if file_exists(book_path2) then return book_path2 end
          if file_exists(found2) then return found2 end
        end
      end
    end
  end
  return src
end

function Image(el)
  el.src = resolve_relative(el.src)
  return el
end


