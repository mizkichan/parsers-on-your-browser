import * as React from "react";

let wasm;

export default class App extends React.Component {
  constructor() {
    super();

    this.state = {
      algorithm: "earley",
      bnf: "",
      input: "",
      grammar: null
    };
  }

  componentDidMount() {
    import("../crate/pkg/parsers_on_your_browser").then(module => {
      wasm = module;
    });
  }

  handleControllsChange(value) {
    if (value.hasOwnProperty("bnf")) {
      value = { ...value, grammar: wasm.parse_bnf(value.bnf) || null };
    }
    this.setState(value);
  }

  render() {
    return (
      <div>
        <Controlls onChange={value => this.handleControllsChange(value)} />
        {this.state.grammar && <GrammarBox grammar={this.state.grammar} />}
        {this.state.algorithm === "earley" && <Earley />}
      </div>
    );
  }
}

const Controlls = ({ onChange }) => (
  <div>
    <div>
      <span className="label">Algorithm:</span>
      <Radio
        name="algorithm"
        defaultChecked
        label="Earley"
        value="earley"
        onChange={onChange}
      />
      <Radio name="algorithm" label="CYK" value="cyk" onChange={onChange} />
      <Radio name="algorithm" label="GLR" value="glr" onChange={onChange} />
    </div>
    <div>
      <span className="label">Grammar:</span>
      <textarea onChange={ev => onChange({ bnf: ev.target.value })} />
    </div>
    <div>
      <span className="label">Text to parse:</span>
      <input
        type="text"
        onChange={ev => onChange({ input: ev.target.value })}
      />
    </div>
  </div>
);

const GrammarBox = ({ grammar }) => (
  <table>
    <caption>Grammar</caption>
    <tbody>
      {grammar.rules.map((rule, i) => (
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

const Radio = ({ label, name, value, defaultChecked, onChange }) => (
  <label>
    <input
      type="radio"
      name={name}
      defaultChecked={defaultChecked}
      onChange={() => onChange({ [name]: value })}
    />
    {label}
  </label>
);

const Earley = () => <div>Earley</div>;

// vim: set ts=2 sw=2 et:
