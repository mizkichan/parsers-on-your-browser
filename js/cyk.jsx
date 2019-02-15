import * as React from "react";
import { Symbolum, Rule } from "./common.jsx";

export const CYKTable = ({ table, input }) => (
  <table>
    <caption>CYK Table</caption>
    <tbody>
      {table.map((_, i) => (
        <tr key={i}>
          {table[table.length - 1 - i].map((col, j) => (
            <td key={j}>
              {0 < col.length && <Symbolum symbolum={{ NonTerminal: col }} />}
            </td>
          ))}
        </tr>
      ))}
    </tbody>
    <tfoot>
      <tr>
        {input.split(" ").map((word, i) => (
          <th key={i}>{word}</th>
        ))}
      </tr>
    </tfoot>
  </table>
);

// vim: set ts=2 sw=2 et:
