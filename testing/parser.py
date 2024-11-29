# basically here i test the mathematical operations
# should probably delete this once implemented in rust
# stolen from this lovely repository:
# https://github.com/gnebehay/parser
#

import enum
import re


class TokenType(enum.Enum):
    T_NUM = 0
    T_PLUS = 1
    T_MINUS = 2
    T_MULT = 3
    T_DIV = 4
    T_LPAR = 5
    T_RPAR = 6
    T_END = 7


class Node:
    def __init__(self, token_type: TokenType, value=None):
        self.token_type = token_type
        self.value = value
        self.children = []


def lexical_analysis(s: str):
    mappings = {
        "+": TokenType.T_PLUS,
        "-": TokenType.T_MINUS,
        "*": TokenType.T_MULT,
        "/": TokenType.T_DIV,
        "(": TokenType.T_LPAR,
        ")": TokenType.T_RPAR,
    }

    tokens = []
    for c in s:
        if c in mappings:
            token_type = mappings[c]
            token = Node(token_type, value=c)
        elif re.match(r"\d", c):
            token = Node(TokenType.T_NUM, value=int(c))
        elif c.isspace():
            continue
        else:
            raise Exception("Invalid token: {}".format(c))
        tokens.append(token)
    tokens.append(Node(TokenType.T_END))
    return tokens


def match(tokens: list[Node], token: Node):
    if tokens[0].token_type == token:
        return tokens.pop(0)
    else:
        raise Exception("Invalid syntax on token {}".format(tokens[0].token_type))


def parse_e(tokens: list[Node]):
    left_node = parse_e2(tokens)

    while tokens[0].token_type in [TokenType.T_PLUS, TokenType.T_MINUS]:
        node = tokens.pop(0)
        node.children.append(left_node)
        node.children.append(parse_e2(tokens))
        left_node = node

    return left_node


def parse_e2(tokens: list[Node]):
    left_node = parse_e3(tokens)

    while tokens[0].token_type in [TokenType.T_MULT, TokenType.T_DIV]:
        node = tokens.pop(0)
        node.children.append(left_node)
        node.children.append(parse_e3(tokens))
        left_node = node

    return left_node


def parse_e3(tokens: list[Node]):
    if tokens[0].token_type == TokenType.T_NUM:
        return tokens.pop(0)

    match(tokens, TokenType.T_LPAR)
    expression = parse_e(tokens)
    match(tokens, TokenType.T_RPAR)

    return expression


def parse(input: str):
    tokens = lexical_analysis(input)
    ast = parse_e(tokens)
    match(tokens, TokenType.T_END)
    return ast
