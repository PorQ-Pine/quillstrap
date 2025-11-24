1. Reopen this repo in devcontainer, vscode, let the docker to build itself
2. Then, click the First time setup button on the bottom toolbar
3. Then click the Welcome back button on the toolbar below. It will download everything
4. Run something like `rq -a --just-built-it -b rootfs` to auto build rootfs. Make sure to read logs, for example, they will warn you that rootfs is an additive build. So for a fresh rootfs build, you need to clean it first, then get it again, then run this command
