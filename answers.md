ps1
===

1
-
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:26.0) Gecko/20100101 Firefox/26.0

The "user-agent:" prefix idicates that what follows is a description of the client environment used to access the web server.

The environment information may be used by a web server to serve specialized content.

Apparently[0], most clients identify themselves as Mozilla for historical reasons.  Gecko is the layout engine of Firefox.  Firefox is the name of the browser.  The numerical information after the "/"s are versions.

[0] http://webaim.org/blog/user-agent-string-history/

2
-
In general, shared mutable state between parallel executing tasks (such as threads) requires a mutual exclusion lock be in place when the state is being mutated.  From what I can tell thus far, the Rust language appears to try to avoid shared state as much as possible; for example, lending certain pointers results in the lenders reference to the object being frozen (so it cannot be changed).  The Rust documentation also states explicitly that Rust has a goal of minimizing concurrencybugs, and also implies that shared state is to be avoided if possible.  With these design goals in mind, I would speculate that the Rust designers thought the "traditional" model, of multiple tasks and developers having the onus of consistently protecting shared memory, was insufficient.
