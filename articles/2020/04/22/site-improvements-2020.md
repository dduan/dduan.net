# Site Improvements 2020
2020-04-22T21:58:03-07:00
tags: Rust, HTML, CSS, Ruby, Jekyll, Markdown
I took back my website.

I took it back from the claws of Jekyll and Ruby. I took it back from some
random template among a few that were immediately available. I took it back
from my own ignorance of modern web technology.

This time, I rewrote the hole effing thing from scratch.

### The Why

The [last time][] I ended with

> I wrote the most Ruby in my life today. Yay?

That question mark turned out to be prescient. Ruby is not my thing. No
judgement against the language per ce. But the Ruby ecosystem is not friendly to
a casual user who needs it once every few months. No really, when I write an
article, it's a toss-up whether I can deploy it without being forced to mess
with Ruby/Gems/Jekyll/Homebrew etc. I'm almost certain there's a set of best
practices I could learn to improve this. But it'd be a skill that I barely need
and probably forget a few times. Meanwhile I just want to translate a new
Markdown file to HTML and put it on Github.

As a teen, one of my favorite things about the web is how accessible it is.
I could sit in front of anyone's computer, open Notepad.exe and type in some
tags, and open the file with a browser to see the result. That simple bootstrap
process, however repetitive, never got old for me.

In the last decade, my professional work is focused on native, mobile
applications. This experience biased me in a few ways. "Native" made me
appreciate the closeness to "the metal": you have an OS; you get the executable;
, you launch strace, and boom, everything the OS thinks what your code should do
is revealed to you. "Mobile" forced me to see the reality: desktop experience
has become a niche. It's nowhere near as important as it was prior to the
iPhone. Not making your software run well on mobile devices is a particular kind
of choice that come with some severe trade-offs.

And, finally, I subscribe to the idea that *plain text is supreme*. Yes, even
more supreme than the web. This website is a derivative of the articles I write
in Markdown. Plain text as a format will out-last the web in the long run (not
necessarily _these_ texts). So it really bothers me when I have to put "front
matter" in YAML/TOML/whatever in front of the real Markdown. Yes, if you want
reasonable HTMLs, these metadata is necessary. But the text is *Supreme*. The
text is to be readable directly. Succumbing the supreme to the derivative is
*wrong*.

Okay, so where does all that leave me? In this iteration of this website,
I wrote every single line of CSS and HTML (there's barely any JavaScript) which
look decent on mobile. It's generated by portable Linux and Darwin executables
that are part of the website. As long as the OSes stay relatively stable, the
site will build without any dependencies, in a matter of milliseconds. The best
part?  It'll stay like this unless I wish it otherwise.

### The Frontend

A few words on the design of the site.

This site gained the concept of "links" the [last time][]. It has since become
clear that I don't use this feature (blame Twitter). It's gone, for now. The old
"about" page is replaced by the home page.

When I decide the site needed a rewrite, I fantasized a place with only HTML.
Perhaps users who want a better reading experience can simply put it in Reader
mode. Alas, if browsers (with the exception of Safari) implemented automatic
dark mode with a [color-scheme meta][] tag, it'd almost be a working idea.

So, that's my starting point.

The site is a list of articles organized by tags and dates, and a few web pages.
_I want you to read the site, not navigate it_. So text is the point. It's the
only design element. There are has 2 fonts and 2 text-color (not counting
highlighted code). Links are always underscored (because you can't hover in
mobile browsers to find something clickable).

The site is responsive to mobile layout, and dark mode. It's aware that it could
be added to the home screen of a mobile device, or linked to some external site
that wants to generate a preview.

I did end up using [solarized][] theme for code highlighting, which lead to 2
static CSS files. Other than that, all CSS and HTML are hand-written. There's no
build step for them, farm to table, Vim to your browser.

It turned out standardized CSS variable is game-changing. Combined with media
query, I barely needed any class to support dark mode/mobile layout. More
importantly, it makes my programmer brain happy. Oh, yes I'm talking about them
here because this is the first time I truly attempted to catch up since they
were introduced to the world. It's freeing to let go of constrains of an
read-to-use theme, or some CSS frameworks.

### The Backend

This site is a collection of static files. The so-called backend is a program
that assembles these files from some HTML templates and Markdown files. In the
past, this program had been [Jekyll][]. This time around, I replaced it with
some Rust code.

Boy, this thing is cool, if I say so myself. I'm going to refer to it as "the
generator".

The biggest "feature" is the fact that it doesn't pretend to be re-usable.
The generator is as part of this site as the articles.

Thanks to Rust, the programs are built into executable binaries. On macOS, it
requires libSystem to run. On Linux, I can (and prefer) build with [musl][]. The
binaries for these two OSes are checked in with the content of the site. So
it requires zero installation to "build" the site. (I may need to
include a 3rd executable for the upcoming ARM-based Macs soon).

The generator spits out the final content of the website. It's deployed without
further modification.

The build process is pretty fast. As of this writing, it averages around 250 ms.
I could probably make it faster by avoiding some repeated reads when it comes to
article inputs.

The generator handles HTML/XML templating with a library named [Askama][].
Learning it had been an eye opening experience. Askama is built atop Rust's
macro system. For each template (e.g. web page), it requires users to write
a Rust data structure that fulfills its variable requirements.  Here's the
kicker: in this data structure, you *cannot* miss any variable the corresponding
template requires.  When you do, the Rust project won't compile! Rust's tooling
is so good that these errors were surfaced in my editor as I wrote this part
in real time. This level of type-safety for template language felt like magic.

Syntax highlighting is powered by [syntect][], the library behind [bat][]. But
the interesting bit here is how the syntax definitions are embedded within the
final executable. To support a particular syntax, syntect takes the syntax's
SublimeText definition. So this is a configurable, extendible system. The
generator includes 118 syntax definitions. Uncompressed, their files take up 5MB
of disk space. As one can imagine, loading 5MB from 118 files each time the
program runs is quite slow. Turns out, Rust has a standard library macro
[include_bytes!][] that solves this problem. [include_bytes!][] embeds
contents of a file as literals in source code, as if it's hand-written. syntect
takes advantage of this feature by supporting serialization of its in-memory
representation of syntaxes into bytes, and accepting in-line byte literals in
reverse in order to create these representations. This system solves two
problems for me:

1. I no longer need to ship the SublimeText syntax files along with the
   generator.
2. The generator doesn't need to perform all that disk I/O, so it runs
   significantly faster.

Overall, the Rust ecosystem delivered. Although a lot of library are still in
rough shape/early stage, there usually are multiple alternatives for roughly the
same purpose. Gluing a few of them together for this generator project had been
fun.

Finally, let's talk metadata, I can't realistically manage to generate the site
without them.  Articles and static pages each define their URL by their file
locations relative to the root. For example, `/articles/2020/04/23/hello.md`
means the URL is `/2020/04/23/hello/`. Each article still has a front matter.
But I made the text version look as "natural" as possible. There's no markers
for beginning and end of the metadata since I know exactly what's needed. The
title is marked as an H1. Date is in RFC3339 format. Tags are comma-separated
values. So an example of an article's beginning looks like:

```markdown
# Site Improvements 2020
2020-04-22T21:58:03-07:00
tags: Rust, HTML, CSS, Ruby, Jekyll, Markdown

I took back my website...
```

Text supremacy! Glory!

### Onward

Over the course of roughly 15 years, I've had blogs of quite a few variates.
I wish I'd done a better job archiving them as I moved on from one to the next.
This is not the first time I attempted to gain full control over the theming,
generating, hosting, and deployment of a site. But I'm hopeful that it'll last
longer. You could say that's what I optimized for. And who knew, maybe my
experience from past failures counts for something.

Here's to less stressful deploys and more writing!

[last time]: /2017/01/16/site-changes/
[color-scheme meta]: https://drafts.csswg.org/css-color-adjust-1/#color-scheme-meta
[solarized]: https://ethanschoonover.com/solarized/
[Jekyll]: https://jekyllrb.com/
[musl]: https://www.musl-libc.org/
[Askama]: https://github.com/djc/askama
[syntect]: https://github.com/trishume/syntect
[bat]: https://github.com/sharkdp/bat
[include_bytes!]: https://doc.rust-lang.org/std/macro.include_bytes.html`