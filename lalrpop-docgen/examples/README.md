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
├── full.md		# Grammar as markdown with railroad diagrams + EBNF for rules
├── grammar.ebnf	# Full grammar as EBNF
├── grammar.md		# Full grammar as EBNF in a markdown code-block
└── svg			# Generated railroad diagrams
    ├── comma.svg
    ├── expr.svg
    ├── exprop.svg
    ├── exprs.svg
    ├── factor.svg
    ├── factorop.svg
    ├── num.svg
    ├── term.svg
    └── tier.svg

2 directories, 12 files
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
$ pandoc --from gfm --to html --standalone full.md --output full.html
```

## Docusaurus

In this example folder, type the following:

```shell
$ npm i
$ npm start
```

Then the documentation will be available on a locally hosted docusaurus site at [http://localhost:3000](http://localhost:3000)

## Notes

If the target environment for your markdown renders the SVG images incorrectly
and shows scaling issues of the SVG railroad diagrams, then changing the code
generation to use HTML `img` tags instead of markdown image tags may be a quick
fix.

Using our example above, we add a flag to switch to HTML img tags:


```shell
lalrpop-docgen --railroad-mode img -mp docs/prolog -me docs/epilog  -gc Deploy,Query,Script ~/code/oss/tremor-rs
```


