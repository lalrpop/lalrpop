# Examples

All examples assume that they are executed from this directory.

## Use defaults and specify lalrpop file only 

```shell
$ lalrpop-docgen calculator.lalrpop
```

This will produce the following resources

```shell
$ tree docs
docs
├── Full.md		# Grammar as markdown with railroad diagrams + EBNF for rules
├── grammar.ebnf	# Full grammar as EBNF
├── grammar.md		# Full grammar as EBNF in a markdown code-block
└── svg			# Generated railroad daigrams
    ├── Comma.svg
    ├── Expr.svg
    ├── ExprOp.svg
    ├── Exprs.svg
    ├── Factor.svg
    ├── FactorOp.svg
    ├── Num.svg
    ├── Term.svg
    └── Tier.svg

1 directory, 12 files
```

## Annotated example

A mixin markdown file can be specified for each rule in a grammar. These mixin files
will be merged with the generated markdown `before` the railroad diagram and EBNF form
of each rule and `after`.

A `prolog` folder with markdown files will be merged `before`.
An `epilog` folder with markdown files will be merged `after`.

A prolog file name for a rule named `Foo` will be `Foo.md`.
An epilog file name for a rule named `Foo` will also be `Foo.md`


```shell
$ lalrpop-docgen --out-dir docs -mp static/prolog -me static/epilog calculator.lalrpop
```

## Complex grammars


Large or complex grammars with hundreds of productions may need to be split
into multiple sections. For example, the [tremor project](https://github.com/tremor-rs/)
uses LALRPOP for its grammar with a custom Lexer.

The language has multiple top level DSLs defined within the same grammar file.

The `--grammar-cut` flag ( `-gc` for short ) will split out multiple markdown documents
from a single grammar. A complete or `Full` grammar will also be produced.

Here is how the tremor project generate their language reference from their
normative LALRPOP grammar.

```
lalrpop-docgen -mp docs/prolog -me docs/epilog  -gc Deploy,Query,Script ~/code/oss/tremor-rs
```

## Using the generated documents

## Pandoc

Convert to html:

```shell
$ pandoc --from gfm --to html --standalone Full.md --output Full.html
```

## Docusaurus

In this example folder, type the following:

```shell
$ npm i
$ npm start
```

Then browse over to a [docusaurus](http://localhost:3000) Site

