import * as React from "react";
import { EarleyChart } from "./earley.jsx";
import { Symbolum, Rule } from "./common.jsx";

let crate;

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
    import("../crate/pkg").then(module => {
      crate = module;
    });
  }

  handleControlsChange(value) {
    if (value.hasOwnProperty("bnf")) {
      this.setState({ bnf: value.bnf }, this.parse);
    } else if (value.hasOwnProperty("input")) {
      this.setState({ input: value.input }, this.parse);
    }
  }

  parse() {
    switch (this.state.algorithm) {
      case "earley":
        const t = crate.parse_earley(this.state.bnf, this.state.input);
        const [grammar, earley] = t;
        this.setState({
          grammar,
          earley
        });
        break;
    }
  }

  render() {
    return (
      <div>
        <Controls onChange={value => this.handleControlsChange(value)} />
        <GrammarBox grammar={this.state.grammar} />
        {this.state.algorithm === "earley" && this.state.earley != null && (
          <EarleyChart
            start={this.state.grammar[0] && this.state.grammar[0].lhs}
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
          <th>#{i + 1}</th>
          <td>
            <Rule rule={rule} />
          </td>
        </tr>
      ))}
    </tbody>
  </table>
);

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

// vim: set ts=2 sw=2 et:
