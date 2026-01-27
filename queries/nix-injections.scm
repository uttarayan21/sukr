; Nix injection queries for embedded language highlighting
; Adapted from Helix editor queries for tree-sitter-highlight compatibility
; Removed Helix-specific predicates: #is-not?, #any-of?, @injection.shebang, @injection.filename

((comment) @injection.content
 (#set! injection.language "comment"))

; mark arbitrary languages with a comment
((((comment) @injection.language) .
  (indented_string_expression (string_fragment) @injection.content))
  (#set! injection.combined))
((binding
    (comment) @injection.language
    expression: (indented_string_expression (string_fragment) @injection.content))
  (#set! injection.combined))

; Common attribute keys corresponding to Python scripts,
; such as those for NixOS VM tests in nixpkgs/nixos/tests.
((binding
   attrpath: (attrpath (identifier) @_path)
   expression: (indented_string_expression
     (string_fragment) @injection.content))
 (#match? @_path "(^|\\.)testScript$")
 (#set! injection.language "python")
 (#set! injection.combined))

; Common attribute keys corresponding to scripts,
; such as those of stdenv.mkDerivation.
((binding
   attrpath: (attrpath (identifier) @_path)
   expression: [
     (indented_string_expression (string_fragment) @injection.content)
     (binary_expression (indented_string_expression (string_fragment) @injection.content))
   ])
 (#match? @_path "(^\\w*Phase|command|(pre|post)\\w*|(.*\\.)?\\w*([sS]cript|[hH]ook)|(.*\\.)?startup)$")
 (#set! injection.language "bash")
 (#set! injection.combined))

; builtins.{match,split} regex str
((apply_expression
   function: (_) @_func
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)match|split$")
 (#set! injection.language "regex")
 (#set! injection.combined))

; builtins.fromJSON json
((apply_expression
   function: (_) @_func
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)fromJSON$")
 (#set! injection.language "json")
 (#set! injection.combined))

; builtins.fromTOML toml
((apply_expression
   function: (_) @_func
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)fromTOML$")
 (#set! injection.language "toml")
 (#set! injection.combined))

; trivial-builders.nix pkgs.writeShellScript[Bin] name content
((apply_expression
   function: (apply_expression function: (_) @_func)
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)writeShellScript(Bin)?$")
 (#set! injection.language "bash")
 (#set! injection.combined))

; trivial-builders.nix, aliases.nix
; pkgs.runCommand[[No]CC][Local] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)runCommand(((No)?(CC))?(Local)?)?$")
  (#set! injection.language "bash")
  (#set! injection.combined))

; trivial-builders.nix pkgs.writeShellApplication { text = content; }
(apply_expression
  function: ((_) @_func)
  argument: (_ (_)* (_ (_)* (binding
    attrpath: (attrpath (identifier) @_path)
     expression: (indented_string_expression
       (string_fragment) @injection.content))))
  (#match? @_func "(^|\\.)writeShellApplication$")
  (#match? @_path "^text$")
  (#set! injection.language "bash")
  (#set! injection.combined))

; trivial-builders.nix pkgs.writeCBin name content
((apply_expression
   function: (apply_expression function: (_) @_func)
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)writeC(Bin)?$")
 (#set! injection.language "c")
 (#set! injection.combined))

; pkgs.writers.write{Bash,Dash}[Bin] name content
((apply_expression
   function: (apply_expression function: (_) @_func)
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)write[BD]ash(Bin)?$")
 (#set! injection.language "bash")
 (#set! injection.combined))

; pkgs.writers.writeFish[Bin] name content
((apply_expression
   function: (apply_expression function: (_) @_func)
   argument: (indented_string_expression (string_fragment) @injection.content))
 (#match? @_func "(^|\\.)writeFish(Bin)?$")
 (#set! injection.language "fish")
 (#set! injection.combined))

; pkgs.writers.writeRust[Bin] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)writeRust(Bin)?$")
  (#set! injection.language "rust")
  (#set! injection.combined))

; pkgs.writers.writeHaskell[Bin] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)writeHaskell(Bin)?$")
  (#set! injection.language "haskell")
  (#set! injection.combined))

; pkgs.writers.writeJS[Bin] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)writeJS(Bin)?$")
  (#set! injection.language "javascript")
  (#set! injection.combined))

; pkgs.writers.write{Python,PyPy}{2,3}[Bin] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)write(Python|PyPy)[23](Bin)?$")
  (#set! injection.language "python")
  (#set! injection.combined))

; pkgs.writers.writeRuby[Bin] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)writeRuby(Bin)?$")
  (#set! injection.language "ruby")
  (#set! injection.combined))

; pkgs.writers.writeLua[Bin] name attrs content
(apply_expression
  (apply_expression
    function: (apply_expression
      function: ((_) @_func)))
    argument: (indented_string_expression (string_fragment) @injection.content)
  (#match? @_func "(^|\\.)writeLua(Bin)?$")
  (#set! injection.language "lua")
  (#set! injection.combined))

; string contents of lib.literalExpression is nix code
((apply_expression
    function: [
      (select_expression)
      (variable_expression)
    ] @_func
    argument: [
      (indented_string_expression (string_fragment) @injection.content)
      (string_expression (string_fragment) @injection.content)
    ])
  (#match? @_func "(lib\\.)?literalExpression$")
  (#set! injection.language "nix")
  (#set! injection.combined))
