# This "hook" is executed right before the site's pages are rendered
Jekyll::Hooks.register :site, :pre_render do |site|
  puts "Registering TED lexer..."
  require "rouge"

  # This class defines the ALG lexer which is used to highlight "alg" code snippets during render-time
  class TEDLexer < Rouge::RegexLexer
    title 'TED'
    desc 'Ted'
    tag 'ted'
    filenames '*.ted'

    state :root do
      rule %r/(true|false)\b/, Name::Constant
      rule %r/#.*$/, Comment::Single
      rule %r/".*?"/m, Literal::String
      rule %r/'.*?'/m, Literal::String
      rule %r/->\s*[0-9A-Za-z_]+/, Keyword::Reserved
      rule %r/<-\s*[0-9A-Za-z_]+/, Name::Function::Magic
      rule %r/\d+/, Literal::Number

      rule %r/./, Name
    end
  end
end
