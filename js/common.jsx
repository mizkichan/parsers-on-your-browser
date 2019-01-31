import * as React from "react";

export const Symbolum = ({ symbolum }) => {
  if (symbolum.hasOwnProperty("Terminal")) {
    return <span className="terminal">{symbolum.Terminal}</span>;
  } else {
    return <span className="non-terminal">{symbolum.NonTerminal}</span>;
  }
};

export const Rule = ({ rule, dot }) => (
  <div className="rule">
    <div>
      <Symbolum symbolum={{ NonTerminal: rule.lhs }} />
    </div>
    →
    <div>
      {[
        rule.rhs.flatMap((symbolum, j) => [
          dot === j ? "·" : null,
          <Symbolum key={j} symbolum={symbolum} />
        ]),
        dot === rule.rhs.length ? "·" : null
      ]}
    </div>
  </div>
);

// vim: set ts=2 sw=2 et:
