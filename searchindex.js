Object.assign(window.search, {"doc_urls":["overview.html#overview","overview.html#developer-experience","overview.html#development","design.html#design","design.html#hooks","design.html#install","design.html#other-hooks","design.html#the-lucky-daemon"],"index":{"documentStore":{"docInfo":{"0":{"body":51,"breadcrumbs":1,"title":1},"1":{"body":41,"breadcrumbs":2,"title":2},"2":{"body":26,"breadcrumbs":1,"title":1},"3":{"body":31,"breadcrumbs":1,"title":1},"4":{"body":20,"breadcrumbs":1,"title":1},"5":{"body":48,"breadcrumbs":1,"title":1},"6":{"body":41,"breadcrumbs":1,"title":1},"7":{"body":24,"breadcrumbs":2,"title":2}},"docs":{"0":{"body":"Lucky is a work-in-progress framwork for writing Juju charms. It is being designed specifically to support writing Docker-powered charms easily. In the future the framework could be useful for more than Docker charms, but development is currently focused on providing facilities to run and configure Docker containers. We want Lucky to as easy to use as possible and be very well documented. We will focus on putting the developer's experience first, starting small and adding features as they become necessary or useful.","breadcrumbs":"Overview","id":"0","title":"Overview"},"1":{"body":"The Lucky framework will provide a charm template that will contain the boilerplate necessary to get started writing a charm with the framework, and it will provide a CLI that will be used by the charm code to interact with Docker and with the Juju controller. We will be focusing on making it easy to write charms in bash or any other shell language, but, because the framework itself provides a CLI for interacting with it, it is possible to write charm code in Python or any other executable format.","breadcrumbs":"Developer Experience","id":"1","title":"Developer Experience"},"2":{"body":"We are very early in development. In fact, we haven't started writing any code yet! We are currently working on this documentation and on outlining the design plan before we start work on coding a proof-of-concept. If you have any questions or thoughts don't hesitate to open an issue.","breadcrumbs":"Development","id":"2","title":"Development"},"3":{"body":"The Lucky framework will be implemented in Rust and will consist of a daemon that runs as a part of the charm and a CLI that will be used by scripts to communicate to the daemon. The overall design can be seen in the diagram below. charm-framework-diagram To explain how the design works we will go through the different components step by step.","breadcrumbs":"Design","id":"3","title":"Design"},"4":{"body":"Just like every Juju charm, charms built with the Lucky framework implement a number of different hooks that the Juju controller will execute. These hooks will not be implemented by the developer but will be provided by the Lucky charm template.","breadcrumbs":"Hooks","id":"4","title":"Hooks"},"5":{"body":"The install hook will first download one of our automated builds of the Lucky framework, which will be a standalone Rust executable. The install hook will be sure to download the binary appropriate to the platform architecture. After downloading the Lucky binary it will run the Lucky daemon. The Lucky binary also acts as the CLI that is used to communicate with the running daemon. The install hook will use this CLI to tell the daemon to execute the developer's install hooks. This will be explained in more detail later.","breadcrumbs":"Install","id":"5","title":"Install"},"6":{"body":"All of the other hooks are scripts that simply use the Lucky CLI to tell the Lucky daemon that it needs to execute the code related to the triggered hook. Note: On a somewhat related note, in the event that something goes wrong that somehow kills the daemon process, when the next hook is triggered by Juju, the CLI will detect that the daemon has stopped and will start it again before notifying the daemon of the hook.","breadcrumbs":"Other Hooks","id":"6","title":"Other Hooks"},"7":{"body":"The Lucky daemon will be run by the charm and will continue running for the whole duration that the charm is installed. The daemon will listen on a Unix socket for commands that will be sent to it by the Lucky CLI. The Lucky daemon and CLI are provided by the same binary.","breadcrumbs":"The Lucky Daemon","id":"7","title":"The Lucky Daemon"}},"length":8,"save":true},"fields":["title","body","breadcrumbs"],"index":{"body":{"root":{"a":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}},"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{},"g":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":1.0}}}}},"df":0,"docs":{}},"p":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}}},"r":{"c":{"df":0,"docs":{},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":1.0}}}}}}},"b":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"h":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":1,"docs":{"0":{"tf":1.0}},"f":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":2,"docs":{"2":{"tf":1.0},"6":{"tf":1.0}}}}},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"i":{"df":0,"docs":{},"n":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"5":{"tf":1.7320508075688772},"7":{"tf":1.0}}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{},"t":{"df":1,"docs":{"4":{"tf":1.0}}}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"df":5,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":2.23606797749979},"3":{"tf":1.4142135623730951},"4":{"tf":1.7320508075688772},"7":{"tf":1.4142135623730951}}}}},"df":0,"docs":{}},"l":{"df":0,"docs":{},"i":{"df":5,"docs":{"1":{"tf":1.4142135623730951},"3":{"tf":1.0},"5":{"tf":1.4142135623730951},"6":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}}},"o":{"d":{"df":0,"docs":{},"e":{"df":3,"docs":{"1":{"tf":1.4142135623730951},"2":{"tf":1.4142135623730951},"6":{"tf":1.0}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"7":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{},"u":{"df":0,"docs":{},"n":{"df":2,"docs":{"3":{"tf":1.0},"5":{"tf":1.0}}}}},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"n":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}}}},"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"t":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"u":{"df":1,"docs":{"7":{"tf":1.0}}}}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":2,"docs":{"1":{"tf":1.0},"4":{"tf":1.0}}}}}}}},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"2":{"tf":1.0}}}}}}}}},"d":{"a":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":4,"docs":{"3":{"tf":1.4142135623730951},"5":{"tf":1.7320508075688772},"6":{"tf":2.0},"7":{"tf":2.0}}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"n":{"df":3,"docs":{"0":{"tf":1.0},"2":{"tf":1.0},"3":{"tf":1.7320508075688772}}}}}},"t":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"2":{"tf":1.4142135623730951},"4":{"tf":1.0}},"e":{"df":0,"docs":{},"r":{"'":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}},"df":0,"docs":{}}}}}}}}},"i":{"a":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"m":{"df":1,"docs":{"3":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"f":{"df":0,"docs":{},"f":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":2,"docs":{"3":{"tf":1.0},"4":{"tf":1.0}}}}}}},"o":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":2,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"2":{"tf":1.0}}}}}}}},"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}},"w":{"df":0,"docs":{},"n":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"a":{"d":{"df":1,"docs":{"5":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"u":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"7":{"tf":1.0}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"2":{"tf":1.0}}}}},"s":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"x":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":4,"docs":{"1":{"tf":1.0},"4":{"tf":1.0},"5":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}},"df":0,"docs":{}},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"l":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":2,"docs":{"3":{"tf":1.0},"5":{"tf":1.0}}}}},"df":0,"docs":{}}}}},"f":{"a":{"c":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"t":{"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"i":{"df":0,"docs":{},"r":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}}}}},"o":{"c":{"df":0,"docs":{},"u":{"df":1,"docs":{"0":{"tf":1.0}},"s":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"r":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":5,"docs":{"0":{"tf":1.0},"1":{"tf":1.7320508075688772},"3":{"tf":1.4142135623730951},"4":{"tf":1.0},"5":{"tf":1.0}}}}}}},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"g":{"df":0,"docs":{},"o":{"df":1,"docs":{"3":{"tf":1.0}},"e":{"df":1,"docs":{"6":{"tf":1.0}}}}},"h":{"a":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":3,"docs":{"4":{"tf":1.7320508075688772},"5":{"tf":2.0},"6":{"tf":2.23606797749979}}}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":2,"docs":{"3":{"tf":1.0},"4":{"tf":1.4142135623730951}}}}}}}}}},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"l":{"df":2,"docs":{"5":{"tf":2.23606797749979},"7":{"tf":1.0}}}},"df":0,"docs":{}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"a":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"s":{"df":0,"docs":{},"s":{"df":0,"docs":{},"u":{"df":1,"docs":{"2":{"tf":1.0}}}}},"t":{"df":0,"docs":{},"s":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"f":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"j":{"df":0,"docs":{},"u":{"df":0,"docs":{},"j":{"df":0,"docs":{},"u":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"4":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}}},"k":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"l":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"7":{"tf":1.0}}}}}}},"u":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"i":{"df":7,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.0},"3":{"tf":1.0},"4":{"tf":1.4142135623730951},"5":{"tf":2.0},"6":{"tf":1.4142135623730951},"7":{"tf":2.0}}}}},"df":0,"docs":{}}},"m":{"a":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"e":{"d":{"df":1,"docs":{"6":{"tf":1.0}}},"df":0,"docs":{}},"x":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}},"o":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}},"u":{"df":0,"docs":{},"m":{"b":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"4":{"tf":1.0}}}}},"df":0,"docs":{}}}},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"5":{"tf":1.0}}},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}}}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"3":{"tf":1.0}}}},"df":0,"docs":{},"v":{"df":0,"docs":{},"i":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"3":{"tf":1.0}}}}},"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}},"t":{"df":0,"docs":{},"f":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"l":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"w":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"r":{"df":0,"docs":{},"o":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"o":{"df":0,"docs":{},"f":{"df":1,"docs":{"2":{"tf":1.0}}}},"v":{"df":0,"docs":{},"i":{"d":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.7320508075688772},"4":{"tf":1.0},"7":{"tf":1.0}}},"df":0,"docs":{}}}}},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"y":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"q":{"df":0,"docs":{},"u":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}}}}}}}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"u":{"df":0,"docs":{},"n":{"df":4,"docs":{"0":{"tf":1.0},"3":{"tf":1.0},"5":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"3":{"tf":1.0},"5":{"tf":1.0}}}}}},"s":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":1,"docs":{"7":{"tf":1.0}}}}},"c":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":2,"docs":{"3":{"tf":1.0},"6":{"tf":1.0}}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"3":{"tf":1.0}}}},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"7":{"tf":1.0}}}}},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}},"m":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"o":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":1,"docs":{"7":{"tf":1.0}}}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"6":{"tf":1.0}}}}},"t":{"df":0,"docs":{},"h":{"df":1,"docs":{"6":{"tf":1.0}}}},"w":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"p":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"i":{"df":0,"docs":{},"f":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"t":{"a":{"df":0,"docs":{},"n":{"d":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"t":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"2":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"p":{"df":1,"docs":{"3":{"tf":1.4142135623730951}}}},"o":{"df":0,"docs":{},"p":{"df":1,"docs":{"6":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"p":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":2,"docs":{"5":{"tf":1.0},"6":{"tf":1.0}}}},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"t":{"df":2,"docs":{"1":{"tf":1.0},"4":{"tf":1.0}}}},"df":0,"docs":{}}}}},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}}}}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":1,"docs":{"3":{"tf":1.0}}}}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}}}}}},"u":{"df":0,"docs":{},"n":{"df":0,"docs":{},"i":{"df":0,"docs":{},"x":{"df":1,"docs":{"7":{"tf":1.0}}}}},"s":{"df":5,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.0},"3":{"tf":1.0},"5":{"tf":1.4142135623730951},"6":{"tf":1.0}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"2":{"tf":1.0}}}}}},"w":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":1,"docs":{"7":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":3,"docs":{"0":{"tf":1.0},"2":{"tf":1.4142135623730951},"3":{"tf":1.0}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":3,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.7320508075688772},"2":{"tf":1.0}}}}},"o":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}},"breadcrumbs":{"root":{"a":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}},"d":{"df":1,"docs":{"0":{"tf":1.0}}},"df":0,"docs":{},"g":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":1.0}}}}},"df":0,"docs":{}},"p":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}}},"r":{"c":{"df":0,"docs":{},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":1.0}}}}}}},"b":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"h":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":1,"docs":{"0":{"tf":1.0}},"f":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":2,"docs":{"2":{"tf":1.0},"6":{"tf":1.0}}}}},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"i":{"df":0,"docs":{},"n":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"5":{"tf":1.7320508075688772},"7":{"tf":1.0}}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":1,"docs":{"5":{"tf":1.0}}},"df":0,"docs":{},"t":{"df":1,"docs":{"4":{"tf":1.0}}}}}}},"c":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"df":5,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":2.23606797749979},"3":{"tf":1.4142135623730951},"4":{"tf":1.7320508075688772},"7":{"tf":1.4142135623730951}}}}},"df":0,"docs":{}},"l":{"df":0,"docs":{},"i":{"df":5,"docs":{"1":{"tf":1.4142135623730951},"3":{"tf":1.0},"5":{"tf":1.4142135623730951},"6":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}}},"o":{"d":{"df":0,"docs":{},"e":{"df":3,"docs":{"1":{"tf":1.4142135623730951},"2":{"tf":1.4142135623730951},"6":{"tf":1.0}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"n":{"d":{"df":1,"docs":{"7":{"tf":1.0}}},"df":0,"docs":{}}},"df":0,"docs":{},"u":{"df":0,"docs":{},"n":{"df":2,"docs":{"3":{"tf":1.0},"5":{"tf":1.0}}}}},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"n":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}}}},"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"t":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"u":{"df":1,"docs":{"7":{"tf":1.0}}}}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":2,"docs":{"1":{"tf":1.0},"4":{"tf":1.0}}}}}}}},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"2":{"tf":1.0}}}}}}}}},"d":{"a":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":4,"docs":{"3":{"tf":1.4142135623730951},"5":{"tf":1.7320508075688772},"6":{"tf":2.0},"7":{"tf":2.23606797749979}}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"n":{"df":3,"docs":{"0":{"tf":1.0},"2":{"tf":1.0},"3":{"tf":2.0}}}}}},"t":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.4142135623730951},"2":{"tf":1.7320508075688772},"4":{"tf":1.0}},"e":{"df":0,"docs":{},"r":{"'":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}},"df":0,"docs":{}}}}}}}}},"i":{"a":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"m":{"df":1,"docs":{"3":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"f":{"df":0,"docs":{},"f":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":2,"docs":{"3":{"tf":1.0},"4":{"tf":1.0}}}}}}},"o":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":2,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"2":{"tf":1.0}}}}}}}},"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}},"w":{"df":0,"docs":{},"n":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"a":{"d":{"df":1,"docs":{"5":{"tf":1.7320508075688772}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"u":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"7":{"tf":1.0}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"2":{"tf":1.0}}}}},"s":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"x":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":4,"docs":{"1":{"tf":1.0},"4":{"tf":1.0},"5":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}},"df":0,"docs":{}},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.4142135623730951}}}}},"l":{"a":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":2,"docs":{"3":{"tf":1.0},"5":{"tf":1.0}}}}},"df":0,"docs":{}}}}},"f":{"a":{"c":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}},"t":{"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{},"e":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"i":{"df":0,"docs":{},"r":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}}}}},"o":{"c":{"df":0,"docs":{},"u":{"df":1,"docs":{"0":{"tf":1.0}},"s":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"r":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":5,"docs":{"0":{"tf":1.0},"1":{"tf":1.7320508075688772},"3":{"tf":1.4142135623730951},"4":{"tf":1.0},"5":{"tf":1.0}}}}}}},"w":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"g":{"df":0,"docs":{},"o":{"df":1,"docs":{"3":{"tf":1.0}},"e":{"df":1,"docs":{"6":{"tf":1.0}}}}},"h":{"a":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"'":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":3,"docs":{"4":{"tf":2.0},"5":{"tf":2.0},"6":{"tf":2.449489742783178}}}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":2,"docs":{"3":{"tf":1.0},"4":{"tf":1.4142135623730951}}}}}}}}}},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"l":{"df":2,"docs":{"5":{"tf":2.449489742783178},"7":{"tf":1.0}}}},"df":0,"docs":{}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"a":{"c":{"df":0,"docs":{},"t":{"df":1,"docs":{"1":{"tf":1.4142135623730951}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"s":{"df":0,"docs":{},"s":{"df":0,"docs":{},"u":{"df":1,"docs":{"2":{"tf":1.0}}}}},"t":{"df":0,"docs":{},"s":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"f":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"j":{"df":0,"docs":{},"u":{"df":0,"docs":{},"j":{"df":0,"docs":{},"u":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"4":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}}},"k":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"l":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":0,"docs":{},"u":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"7":{"tf":1.0}}}}}}},"u":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"i":{"df":7,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.0},"3":{"tf":1.0},"4":{"tf":1.4142135623730951},"5":{"tf":2.0},"6":{"tf":1.4142135623730951},"7":{"tf":2.23606797749979}}}}},"df":0,"docs":{}}},"m":{"a":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":1,"docs":{"1":{"tf":1.0}}}}},"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":2,"docs":{"0":{"tf":1.0},"5":{"tf":1.0}}}}}},"n":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"e":{"d":{"df":1,"docs":{"6":{"tf":1.0}}},"df":0,"docs":{}},"x":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}},"o":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}},"i":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}},"u":{"df":0,"docs":{},"m":{"b":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"4":{"tf":1.0}}}}},"df":0,"docs":{}}}},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"5":{"tf":1.0}}},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}}}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"3":{"tf":1.0}}}},"df":0,"docs":{},"v":{"df":0,"docs":{},"i":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.4142135623730951}}}}}}}}}},"p":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"3":{"tf":1.0}}}}},"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}},"t":{"df":0,"docs":{},"f":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"m":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"l":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}}}},"df":0,"docs":{}}}},"w":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"r":{"df":0,"docs":{},"o":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"o":{"df":0,"docs":{},"f":{"df":1,"docs":{"2":{"tf":1.0}}}},"v":{"df":0,"docs":{},"i":{"d":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.7320508075688772},"4":{"tf":1.0},"7":{"tf":1.0}}},"df":0,"docs":{}}}}},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}},"y":{"df":0,"docs":{},"t":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"q":{"df":0,"docs":{},"u":{"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"2":{"tf":1.0}}}}}}}}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"u":{"df":0,"docs":{},"n":{"df":4,"docs":{"0":{"tf":1.0},"3":{"tf":1.0},"5":{"tf":1.4142135623730951},"7":{"tf":1.4142135623730951}}},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"3":{"tf":1.0},"5":{"tf":1.0}}}}}},"s":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":1,"docs":{"7":{"tf":1.0}}}}},"c":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":2,"docs":{"3":{"tf":1.0},"6":{"tf":1.0}}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":1,"docs":{"3":{"tf":1.0}}}},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"7":{"tf":1.0}}}}},"h":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}},"m":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}},"o":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"e":{"df":0,"docs":{},"t":{"df":1,"docs":{"7":{"tf":1.0}}}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":1,"docs":{"6":{"tf":1.0}}}}},"t":{"df":0,"docs":{},"h":{"df":1,"docs":{"6":{"tf":1.0}}}},"w":{"df":0,"docs":{},"h":{"a":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}},"df":0,"docs":{}}}}}},"p":{"df":0,"docs":{},"e":{"c":{"df":0,"docs":{},"i":{"df":0,"docs":{},"f":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{}}},"t":{"a":{"df":0,"docs":{},"n":{"d":{"a":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"r":{"df":0,"docs":{},"t":{"df":4,"docs":{"0":{"tf":1.0},"1":{"tf":1.0},"2":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"p":{"df":1,"docs":{"3":{"tf":1.4142135623730951}}}},"o":{"df":0,"docs":{},"p":{"df":1,"docs":{"6":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"p":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}}}},"r":{"df":0,"docs":{},"e":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"t":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":2,"docs":{"5":{"tf":1.0},"6":{"tf":1.0}}}},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"l":{"a":{"df":0,"docs":{},"t":{"df":2,"docs":{"1":{"tf":1.0},"4":{"tf":1.0}}}},"df":0,"docs":{}}}}},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":1,"docs":{"2":{"tf":1.0}}}}}}},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"df":0,"docs":{},"h":{"df":1,"docs":{"3":{"tf":1.0}}}}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"g":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}}}}}},"u":{"df":0,"docs":{},"n":{"df":0,"docs":{},"i":{"df":0,"docs":{},"x":{"df":1,"docs":{"7":{"tf":1.0}}}}},"s":{"df":5,"docs":{"0":{"tf":1.7320508075688772},"1":{"tf":1.0},"3":{"tf":1.0},"5":{"tf":1.4142135623730951},"6":{"tf":1.0}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":2,"docs":{"0":{"tf":1.0},"2":{"tf":1.0}}}}}},"w":{"a":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"0":{"tf":1.0}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"l":{"df":0,"docs":{},"e":{"df":1,"docs":{"7":{"tf":1.0}}}}}},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"df":3,"docs":{"0":{"tf":1.0},"2":{"tf":1.4142135623730951},"3":{"tf":1.0}}}}},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"df":3,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.7320508075688772},"2":{"tf":1.0}}}}},"o":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}},"title":{"root":{"d":{"a":{"df":0,"docs":{},"e":{"df":0,"docs":{},"m":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"7":{"tf":1.0}}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"i":{"df":0,"docs":{},"g":{"df":0,"docs":{},"n":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":2,"docs":{"1":{"tf":1.0},"2":{"tf":1.0}}}}}}}}},"df":0,"docs":{},"e":{"df":0,"docs":{},"x":{"df":0,"docs":{},"p":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":1,"docs":{"1":{"tf":1.0}}}}}}}},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":2,"docs":{"4":{"tf":1.0},"6":{"tf":1.0}}}}}},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}}}}},"l":{"df":0,"docs":{},"u":{"c":{"df":0,"docs":{},"k":{"df":0,"docs":{},"i":{"df":1,"docs":{"7":{"tf":1.0}}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"v":{"df":0,"docs":{},"e":{"df":0,"docs":{},"r":{"df":0,"docs":{},"v":{"df":0,"docs":{},"i":{"df":0,"docs":{},"e":{"df":0,"docs":{},"w":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}}}}}}},"pipeline":["trimmer","stopWordFilter","stemmer"],"ref":"id","version":"0.9.5"},"results_options":{"limit_results":30,"teaser_word_count":30},"search_options":{"bool":"OR","expand":true,"fields":{"body":{"boost":1},"breadcrumbs":{"boost":1},"title":{"boost":2}}}});