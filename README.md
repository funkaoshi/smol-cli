# smol-cli

A small CLI for posting to smol.pub that mimics the same interface as the
smolpub.sh shell script.

## Usage

This tool lets you send properly formatted posts to https://smol.pub, a simple
blogging platform that publishes to the Web, [Gemini][], and Gopher.

A post is a [gemtext][] file that begins with a title, followed by a blank
line, followed by the contents of the post. A post should be formatted using
[gemtext][]. The following is an example post:

```
# Plan Files?

A long time ago you would use finger to see if other users were logged into a
system. They could share .plan or .project files that were accessible via
finger. One way I could imagine using this service is to publish short messages
about what i'm up to, at my computer, since that's really the most convenient
way to post? I started converting the smolpub.sh script into a small rust
program. I'll finsih that soon.
```

The file name of the post becomes the posts slug: both will be unique.

You need to

Once you have saved your post you can send it off to smol.pub with the
following command: `smol-cli <post>`. If the post doesn't exist, it will be
created. If it does exist, it will be updated.

This is a pretty simple and probably pointless program, mostly an excuse to
practice writing Rust code.

## Installation

### crates.io

You can install this tool from crates.io: `cargo install smol-cli`. Remove with
`cargo uninstall smol-cli`.

### Source

You can build the tool with the command `cargo build --release`. Once built you
can move the file to a place on your file system that is on your `$PATH`,
typically `/usr/local/bin`.

```
cargo build --release
sudo cp target/release/smol-cli /usr/local/bin
```

If you decide you hate this program, you can simply delete the binary and your
authorization config file.

```
rm /usr/local/bin/smol-cli
rm ~/.config/.smolpub
```

[gemini]: https://geminiprotocol.net/
[gemtext]: https://geminiprotocol.net/docs/gemtext-specification.gmi
