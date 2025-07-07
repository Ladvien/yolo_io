Here’s a general-purpose, bulletized list of what an agent should do — independent of the specific application:

---

### ✅ General Agent Responsibilities

* **Understand the Mission**: Clearly grasp the goal of the feature or system you’re working on.
* **Design Before You Code**: Think through architecture, interfaces, and data flow before implementing.
* **Use Idiomatic Code**: Follow the conventions and best practices of the language you’re working in.
* **Write Modular Code**: Decompose functionality into focused, reusable components.
* **Avoid Global State**: Use explicit dependency injection or configuration objects for shared context.
* **Leave Traceable Notes**:

  * Use `# TODO:` for unfinished logic.
  * Use `# AGENT NOTE:` for decisions, warnings, or coordination needs.
* **Document Public Interfaces**: Include docstrings or comments on exported functions, classes, and config structures.
* **Test All Critical Paths**:

  * Write unit tests for all logic-heavy functions.
  * Add integration tests when functionality spans components.
* **Commit Thoughtfully**:

  * Keep commits small and self-contained.
  * Use descriptive commit messages (what changed and why).
* **Follow Version Control Hygiene**:

  * Regularly pull and rebase against `main`.
  * Avoid large, unreviewable diffs.
* **Measure Performance**: Monitor and improve latency, memory usage, or throughput where relevant.
* **Use Linting & Formatters**:

  * Enforce automatic formatting and linting to reduce stylistic churn.
* **Keep Dependencies Minimal**:

  * Don’t add libraries unless the benefits clearly outweigh the cost.
* **Think About Extensibility**:

  * Design with future use cases and modularity in mind.
* **Collaborate Openly**:

  * Leave inline comments for other agents.
  * Tag assumptions or blocked items clearly.
* **Build for Change**:

  * Design your code so that config—not code—drives behavior when possible.

