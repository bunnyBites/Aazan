# Contributing to Aazan

First off, thank you for considering contributing to [Project Name]! It's people like you who make this project great. Your help is essential for keeping it alive.

This document provides guidelines for contributing to this project. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Pull Requests](#pull-requests)
- [Style Guides](#style-guides)
  - [Git Commit Messages](#git-commit-messages)
  - [Code Style](#code-style)
- [Setting Up Your Development Environment](#setting-up-your-development-environment)
- [Any Questions?](#any-questions)

## Code of Conduct

This project and everyone participating in it are governed by the `CODE_OF_CONDUCT.md`. By participating, you are expected to uphold this code. Please report unacceptable behaviour.

## How Can I Contribute?

### Reporting Bugs

This section guides you through submitting a bug report for Aazan. Following these guidelines helps maintainers and the community understand your report, reproduce the behaviour, and find related reports.

- **Check the issue tracker** to see if the bug has already been reported. If it has and the issue is still open, add a comment to the existing issue instead of opening a new one.
- If you are unable to find an open issue addressing the problem, **open a new one**. Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample or an executable test case** demonstrating the expected behaviour that is not occurring.

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for [Project Name], including completely new features and minor improvements to existing functionality.

- **Perform a cursory search** in the [issue tracker]([Link to issue tracker]) to see if the enhancement has already been suggested.
- If it has not, open a new issue. Provide a clear and detailed explanation of the feature you want, along with its importance.
- Explain the **"what" and "why"** of the enhancement. What is the use case? What problem does it solve?

### Your First Code Contribution

Unsure where to begin contributing? You can start by looking through these `good first issue` and `help wanted` issues:
* **Good first issues** - issues which should only require a few lines of code, and a test or two.
* **Help wanted issues** - issues which should be a bit more involved than `good first issues`.

### Pull Requests

The process described here has several goals:
- Maintain [Project Name]'s quality
- Fix problems that are important to users
- Engage the community in working toward the best possible [Project Name]
- Enable a sustainable system for maintainers to review contributions

Please follow these steps to have your contribution considered by the maintainers:

1.  **Fork the repository** on GitHub.
2.  **Clone your fork** locally:
    ```bash
    git clone [https://github.com/your-username/](https://github.com/your-username/)Aazon.git
    ```
3.  **Add the original repository as an upstream remote**:
    ```bash
    git remote add upstream [https://github.com/](https://github.com/)bunnyBites/Aazan.git
    ```
4.  **Create a new branch** for your changes from the `main` branch:
    ```bash
    git checkout -b name-of-your-feature-or-fix
    ```
    (Please use a descriptive branch name like `feat/new-button` or `fix/login-bug`)
5.  **Make your changes** and commit them. Please follow our [Git Commit Messages](#git-commit-messages) style guide.
6.  **Push your branch** to your fork on GitHub:
    ```bash
    git push origin name-of-your-feature-or-fix
    ```
7.  **Open a pull request** from your fork to the original repository's `main` (or `develop`) branch.
8.  **Provide a clear description** of your changes in the pull request. Explain the "what" and "why" of your changes. If your PR fixes an open issue, please link it (e.g., `Closes #123`).
9.  Wait for a maintainer to review your PR. Address any feedback or requested changes.

## Style Guides

### Git Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. Please ensure your commit messages are structured as follows:
