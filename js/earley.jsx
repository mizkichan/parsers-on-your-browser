import * as React from "react";
import { Symbolum, Rule } from "./common.jsx";

export const EarleyChart = ({ start, stateSets }) => (
  <table>
    <caption>Earley Chart</caption>
    <thead>
      <tr>
        <th>State Set</th>
        <th>#State</th>
        <th>Rule</th>
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
            {i === 0 ? <th rowSpan={stateSet.length}>S({k})</th> : null}
            <th>#{i + 1}</th>
            <td>
              <Rule rule={state.rule} dot={state.dot} />
            </td>
            <td>{state.position}</td>
          </tr>
        ))
      )}
    </tbody>
  </table>
);

// vim: set ts=2 sw=2 et:
