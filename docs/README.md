# Configuration

The configuration file to sysmon is written in [TOML](toml).

[toml]: https://github.com/toml-lang/toml

#### threads = &lt;number&gt;

How many threads sysmon should use.

#### thread_per_cpu = &lt;bool&gt;

If number of `threads` is configured per cpu or not.

#### [input.&lt;id&gt;]

Configure an input plugin with the id `<id>`.

Example:

```toml
[input.cpu]
type = "cpu"

[input."website frontend poller"]
type = "http_poller"
```
