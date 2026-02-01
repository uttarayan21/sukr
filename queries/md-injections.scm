; Markdown injection queries with include-children directive
; Enables proper highlighting of embedded languages in code blocks and frontmatter

; Fenced code blocks - inject language specified in info string
(fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content
  (#set! injection.include-children))

; YAML frontmatter (--- delimited at start of document)
((minus_metadata) @injection.content
  (#set! injection.language "yaml")
  (#set! injection.include-children))

; TOML frontmatter (+++ delimited)
((plus_metadata) @injection.content
  (#set! injection.language "toml")
  (#set! injection.include-children))

; HTML blocks
((html_block) @injection.content
  (#set! injection.language "html")
  (#set! injection.include-children))
