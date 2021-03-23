export const variants = [{
    VAR1: 1000,
    VAR2: 11,
    VAR3: 'foo'
}, {
    VAR1: 10,
    VAR2: 1,
    VAR3: 'foo'
}, {
    VAR1: 10,
    VAR2: 9,
    VAR3: 'foo'
}, {
    VAR1: 10,
    VAR2: 9,
    VAR3: 'bar'
}];

export const yaml = `
---
a: 1
b:
  - if: VAR1 > 100
    value: condition1
  - if: VAR1 + VAR2 > VAR1 * VAR2
    value: condition2
  - if: VAR1 > VAR2 and VAR3 == 'foo'
    value: condition3
  - if: true
    value: default
`;

export const toml = `
a = 1

[[b]]
if = "VAR1 > 100"
value = "condition1"

[[b]]
if = "VAR1 + VAR2 > VAR1 * VAR2"
value = "condition2"

[[b]]
if = "VAR1 > VAR2 and VAR3 == 'foo'"
value = "condition3"

[[b]]
if = true
value = "default"
`;

export const json = `
{
  "a": 1,
  "b": [
    {
      "if": "VAR1 > 100",
      "value": "condition1"
    },
    {
      "if": "VAR1 + VAR2 > VAR1 * VAR2",
      "value": "condition2"
    },
    {
      "if": "VAR1 > VAR2 and VAR3 == 'foo'",
      "value": "condition3"
    },
    {
      "if": true,
      "value": "default"
    }
  ]
}
`;