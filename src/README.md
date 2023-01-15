 Stochcastic, 2-L context-sensitive, parametic, bracketed, L-System.

- → are used to separate the three components of a
production: the predecessor, the condition and the successor
- V is the alphabet of the system,

DOLsystems

OLsystems

1L/2L a.k.a Context Sensitive systems

## Bracted Systems
Bracked sysmtes are l-systems that contain information for the generation code on how to generate the structure. The information is a are two '[' and ']'. '[' means saving the current transform of the turtle and ']' restoring to the last saved tranform. This is implemented with a stack structure.

## Parametic Systems

Parametric L-systems operate on parametric words, which are strings of modules consisting of letters with associated parameters. The letters belong to an alphabet V, and the parameters belong to the set of real numbers R. A module with letter A ∈ V and parameters
a1, a2, ..., an ∈ R is denoted by A(a1, a2, ..., an). Every module belongs to the set M = V × R∗, where R∗ is the set of all finite sequences of parameters. The set of all strings of modules and the set of all nonempty strings are denoted by M∗ = (V × R∗)∗ and M+ = (V × R∗)+, respectively.

Parameters can be real number expression using +, −, ∗, /; the exponentiation operator ∧, the relational operators <, >, =; the logical operators !, &, | (not, and, or); and parentheses (). Standard rules for constructing syntactically correct expressions and for operator precedence are observed.

A parametric OL-system is defined as an ordered quadruplet G = Parametric
<V, Σ, ω, P>, where
- V is the alphabet of the system,
- Σ is the set of formal parameters,
- ω ∈ (V × R∗)+ is a nonempty parametric word called the axiom,
- P ⊂ (V × Σ∗) × C(Σ) × (V × E(Σ))∗ is a finite set of productions.

left context can be used to simulate control signals that propagate acropetally from
the root or basal leaves towards the apices of the modeled plant

the right context represents signals that propagate basipetally, from
the apices towards the root

- [1] http://algorithmicbotany.org/papers/abop/abop.pdf