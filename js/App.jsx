import * as React from "react";

let wasm;

export default class App extends React.Component {
  constructor() {
    super();

    this.state = {
      algorithm: "earley",
      bnf: "",
      input: "",
      grammar: [],
      earley: null
    };
  }

  componentDidMount() {
    import("../crate/pkg/parsers_on_your_browser").then(module => {
      wasm = module;
    });
  }

  handleControlsChange(value) {
    if (value.hasOwnProperty("bnf")) {
      value.grammar = wasm.parse_bnf(value.bnf);
    }
    if (value.hasOwnProperty("input")) {
      if (this.state.algorithm === "earley") {
        value.earley = wasm.parse_earley(this.state.bnf, value.input);
      }
    }
    this.setState(value);
  }

  render() {
    return (
      <div>
        <Controls onChange={value => this.handleControlsChange(value)} />
        <GrammarBox grammar={this.state.grammar} />
        {this.state.algorithm === "earley" && this.state.earley != null && (
          <EarleyChart
            start={this.state.grammar[0].lhs}
            stateSets={this.state.earley}
          />
        )}
      </div>
    );
  }
}

const Controls = ({ onChange }) => (
  <div className="controls">
    <Row label="Algorithm:">
      <RadioGroup
        name="algorithm"
        onChange={algorithm => onChange({ algorithm })}
      >
        <RadioButton defaultChecked label="Earley" value="earley" />
        <RadioButton label="CYK" value="cyk" />
        <RadioButton label="GLR" value="glr" />
      </RadioGroup>
    </Row>
    <Row label="Grammar:">
      <textarea onChange={ev => onChange({ bnf: ev.target.value })} />
    </Row>
    <Row label="Text to parse:">
      <input
        type="text"
        onChange={ev => onChange({ input: ev.target.value })}
      />
    </Row>
  </div>
);

const Row = ({ label, children }) => (
  <div className="control-row">
    <div>{label}</div>
    {children}
  </div>
);

const GrammarBox = ({ grammar }) => (
  <table className="grammar">
    <caption>Grammar{grammar.length == 0 ? " (No Rules)" : ""}</caption>
    <tbody>
      {grammar.map((rule, i) => (
        <tr key={i}>
          <td>
            <Symbolum symbolum={{ NonTerminal: rule.lhs }} />
          </td>
          <td>→</td>
          <td>
            {rule.rhs.map((symbolum, i) => (
              <Symbolum key={i} symbolum={symbolum} />
            ))}
          </td>
        </tr>
      ))}
    </tbody>
  </table>
);

const Symbolum = ({ symbolum }) => {
  if (symbolum.hasOwnProperty("Terminal")) {
    return <span className="terminal">{symbolum.Terminal}</span>;
  } else {
    return <span className="non-terminal">{symbolum.NonTerminal}</span>;
  }
};

const RadioGroup = ({ name, onChange, children }) => (
  <div>
    {React.Children.map(children, child =>
      React.cloneElement(child, { name, onChange })
    )}
  </div>
);

const RadioButton = ({ label, name, value, defaultChecked, onChange }) => (
  <label>
    <input
      type="radio"
      name={name}
      defaultChecked={defaultChecked}
      onChange={() => onChange(value)}
    />
    {label}
  </label>
);

const EarleyChart = ({ start, stateSets }) => (
  <table>
    <caption>Earley Chart</caption>
    <thead>
      <tr>
        <th>State Set</th>
        <th>#State</th>
        <th colSpan={3}>Rule</th>
        <th>Position</th>
      </tr>
    </thead>
    <tbody>
      {stateSets.map((stateSet, k) =>
        stateSet.map((state, i) => (
          <tr
            key={`${k}${i}`}
            className={
              state.rule.lhs === start &&
              state.rule.rhs.length === state.dot &&
              state.position === 0
                ? "earley-complete"
                : null
            }
          >
            {i === 0 ? <th rowSpan={stateSet.length}>{`S(${k})`}</th> : null}
            <th>{`#${i + 1}`}</th>
            <td>
              <Symbolum symbolum={{ NonTerminal: state.rule.lhs }} />
            </td>
            <td>→</td>
            <td>
              {[
                state.rule.rhs.map((symbolum, j) => [
                  state.dot === j ? "·" : null,
                  <Symbolum key={j} symbolum={symbolum} />
                ]),
                state.dot === state.rule.rhs.length ? "·" : null
              ]}
            </td>
            <td>{state.position}</td>
          </tr>
        ))
      )}
    </tbody>
  </table>
);

// vim: set ts=2 sw=2 et:
