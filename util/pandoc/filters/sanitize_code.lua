-- Replace problematic Unicode in Code and CodeBlock to avoid LaTeX font issues

local replacements = {
  ['≥'] = '\\ensuremath{\\geq}',
  ['≤'] = '\\ensuremath{\\leq}',
  ['→'] = '->',
  ['👷‍♂️'] = 'worker',
  ['👷'] = 'worker',
  ['❯'] = '>'
}

local function sanitize_text(s)
  -- remove zero-width joiner U+200D
  s = s:gsub('\226\128\141', '')
  for k, v in pairs(replacements) do
    s = s:gsub(k, v)
  end
  return s
end

function Code(el)
  el.text = sanitize_text(el.text)
  return el
end

function CodeBlock(el)
  el.text = sanitize_text(el.text)
  return el
end


