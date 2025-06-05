<div align="center">
<br/>

## 🙀 GitGud 🙀

### Automatically generate commit messages to ship faster

### 🚀🚀🚀🚀🚀🚀🚀🚀

<br/>
</div>

<div align="center">

[![PRs Welcome](https://img.shields.io/badge/PRs-Are%20welcome-brightgreen.svg?style=flat-square)](https://makeapullrequest.com) [![License](https://img.shields.io/github/license/syneki/notion-cms?label=License&style=flat-square)](LICENCE)

![Crates.io Total Downloads](https://img.shields.io/crates/d/gitgud) ![Crates.io Version](https://img.shields.io/crates/v/gitgud)

[🔨 Install](#🔨-install) • [🚀 Get started](#🚀-get-started) • [🔧 Configuration](#🔧-configuration)

[Contribute](#contributing) • [License](https://github.com/kerwanp/gitgud/LICENSE)

</div>

# 🔨 Install

## Linux

| Distribution | Repository                                   | Instructions                    |
| ------------ | -------------------------------------------- | ------------------------------- |
| Any          | [crates.io](https://crates.io/crates/gitgud) | `cargo install gitgud --locked` |

# 🚀 Get started

To get started you must initialize **GitGud** in your Git repository:

```sh
gitgud init
```

This will generate a `.gitgud` folder containing the configuration allowing you to write your own guidelines based on the standards of the project.

By default **GitGud** uses [OpenAI API](https://openai.com/) to generate your commit messages requiring you to provide an API Key that you can create in your [developer dashboard](https://platform.openai.com/api-keys).

Once created you can either set the `OPENAI_KEY` environment variable or add it in a `.env` file at the root of the repository.

Then simply run the following command to commit:

```
gitgud
```

# 🔧 Configuration

Configuration is located in the `.gitgud` folder and is made of two parts:

- `config.toml`: allows you to configure the model and the prompt variables.
- `prompt.txt`: which correspond to the template used for the prompt.

## Prompt

The prompt is a `.txt` used as a [tinytemplate](https://github.com/bheisler/TinyTemplate) allowing you to specify your commit message policies, guidelines, etc.

### Special variables

- `git_diff`: contains the diff of staged files.

## Config

The configuration allows you to configure the AI model used for generating messages as well as for providing variables to your prompt template making it easier to iterate.

```toml
[ai.openai]
model = "gpt-4.1-nano-2025-04-14"

[[prompt.examples]]
title = "Commit with infrastructure changes"
content = """
🧱 Added Horizontal Pod Autoscaling to helm charts
"""

[[prompt.examples]]
title = "Commit with bug fixes"
content = """
🐛 Fix homepage redirection with unknown browser language
"""
```

> [!NOTE]
> The prompt option values are accessible from within the prompt template

# Contributing

I'd love for you to contribute to this project. You can request new features by creating an issue, or submit a pull request with your contribution.
