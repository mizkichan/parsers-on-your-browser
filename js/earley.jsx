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
        <th>Reason</th>
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
            <td>{reason(state.reason)}</td>
          </tr>
        ))
      )}
    </tbody>
  </table>
);

const reason = reason => {
  switch (reason.kind) {
    case "Initial":
      return "Start rule";

    case "Predict":
      return `Predict from S(${reason.from_position}) #${
        reason.from_state
      } with rule #${reason.from_rule}`;

    case "Scan":
      return `Scan from S(${reason.from_position}) #${reason.from_state}`;

    case "Complete":
      return `Complete from S(${reason.from_position}) #${
        reason.from_state
      } with S(${reason.with_position}) #${reason.with_state}`;

    default:
      return null;
  }
};

// vim: set ts=2 sw=2 et:
