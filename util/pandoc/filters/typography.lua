-- Replace some problematic Unicode characters for LaTeX engines

function walk_inlines(inlines)
  for i = 1, #inlines do
    local el = inlines[i]
    if el.t == 'Str' then
      local s = el.text
      s = s:gsub('≥', '\\ensuremath{\\geq}')
      s = s:gsub('≤', '\\ensuremath{\\leq}')
      s = s:gsub('→', '->')
      s = s:gsub('👷‍♂️', 'worker')
      s = s:gsub('👷', 'worker')
      s = s:gsub('❯', '>')
      inlines[i] = pandoc.Str(s)
    end
  end
  return inlines
end

return {
  { Inlines = walk_inlines }
}


