# Governance

This document outlines the governance model for Biome.

It describes various parts of how the project is managed as well as accepted practices for day-to-day operation:

* [Contributor Model](#contributor-model)
  + [Lead](#lead)
    - [Lead nomination](#lead-nomination)
  + [Core Contributor](#core-contributor)
    - [Core contributor nomination](#core-contributor-nomination)
  + [Maintainer](#maintainer)
    - [Maintainer nomination](#maintainer-nomination)
  + [Voting Rules](#voting-rules)
  + [Ownership](#ownership)
  + [Inactivity](#inactivity)
  + [Contributions](#contributions)
  + [Governance changes](#governance-changes)
* [Project Direction](#project-direction)
  + [Roadmap](#roadmap)
* [Code review](#code-review)
* [Financial Contributions](#financial-contributions)
  + [Sponsorship](#sponsorship)
  + [Community-Funded Bounties](#community-funded-bounties)
  + [Project-Funded Bounties](#project-funded-bounties)
  + [Paid Contracting](#paid-contracting)
  + [Fund Allocation](#fund-allocation)
    - [Expenses](#expenses)
* [Moderation](#moderation)
  + [Reporting](#reporting)
  + [Who gets involved?](#who-gets-involved)
  + [Review](#review)
  + [Resolution](#resolution)


## Contributor Model

All members must follow the [Code of Conduct](CODE_OF_CONDUCT.md). Consequences for member violations are detailed in [Moderation](#moderation).

### Lead

Leads are the owners of the organisation.

Leads have additional privileges over core contributors. Leads control and maintain sensitive project assets and act as tiebreakers in the event of disagreements. In case of disagreements, only **one** lead must be involved in the resolution.

These assets and responsibilities are but are not limited to:
- Access to social accounts.
- Administration privileges of the [Biome Discord server][discord].
- Administration privileges of the [Biome GitHub org][gh-org].
- Website credentials (hosting platform, domain name, etc.).
- Access to sensitive emails, such as the CoC email.

Also:
- Ability to vote for new leads.
- Onboard new [core contributors](#core-contributor) and new leads.
- Takes part in [project decisions](#project-direction-and-planning).
- Access to the Discord `Lead` category and channels that belong to this category.
- Propose changes to the governance document via PR.

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
- Assign [pledges to issues](#bounties).
- Access to the Discord `Core contributors` category and channels that belong to this category.
- Propose changes to the governance document via PR.

#### Core contributor nomination

Only [maintainers](#maintainer) can be nominated to be core contributors.

Core contributors may either be nominated by another, [Core contributor](#core-contributor), or [lead](#lead).

When nominating a core contributor, a new **private** thread in the `#core-contributors` channel has to be created, with a case and [their contributions](#contributions) of why the person should join as [core contributor](#core-contributor).

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
- Access to the Discord `Maintainers` category and channels that belong to this category.
#### Maintainer nomination

Maintainers may either be nominated by another [maintainer](#maintainer), [Core contributor](#core-contributor), [lead](#lead).

When nominating a new maintainer, a new **private** thread in the `#maintainers` channel has to be created, provide a link to the history of the [person's contributions](#contributions) to the project and a brief explanation of why the person should join as [maintainer](#maintainer).

`#maintainers` is a private channel available only to people who have the `maintainer` role.

The voting period will follow the [voting rules](#voting-rules). An individual becomes a [maintainer](#maintainer) if:
- reaches more than 50% of positive votes among the people who have voting rights;
- if the voting session ends with 50%, the Leads will take the decision;

A maintainer can be self-nominated. They will have to message a [lead maintainer](#lead) privately with links to their contributions to the project. Then, the lead can:
- reject the nomination in place, provide reasons why and possibly some suggestions for the future;
- initiate a voting session among the maintainers;

When the verdict is reached, the thread will be deleted, and a [core contributor](#core-contributor) or [lead](#lead) will reach out to the nominee.

If vote is successful, the person reaching out will ask the nominee if they want to accept the title.

In the event of a rejection, the nominated person will be privately given the requirements they have not met. Details of the discussion, such as the names of those who objected the nomination, will not be disclosed.

### Voting Rules

- The voting sessions are run in Discord, so Maintainers and Core Contributors are expected to have an account.
- Members are required to vote.
- Members may abstain from a vote.
- Members who do not vote within three days will automatically abstain.
- Leads may reduce the three days automatic abstain for urgent decisions.
- Leads reserve the right to veto approval with a publicly disclosed reason.
- It's highly suggested to pair the vote with a reason of the vote, e.g. "I vote in favour because that person writes good code". These reasons will be collected and brought up to the nominee at the end of the voting phase.

### Ownership

One or more people can own certain parts of the codebase. This process is informal, and inclusion could result from substantial contributions or delegation by other members. A maintainer's responsibility is to identify the relevant owners and ensure there's an understanding when it comes to code review.

### Inactivity

There aren't strict requirements and expectations around activity for core contributors and maintainers; although long periods of inactivity must be communicated to the team.

Inactive core contributors or maintainers may have voting and access rights removed and status removed, and given them the _Past maintainer_ status. A core contributor or maintainer may request their voting rights and status back upon sufficient activity.

> [!NOTE]
> The definition of "long periods", and how long an individual needs to stay inactive in order to have their voting rights removed have yet to be decided.

### Contributions

Contributions to the project aren't only limited to code. Contributions come in different forms and ways:
- Documentation
- Code to all the repositories of the [GitHub org][gh-org]
- Support in primary channels of the organization, e.g. GitHub and Discord
- Support in secondary channels, e.g. StackOverflow, etc.
- Website;

### Governance changes

Changes to the governance document must be approved by at least fifty percent of the Core Contributors and a majority of the Leads.

## Project Direction

Project direction and planning is a shared responsibility amongst members. Core contributors are responsible for dictating high-level goals and the project scope that should be adhered to.

### Roadmap

Leads may create a roadmap document to share with the community once the core contributors have set the project's goals.

Roadmaps don't provide dates or deadlines; they only reflect what the core contributors decide to work on and how to spend the resources.

The span of a roadmap should cover one year, but it might vary.

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

## Financial Contributions

There are three ways to fund Biome and its development: sponsorship, funded bounties, and paid
contracts. We strongly prefer and recommend sponsorship, but are willing to facilitate both bounties
and contracting under some conditions.

### Sponsorship

Sponsorship is the first and foremost way of making financial contributions to Biome. Sponsorship can be
done through a one-time donation, or through recurring donations. We offer incentives for certain levels of recurring donations.

To learn more about sponsorship incentives, or to make a donation, please visit our [Open Collective page](https://opencollective.com/biome).
You can also sponsor the project through [GitHub](https://github.com/sponsors/biomejs).

### Community-Funded Bounties

Bounties are the second approach towards funding Biome development. Unlike a sponsorship, bounties are directed towards the implementation of a specific feature. Compared to sponsorship, bounties come with a few risks, both for Biome as a project and for the people or entities offering and collecting the bounty. To minimize these risks, we only sanction bounties under the following conditions:

- Bounties must be pledged through Polar.sh on an issue that is opened in our issue tracker.
  - Unless otherwise specified, Biome reserves 30% of the payment amount, meaning contributors receive 70% of the pledged amount.
  - The commission charged by Polar.sh is split accordingly.
  - We reserve the right to decide on a different split in the future.
- Bounties cannot be opened for bug fixes. Users who wish to see bugs fixed are advised to consider [sponsorship](#sponsorship) instead, in order to give the project a better long-term ability to fix bugs as well.
- If an issue in our issue tracker is not yet open for pledges, the person who wishes to pledge should indicate their desire in the comments of the issue, or do so through the `Community > #funding` channel on Discord. A [Core Contributor](#core-contributor) can then **accept** or **reject** the request for the bounty. Reasons for rejection include, but are not limited to:
  - Implementation of the task would conflict with the project direction or [its values](https://biomejs.dev/internals/philosophy/).
  - Implementation of the task is (currently) technically infeasible.
  - The suggested pledge is deemed inappropriate for the amount of work involved.
- The person making the pledge acknowledges their pledge does not come with a guarantee of work done.
- A task is only completed when a Biome maintainer merges the pull request that closes the task. The person trying to complete the task is responsible for addressing review comments to make sure the work meets the quality standards of the Biome project.
- There is a 6 month expiration on pledges, as managed by Polar.sh.
- Terms and conditions of Polar.sh apply to all pledges.
  - For the FAQ, please see: https://docs.polar.sh/maintainers/faq/
  - For the legal terms, please see: https://polar.sh/legal/terms
- If you are receiving money as part of a bounty, please consider whether you need to pay taxes in your country. Core contributors and Maintainers aren't obligated to give tax advice over the money received.

### Project-Funded Bounties

In some cases, Biome funds bounties on its own issues in order to further the project goals as laid out in its [roadmap](#roadmap). Because Biome is the one funding these issues, it also means **100% of the pledged amount will go to the contributor completing the task**. Project-Funded Bounties have the same conditions applied to them as [Community-Funded Bounties](#community-funded-bounties). But given that these issues are funded using money that ultimately comes from our sponsors, we have additional restrictions in place:

- Implementation of the task funded by Biome must advance our [roadmap](#roadmap) or help with essential infrastructure.
  - The task description must include a clear rationale for why this task is eligible for funding.
- To avoid paying a commission on our own bounties, we do not assign funds through Polar.sh. Instead, we ask contributors to take the money directly from our Open Collective account upon completion of the task.
  - Unfortunately, this means Project-Funded Bounties can only be assigned to [Core Contributors](#core-contributor) at this moment since they have access to our Open Collective account.

Additionally, there is a strict process for assigning Project-Funded Bounties:

- To request Biome funding for a task, a request with a clear rationale must be made in the `Community > #funding` channel on Discord.
  - To make sure our Core Contributors don't have to worry about overlooking Biome-funding-related decisions, requests made on GitHub issues are not sufficient for a funding request.
- If no Core Contributor raises objections within three (3) days, then any Core Contributor can assign the pledge to the issue and assign the requester to it.
  - The waiting period is optional if a Lead approves the request.
  - If an objection is raised, the request is put on hold until a Lead makes the final decision.
- There is a maximum amount of issues with pledges someone can have assigned to them. The limit is 2 for core contributors, and 1 for anyone else. By limiting the amount of issues with pledges that someone can have assigned, we make sure the bounties remain available for others to pick up.

### Paid Contracting

Core Contributors may enter freelance contracts with clients to work on Biome. Such contracts are
between the contributor and their client, so they fall mostly outside the responsibility of the
Biome project. Nevertheless, we can explicitly endorse such contracts under the following
conditions:

- Core Contributors that are open to contracts may be advertised on the project website if they
  please.
- Clients that hire a Core Contributor to work on Biome for an extended period (3 months or more)
  are eligible to the same benefits as a project sponsor. Their sponsorship benefits will be based
  on the monthly fee paid to the Core Contributor.
- If the work that is expected to be delivered does not contribute to Biome's last-published
  roadmap, Biome asks for a 30% fee over the total gross amount, excluding VAT, earned through the
  contract. This is to cover review, project upkeep and continued maintenance of the functionality
  after merging.
  - For work that directly benefits the Biome roadmap, no such fee is required. For work that partly
    or indirectly benefits the roadmap, a customized fee may be negotiated.
  - If the last-published roadmap is more than 12 months old, the Core Contributor is asked to
    consult with the Core team about alignment with the project goals.
  - Fee negotiation may happen either publicly in the `Community > #funding` channel, or privately
    in the `Core > #core-team` channel.
  - Custom fees require the approval of at least one Lead.
- The work may not conflict with the project direction or
  [its values](https://biomejs.dev/internals/philosophy/).
- When a Core Contributor starts or ends a paid contract, it should be announced in the
  `Core > #core-team` channel.
- Biome and its members cannot be held responsible for the performance of any individual
  contributor. We may help clients who are interested in hiring a Biome contributor for a contract
  to get in contact with them, but we cannot guarantee their performance. It is the client's
  responsibility to do due diligence and determine whether the contributor is suitable for the
  assigned contract.

### Fund Allocation

**Funds will be allocated for project-specific services**:
  - Domain registrations
  - Website hosting
  - Password manager to manage secrets and passwords among Lead members

Additionally, we may use funds for assigning [Project-Funded Bounties](#project-funded-bounties).

#### Expenses

Core contributors are allowed to claim expenses related to conferences where they talk about Biome, or Biome workshops that they run. Expenses that can be claimed include, but are not limited to: trip (plane, train, etc.), meals, transport, parking.
  - The person will have to provide proof of attendance (badge, video, etc.);
  - Expenses will have to be uploaded via the open collective website.
  - Only the expenses that are afforded in the days of the workspace/conference are accepted:
    - If a conference lasts multiple days and the member decides to attend *both* days, and the member speaks only the first day, only the expenses of the first day should be taken in consideration. In this example, if the member booked three nights in a hotel to attend these two days, the expense should be divided by 3  - three nights - and only expense 2 days: night before the conference, plus the night of the day the member speaks.
    - If a conference lasts one day and the member decides to make their performance longer, all the expenses afforded in the other days won't be accepted. Only 2 nights of accommodation are accepted.

Expenses must be approved by at least one [Lead](#lead). In the presence of one single Lead, the Lead will ask one Core Contributor to fact-check the honesty of expenses, and leave a comment in the Open Collective expense page. In case of multiple leads, the Lead submitting the expense must seek the approval of at least one other Lead.

> [!NOTE]
> Other usage of funds has yet to be decided.

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

[gh-org]: https://github.com/biomejs
[discord]: https://biomejs.dev/chat
