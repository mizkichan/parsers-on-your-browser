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
        {this.state.algorithm === "earley" && <Earley />}
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

const Earley = () => <div>Earley</div>;

// vim: set ts=2 sw=2 et:
