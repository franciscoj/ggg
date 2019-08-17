# ggg: a CLI for Github

`ggg` is a CLI to assist with using Github.

It is on an extremely early development stage and its usage is completely
discouraged. (That's why there are no instructions on this README on how to do
so).

The final goal is that a developer can perform the most common operations on
pull requests without needing to leave the terminal or her editor. 

The main purpose of developing such a tool is that I teach myself some Rust.

## TODO / wanted features

### CLI

  - [x] Be able to count pending notifications on Github.
  - [x] Be able to list pending notifications per repo.
  - [ ] Interactive interface for notifications.  Allow to list, inspect and
   comment/reply when possible. Grab some inspiration from
    [yarn-upgrade-interactive]

### Integrations / Editor plugins
  - [ ] Notifications indicator plugin for tmux/Vim.
  - [ ] Current pull request notifications indicator for Vim. (If the current
      branch is a PR show notifications on it together with the code
      associated, if any)
  - [ ] Allow to comment/reply on PRs.

### Development
  - [ ] Add unit tests.
  - [ ] Add integration tests (check [mockito])

## Resources

  - crate to mock http requests: [mockito]
  - crate for general mocks: [mockall]
  - Rust doc on building CLIs: [cli]
  - Cookin' with rust: [cookbook]

[mockall]: https://docs.rs/mockall/0.3.0/mockall/
[mockito]: https://docs.rs/mockito/0.20.0/mockito/
[cli]: https://rust-lang-nursery.github.io/cli-wg/index.html
[cookbook]: https://rust-lang-nursery.github.io/rust-cookbook/intro.html
[yarn-upgrade-interactive]: https://yarnpkg.com/lang/en/docs/cli/upgrade-interactive/
