# dimple
`dimple` is a prototype of an app server in Rust with an embedded Rhai interpreter for HTTP services.

The idea here is that a software engineer for SaaS apps typically acquires some familiarity with a far-ranging gamut of technologies.  For development, technologies may include:

* App server setup (Tomcat, JBoss, WebSphere, etc)
* HTTP protocol
* JSON, HTML, etc.
* Some programming language.

And for deployment:

* Containerization (Docker)
* Kubernetes, Helm Charts, and YAML
* CI / CD (for example, syntax for `.gitlab.ci.yml` files)

The list probably goes on.

`dimple` is a response to the question of:  Can all this just be simpler?

## Background
In the 1990s, Prof. John Ousterhout (Berkeley, Stanford professor) developed Tcl -- the "Tool Command Language" -- for embedded control in C/C++ application logic.  Tcl was a cool way to externalize programmatic control of software without having to re-build, re-compile, and re-deploy your application.

In the same way, the `Rhai` interpreter provides for embedded control in Rust apps.  The thinking here is that deployment considerations would be contained within the configuration and deployment of the Rust app server.  Need greater redundancy in kubernetes to support greater service consumption?  Spell out redundancy rules in Helm Charts for the Rust app server.  Need more hardware like processors, network bandwidth, or memory?  Again, make all those rules specific to the Rust app server.

In short, the idea here is that service authors would return to the art of programming and defer operational considerations to, well, those who actually do operations.

## Rhai interpreter
Just as Tcl/Tk was embedded in C/C++ applications, the Rhai interpreter is embedded in the `dimple` app server.  All that happens for now is that any Rhai scripts archived under `scripts` are ingested by `dimple` and dynamically augment `dimple` with additional HTTP services.  Of course, with any real app server, there would be a huge number of additional problems -- dealing with security, accessing data resources, etc.  In truth, others have thought through these challenges and the current service paradigm likely is not going to shift much.