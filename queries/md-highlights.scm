; Minimal markdown block highlighting - only capture outside code blocks
; Removed all captures that could match inside code_fence_content

(atx_heading (inline) @text.title)
(setext_heading (paragraph) @text.title)

[
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
  (setext_h1_underline)
  (setext_h2_underline)
] @punctuation.special

(fenced_code_block_delimiter) @punctuation.delimiter
(info_string (language) @string)

(link_title) @string
(link_destination) @text.uri
(link_label) @text.reference

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
  (thematic_break)
] @punctuation.special

(block_quote_marker) @punctuation.special
(backslash_escape) @string.escape
