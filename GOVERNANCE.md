# Governance

This document outlines the governance model for Biome. This document includes the contributor model, code review, merging, and the consequences and process for Code of Conduct violations.

## Member Roles

All members must follow the [Code of Conduct](CODE_OF_CONDUCT.md). Consequences for member violations are detailed in [Moderation](#moderation).

### Lead

Leads are the owners of the organisation.

Leads have additional privileges over core contributors. Leads control and maintain sensitive project assets and act as tiebreakers in the event of disagreements. In case of disagreements, only **one** lead must be involved in the resolution.

These assets and responsibilities are but are not limited to:
- Access to social accounts
- Administration privileges of the [Biome Discord server][discord]
- Administration privileges of the [Biome GitHub org][gh-org]
- Website accesses (hosting platform, domain name, etc.)
- Access to sensitive emails, such as the CoC email

Also:
- Ability to vote for new leads
- Onboard new [core contributors](#core-contributor) and new leads;
- Takes part in [project decisions](#project-direction-and-planning)

The ownership of assets is only sometimes evenly distributed among all the leads.

#### Lead nomination

Leads may only be self-nominated. Being a lead only adds burden to a core contributor: more rights to be held accountable with, and more responsibilities.

When nominating a lead, a new thread in the `#leads` channel with a case of why the person should join as [core contributor](#core-contributor) has to be created.

`#leads` is a private channel available only to people who have the `Lead` role.

The voting period will follow the [voting rules](#voting-rules). An individual becomes a lead if all the other leads vote in favour.

If vote is successful, the lead will ask the nominee if they want to accept the title.

In the event of a rejection, the nominated person will be privately given the requirements they have yet to meet. Details of the discussion - for example, those who objected to the nomination - will not be disclosed.

### Core Contributor

Core Contributors are outstanding [maintainers](#maintainer), are ambassadors of Biome organisation and lead by example the community.

- Push access to the [Biome GitHub org][gh-org], this includes all repositories
- `Core contributor` status on the [Biome Discord server][discord]
- Takes part in [project decisions](#project-direction-and-planning)
- Ability to [vote](#maintainer-nomination) on new maintainers and [vote](#core-contributor-nomination) on new core contributors
- Onboard new [maintainers](#maintainer)

#### Core contributor nomination

Only [maintainers](#maintainer) can be nominated to be core contributors.

Core contributors may either be nominated by another, [Core contributor](#core-contributor), or [lead](#lead).

When nominating a core contributor, a new thread in the `#core-contributors` channel has to be created, with a case and [their contributions](#contributions) of why the person should join as [core contributor](#core-contributor).

`#core-contributors` is a private channel available only to people who have the `Core contributor` role.

The voting period will follow the [voting rules](#voting-rules). An individual becomes a [core contributor](#core-contributor) if:
- receive at least a positive vote from a [lead](#lead);
- more than 70% of positive votes across the members of the Core contributors team;

Voting requirements may change in the future based on the amount of people involved in the vote.

A core contributor can be self-nominated. They will have to message a [lead maintainer](#lead) privately and present a case. Then, the lead can:
- reject the nomination in place, provide reasons why and possibly some suggestions for the future; the lead
- initiate a voting session among the core contributors;

When the verdict is reached, the thread will be deleted, and a [lead](#lead) will reach out to the nominee.

If vote is successful, the core member will ask the nominee if they want to accept the title.

In the event of a rejection, the nominated person will be privately given the requirements they have yet to meet. Details of the discussion - for example, those who objected to the nomination - will not be disclosed.

### Maintainer

Maintainers are those with a history of consistent contributions, including but not limited to pull requests, project management, or support. These privileges include:

- Push access to some of the repositories of the [Biome GitHub org][gh-org]
- `Maintainer` status on the [Biome Discord server][discord]
- Ability to [vote](#voting) on project decisions
- Ability to [vote](#maintainer-nomination) on new maintainers

#### Maintainer nomination

Maintainers may either be nominated by another [maintainer](#maintainer), [Core contributor](#core-contributor), [lead](#lead).

When nominating a new maintainer, a new thread in the `#maintainers` channel has to be created, provide a link to the history of the [person's contributions](#contributions) to the project and a brief explanation of why the person should join as [maintainer](#maintainer).

`#maintainers` is a private channel available only to people who have the `maintainer` role.

The voting period will follow the [voting rules](#voting-rules). An individual becomes a [maintainer](#maintainer) if:
- reaches more than 50% of positive votes among the people who have voting rights;
- if the voting session ends with 50%, the Lead maintainers will take the decision;

A maintainer can be self-nominated. They will have to message a [lead maintainer](#lead) privately with links to their contributions to the project. Then, the lead can:
- reject the nomination in place, provide reasons why and possibly some suggestions for the future;
- initiate a voting session among the maintainers;

When the verdict is reached, the thread will be deleted, and a [core contributor](#core-contributor) or [lead](#lead) will reach out to the nominee.

If vote is successful, the lead will ask the nominee if they want to accept the title.

In the event of a rejection, the nominated person will be privately given the requirements they have not met. Details of the discussion, such as the names of those who objected the nomination, will not be disclosed.

### Voting Rules

- Members are required to vote.
- Members may abstain from a vote.
- Members who do not vote within seven days will automatically abstain.
- Leads may reduce the seven days automatic abstain for urgent decisions.
- Leads reserve the right to veto approval with a publicly disclosed reason.
- It's highly suggested to pair the vote with a reason of the vote, e.g. "I vote in favour because that person writes good code". These reasons will be collected and brought up to the nominee at the end of the voting phase.

### Ownership

One or more people can own certain parts of the codebase. This process is informal, and inclusion could result from substantial contributions or delegation by other members. A maintainer's responsibility is to identify the relevant owners and ensure there's an understanding when it comes to code review.

### Inactivity

There aren't strict requirements and expectations around activity for core contributors and maintainers; although long periods of inactivity must be communicated to the team.

Inactive core contributors or maintainers may have voting and access rights removed and status removed, and given them the _Past contributor_ status. A core contributor or maintainer may request their voting rights and status back upon sufficient activity.

> [!NOTE]
> The definition of "long periods", and how long an individual needs to stay inactive in order to have their voting rights removed have yet to be decided.

### Contributions

Contributions to the project aren't only limited to code. Contributions come in different forms and ways:
- Documentation
- Code to all the repositories of the [GitHub org][gh-org]
- Support in primary channels of the organization, e.g. GitHub and Discord
- Support in secondary channels, e.g. StackOverflow, etc.
- Website;

## Project direction and planning

Project direction and planning is a shared responsibility amongst members. Core contributors are responsible for dictating high-level goals and the project scope that should be adhered to.

### Roadmap

Leads may create a roadmap document to share with the community once the core contributors have set the project's goals.

Roadmaps don't provide dates or deadlines; they only reflect what the core contributors decide to work on and how to spend the resources.

The span of a roadmap should cover six months, but it might vary.

## Code review

We have a reasonably liberal approach to code review and merging. We value quick iteration and low development friction, which comes with great responsibility. Reverting code is easy, so landing code should be just as easy. Because of this, Biome will have discrete releases rather than rolling releases that are automatically published.

- If you own a particular area, you can merge it without any review despite PR size.
- If there are comments or suggestions after a PR is merged after the fact, allow yourself time to address them in a follow-up PR. If you don't respond in a reasonable timeframe, then create an issue to track.
- Ensure that the PR summary is detailed, listing steps you took to verify the rationale and relevant issues and people involved in any prior discussion.
- Ensure that PRs contain adequate tests and code comments for a future contributor to derive intent and modify your code safely.
- You are welcome to the repos for your WIP branches. If you don't use your own fork, prepend the name of the branch with your github handle, e.g. `<handle>/experiment-code` Branches that have more than four months of inactivity will be pruned.
- If you add a new feature, ensure it has been discussed or approved on GitHub or Discord.
- If necessary, identify potential owners for PR review and approval.
- All code must go through Pull Requests (PR) and pass status checks before being merged. If a PR is merged that breaks `main` due to the branch not being up-to-date, then it should either be reverted or a quick fix merged as a separate PR.
- If a PR is against code you have previously committed and is either small changes, bug fixes, or refactors, then you're free to merge it without any review. However, if you need more confidence in your changes, you can wait for approval from another maintainer or core contributor.

## Moderation

Outlined below is the process for Code of Conduct violation reviews.

### Reporting

Anyone may report a violation. Violations can be reported in the following ways:

- In private, via <biomejs@googlegroups.com> which is listed in the [Code of Conduct](./CODE_OF_CONDUCT.md). All core contributors monitor this email address.
- In private, via email to one or more core contributors.
- In private, via direct message to a core contributor on [Discord server][discord].
- In public, via a GitHub comment (mentioning `@biomejs/core-contributors`).
- In public, via the project [Discord server][discord].

### Who gets involved?

Each report will be assigned reviewers. These will initially be all project [core contributors](#core-contributors-team).

In the event of any conflict of interest - e.g. core contributors who are personally connected to a situation they must immediately recuse themselves.

At the reporter's request and if deemed appropriate by the reviewers, another neutral third party may be involved in the review and decision process.

### Review

If a report doesn't contain enough information, the reviewers will strive to obtain all relevant data before acting.

The reviewers will then review the incident and determine, to the best of their ability:

- What happened.
- Whether this event constitutes a Code of Conduct violation.
- Who, if anyone, was involved in the violation.
- Whether this is an ongoing situation.

The reviewers should aim to have a resolution agreed very rapidly; if they have not decided within a week, they will inform the parties of the planned date.

### Resolution

The reviewers will determine responses based on the information gathered and the potential consequences. It may include:

- Taking no further action
- Issuing a reprimand (private or public)
- Asking for an apology (private or public)
- Permanent ban from the GitHub org and Discord server
- Revoked contributor or moderator status

## OpenCollective fund allocation

- Funds will be allocated for project-specific services such as domain registration and website hosting.
- Core contributors are allowed to claim expenses related to conferences where they talk about Biome, or Biome workshops that they run. Expenses that can be claimed are but not limited to: trip (plane, train, etc.), meals, transport, parking.
  - The person will have to provide proof of attendance (badge, video, etc.);
  - Expenses will have to be uploaded via the open collective website.
- Expenses will be approved by the [leads](#lead).

> [!NOTE]
> Other usage of funds has yet to be decided.


[gh-org]: https://github.com/biomejs
[discord]: https://discord.gg/BypW39g6Yc±
