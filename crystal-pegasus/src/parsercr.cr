require "./parser.cr"

enum TerminalIds
  # in the grammar, whitespace has [skip] so this token will never appear
  Whitespace
  Number
  Add
  Minus
  Mul
  Div
  LParen
  RParen
end

enum NonTerminalIds
  S
  Expr
  Fact
  Atom
end

allowableTerminalIds = [
  TerminalIds::Number,
  TerminalIds::Add,
  TerminalIds::Minus,
  TerminalIds::Mul,
  TerminalIds::Div,
]

def evaluate(tree, allowableTerminalIds)
  case tree
  # parse leaves of the tree
  when Pegasus::Generated::TerminalTree
    tId = TerminalIds.new(Int32.new tree.terminal_id)

    if !(allowableTerminalIds.includes? tId)
      return Nil
    else
      if tId == TerminalIds::Number
        return Float64.new tree.string
      else
        return tId
      end
    end
    # parse branches of the tree
  when Pegasus::Generated::NonterminalTree
    nodeType = NonTerminalIds.new(Int32.new tree.nonterminal_id)

    case nodeType
    when NonTerminalIds::S
      return evaluate(tree.children[0], allowableTerminalIds)
    when NonTerminalIds::Atom
      # number
      if (tree.children.size == 1)
        return evaluate(tree.children[0], allowableTerminalIds)
      else
        # parenthesis
        return evaluate(tree.children[1], allowableTerminalIds)
      end
    when NonTerminalIds::Fact
      # atom
      if (tree.children.size == 1)
        return evaluate(tree.children[0], allowableTerminalIds)
      else
        # mul or div
        lhs = evaluate(tree.children[0], allowableTerminalIds)
        sign = evaluate(tree.children[1], allowableTerminalIds)
        rhs = evaluate(tree.children[2], allowableTerminalIds)

        case lhs
        when Float64
          case rhs
          when Float64
            case sign
            when TerminalIds::Mul
              return lhs * rhs
            when TerminalIds::Div
              return lhs / rhs
            end
          end
        end
      end
    when NonTerminalIds::Expr
      # atom
      if (tree.children.size == 1)
        return evaluate(tree.children[0], allowableTerminalIds)
      else
        # add or sub
        lhs = evaluate(tree.children[0], allowableTerminalIds)
        sign = evaluate(tree.children[1], allowableTerminalIds)
        rhs = evaluate(tree.children[2], allowableTerminalIds)

        case lhs
        when Float64
          case rhs
          when Float64
            case sign
            when TerminalIds::Add
              return lhs + rhs
            when TerminalIds::Minus
              return lhs - rhs
            end
          end
        end
      end
    end
  end

  puts "oops, parse_tree() should never reach this statement! something broke!"
end

def print_tree(tree, ident = 0)
  ident.times { STDOUT << "| " }

  case tree
  when Pegasus::Generated::TerminalTree
    puts "terminal tree: #{tree.terminal_id}, #{tree.string}"
  when Pegasus::Generated::NonterminalTree
    puts "non terminal tree: #{tree.name}"
    tree.children.each { |it| print_tree(it, ident + 1) }
  end
end

raw = gets
case raw
in String
  tree = Pegasus::Generated.process(raw)

  puts evaluate(tree, allowableTerminalIds)
in Nil
  puts "pegasus returned nil wtf"
end
