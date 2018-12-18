import * as React from "react";
import { Grammar } from "../crate/pkg/parsers_on_your_browser";

let wasm: typeof import("../crate/pkg/parsers_on_your_browser");

export default class App extends React.Component {
  state: {
    bnf: string;
    input: string;
    grammar: Grammar | null;
  };

  constructor(props: {}) {
    super(props);

    this.state = {
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

  handleBNFUpdate(bnf: string) {
    const grammar = wasm.parse_bnf(bnf) || null;
    this.setState({ bnf, grammar });
  }

  render() {
    return (
      <div>
        <Controlls
          bnf={this.state.bnf}
          input={this.state.input}
          onBNFChange={bnf => this.handleBNFUpdate(bnf)}
          onInputChange={input => {
            this.setState({ input });
          }}
        />

        {this.state.grammar && <GrammarBox grammar={this.state.grammar} />}
      </div>
    );
  }
}

const Controlls = ({
  bnf,
  input,
  onBNFChange,
  onInputChange
}: {
  bnf: string;
  input: string;
  onBNFChange: (x: string) => void;
  onInputChange: (x: string) => void;
}) => (
  <div>
    <div>
      <span className="label">Grammar:</span>
      <textarea
        value={bnf}
        onChange={ev => {
          onBNFChange(ev.target.value);
        }}
      />
    </div>
    <div>
      <span className="label">Text to parse:</span>
      <input
        type="text"
        value={input}
        onChange={ev => {
          onInputChange(ev.target.value);
        }}
      />
    </div>
  </div>
);

const GrammarBox = ({ grammar }: { grammar: Grammar }) => (
  <table>
    <caption>Grammar</caption>
    <tbody>
      <tr />
    </tbody>
  </table>
);

// vim: set ts=2 sw=2 et:
