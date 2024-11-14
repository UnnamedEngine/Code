# Contributing Guidelines

This is the contributing guidelines for this repository. The guidelines explains how contributions should be made.

- [Contributing Guidelines](#contributing-guidelines)
  - [Forking the Repository](#forking-the-repository)
  - [Cloning your Fork](#cloning-your-fork)
  - [Issue Selection](#issue-selection)
  - [Branch Creation](#branch-creation)
    - [Example](#example)
  - [Before Pull](#before-pull)
  - [Creating a Pull Request](#creating-a-pull-request)
  - [Request Review](#request-review)
- [Important Information](#important-information)

## Forking the Repository

1. Go to the main repository page.
2. Click on the **Fork** button in the top-right corner to create a copy of the repository in your Github account.

This personal fork will be the repository to which you will push yur changes.

## Cloning your Fork

After forking the repository, clone your forked repository to your local machine:

```bash
git clone https://github.com/<your-username>/<forked-repository-name>.git unnamedcode
```

Replace <your-username> with your GitHub username and <forked-repository-name> with the name you gave the fork. This creates a local copy of your fork.

## Issue Selection

1. Browse the open [**issues**](https://github.com/UnnamedEngine/Code) in the main repository.
2. Choose an issue that interests you or is labaled as [**good first issue**](https://github.com/UnnamedEngine/Code/issues?q=is%3Aopen+is%3Aissue+label%3A%22%E2%9D%95+Good+First+Issue%22) if you're a new contributor.
3. If no open issue suits your contribution, create a new issue describing the problem or enhancement. Wait for feedback if needed before proceeding.

## Branch Creation

Create a new branch for each issue or feature you work on. Use a consistent naming convention for branches to make it easier to track changes:

```bash
git checkout -b <issue_category>/issue-<issue_number>-short-description-kebab-case
```

### Example

```bash
git checkout -b fix/issue-12-enhance-performance
```

## Before Pull

1. **Code Style**: Ensure your code adheres to the project's code guidelines.
2. **Documentation**: Update or add documentation for any new functionality.
3. **Testing**: Run existing tests to confirm nothing breaks. Add new tests if relevant.

## Creating a Pull Request

1. Push your branch to your forked repository.

```bash
git push origin <issue_category>/issue-<issue_number>-short-description-kebab-case
```

2. Go to your fork on Github and open a pull request (PR) from your branch to the main repository's main branch.
3. In the PR description, include:
   - The issue number being addressed.
   - A brief summary of changes.
   - Any additional information that reviewers should know .

## Request Review

Tag the appropriate reviewers in your pull request. Wait for approval before merging.

# Important Information

1. Possible values for `<issue_category>` are:
   - `feature`: new feature or enhancement
   - `fix`: fixes a bug
   - `docs`: documentation changes
   - `release`: marks a new release
   - `refactor`: changing code for performance or convenience purpose
   - `experiment`: features that are being tested or researched
