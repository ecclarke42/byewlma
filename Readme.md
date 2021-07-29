# `byewlma`: Opinionated Yew Components Styled with Bulma

## Usage

This library is designed with..

<!-- TODO -->

## Showcase

You can get a taste of usage for the components by looking at the [`showcase`](./examples/showcase). This also provides a good example of how to use `byewlma` in a project. Run the showcase using [`trunk`](https://github.com/thedodd/trunk):

```bash
cd ./examples/showcase && trunk serve --open
```

## Customizing Bulma

Bulma can be customized through a number of `sass` variables. The [`showcase`](./examples/showcase) example uses standard bulma colors, but the [`customize`](./examples/customize) example shows a very simple example of using different variables.

For further customization, see the [`trunk`](https://github.com/thedodd/trunk) documentation

## Specialized Styles

Some more complex components ([`search_select`](./src/components/search_select/mod.rs) and [`message_service`](./src/components/message_service/mod.rs)) require extra styles. These can be included directly using the `index.scss` file in each components module, or by `npm install`ing the `byelma` package from NPM, which
