; Nix syntax highlighting queries (adapted from Helix editor)
; Maps to standard tree-sitter capture names for compatibility with tree-sitter-highlight

(comment) @comment

; Punctuation
[
  ";"
  "."
  ","
  "="
  ":"
  (ellipses)
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Keywords
"assert" @keyword
"or" @keyword
"rec" @keyword

[
  "if" 
  "then"
  "else"
] @keyword

[
  "let"
  "inherit"
  "in"
  "with" 
] @keyword

; Variables and identifiers
(variable_expression name: (identifier) @variable)

; Attribute access (properties)
(select_expression
  attrpath: (attrpath attr: (identifier)) @property)

; Function calls
(apply_expression
  function: [
    (variable_expression name: (identifier) @function)
    (select_expression
      attrpath: (attrpath
        attr: (identifier) @function .))])

; Builtin variables
((identifier) @variable.builtin
 (#match? @variable.builtin "^(__currentSystem|__currentTime|__nixPath|__nixVersion|__storeDir|builtins)$")
 (#is-not? local))

; Builtin functions
((identifier) @function.builtin
 (#match? @function.builtin "^(__add|__addErrorContext|__all|__any|__appendContext|__attrNames|__attrValues|__bitAnd|__bitOr|__bitXor|__catAttrs|__compareVersions|__concatLists|__concatMap|__concatStringsSep|__deepSeq|__div|__elem|__elemAt|__fetchurl|__filter|__filterSource|__findFile|__foldl'|__fromJSON|__functionArgs|__genList|__genericClosure|__getAttr|__getContext|__getEnv|__hasAttr|__hasContext|__hashFile|__hashString|__head|__intersectAttrs|__isAttrs|__isBool|__isFloat|__isFunction|__isInt|__isList|__isPath|__isString|__langVersion|__length|__lessThan|__listToAttrs|__mapAttrs|__match|__mul|__parseDrvName|__partition|__path|__pathExists|__readDir|__readFile|__replaceStrings|__seq|__sort|__split|__splitVersion|__storePath|__stringLength|__sub|__substring|__tail|__toFile|__toJSON|__toPath|__toXML|__trace|__tryEval|__typeOf|__unsafeDiscardOutputDependency|__unsafeDiscardStringContext|__unsafeGetAttrPos|__valueSize|abort|baseNameOf|derivation|derivationStrict|dirOf|fetchGit|fetchMercurial|fetchTarball|fromTOML|import|isNull|map|placeholder|removeAttrs|scopedImport|throw|toString)$")
 (#is-not? local))

; Strings
[
  (string_expression)
  (indented_string_expression)
] @string

; Special string types (paths, URIs)
[
  (path_expression)
  (hpath_expression)
  (spath_expression)
] @string.special.path

(uri_expression) @string.special.uri

; Boolean constants
((identifier) @constant.builtin (#match? @constant.builtin "^(true|false)$"))

; Null constant
((identifier) @constant.builtin (#eq? @constant.builtin "null"))

; Numbers
(integer_expression) @number
(float_expression) @number

; Escape sequences
(escape_sequence) @escape
(dollar_escape) @escape

; Function parameters
(function_expression
  "@"? @punctuation.delimiter
  universal: (identifier) @variable.parameter
  "@"? @punctuation.delimiter
)

(formal
  name: (identifier) @variable.parameter
  "?"? @punctuation.delimiter)

; String interpolation
(interpolation
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

; Operators
(unary_expression
  operator: _ @operator)

(binary_expression
  operator: _ @operator)

; Attribute bindings
(binding
  attrpath: (attrpath attr: (identifier)) @property)

; Inherit expressions
(inherit_from attrs: (inherited_attrs attr: (identifier) @property))
(inherited_attrs attr: (identifier) @variable)

; Has attribute expression
(has_attr_expression
  expression: (_)
  "?" @operator
  attrpath: (attrpath
    attr: (identifier) @property))
