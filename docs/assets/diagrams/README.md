# Diagram assets

[← Documentation hub](../../README.md) · [Architecture](../../guides/architecture.md)

The architecture guide uses two generated diagrams:

- `tokenizer.png` illustrates lexical analysis;
- `parser.png` illustrates recursive-descent parsing and direct evaluation.

The adjacent `.dot` files are the editable sources. Do not edit the generated PNG files directly.

## Regenerate

Install [Graphviz](https://graphviz.org/download/) and run these commands from the repository root:

```bash
dot -Tpng -Gdpi=144 docs/assets/diagrams/tokenizer.dot -o docs/assets/diagrams/tokenizer.png
dot -Tpng -Gdpi=144 docs/assets/diagrams/parser.dot -o docs/assets/diagrams/parser.png
```

Review both images after regeneration to confirm that labels remain readable and edges do not overlap important content.
