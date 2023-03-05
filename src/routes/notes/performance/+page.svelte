<script>
	import A from '$lib/components/A.svelte';
	import C from '$lib/components/C.svelte';
	import Code from '$lib/components/Code.svelte';
	import H2 from '$lib/components/H2.svelte';
	import H3 from '$lib/components/H3.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
	import SSR from './ssr-table.svelte';
	import SSG from './ssg-table.svelte';
	import Caddy from './caddy-table.svelte';
	import CaddyStatic from './caddy-static-table.svelte';
	import { bash } from 'svelte-highlight/languages';
	import Details from '$lib/components/Details.svelte';
	import Info from '$lib/components/Info.svelte';
</script>

<Note>
	<Title subtitle="Making sure my website can handle all those page views it isn't getting"
		>Performance tuning a specific SvelteKit application</Title>
	<P
		>During my studies, I focused on high-performance computing. My interest in this field
		culminated in my (only mildly interesting) <A
			href="https://ieeexplore.ieee.org/abstract/document/7967093">bachelor thesis</A
		>. What's left of my passing passion is <em class="line-through">the need for speed</em> a distain
		for slow software.
	</P>
	<P
		>Due to this conditioning, I always find myself curious about performance characteristics of my
		own applications. I wouldn't want to just deploy something for the world to see without knowing
		how it would behave under load. While the web applications and backend systems I tend to spend
		my time with nowadays are a far cry from the number-crunching code screaming along on a <A
			href="https://www.cscs.ch/">super computer</A> from back in the day, they are still worthwhile
		to worry about.
	</P>
	<P
		>While writing / building the <A href="/notes/lines">A study of lines</A> note, I noticed that SvelteKit
		does not perform as well as I would like. In particular, load testing the site yielded a capacity
		for about 40 requests per second on my small VPS. While that's a load that will never organically
		occur here, it's a point of pride to make it faster. There is no reason such a simple site should
		be served at such a mediocre rate.
	</P>
	<P
		>What follows is a a highly unscientific and subjective exploration into improving the
		performance of this website.</P>
	<H2 id="background">Background on SvelteKits rendering modes (CSR, SSR, SSG)</H2>
	<P
		>This site is my first project using SvelteKit, so my knowledge is limited. However, from
		glancing at the docs and some quick experimentation we can learn that there are three rendering
		modes supported out of the box: Client-side rendering (CSR), server-side rendering (SSR) and
		static site generation (SSG). SvelteKit is very flexible and allows combinations of CSR and SSR
		as well as page specific configuration.
	</P>
	<P
		>CSR refers to the browser rendering the page. This is the mode employed by any single-page
		application framework on its own. In this mode, the backend would send back a document that just
		contains the script to render the page, but no markup.</P>
	<P
		>SSR is the counter part: The server pre-renders the page for every request. The browser just
		needs to show the document, which already contains all the markup. What's somewhat unintuitive
		is that SvelteKit employs these two techniques at the same time. Pages are pre-rendered on the
		server, but the code to render the page on the client is still shipped along. In a process
		called <em>hydration</em> the browser renders the page again. This is supposed to give us benefits
		from both options. Fast initial page load as well as full js-driven interactivity.</P>
	<P
		>Additionally, if CSR is enabled SvelteKit serves code that takes over navigation in the client.
		That means instead of doing full page loads when clicking a link to a subpage, only the strictly
		necessary code and data are loaded.</P>
	<P
		>Finally, SSG is similar to SSR with the difference that the pages are not rendered on demand
		for every request, but instead up front during the build step. This is useful if the page is
		static anyway and doesn't change depending on the request.</P>
	<H3>Current config for marending.dev</H3>
	<P
		>While this is primarily a static site, there is one piece of functionality that makes use of
		the dynamic server-side rendering. The metrics section counts page views by writing to a SQLite
		database on every request. If we pre-render the pages at build-time and serving them from a
		webserver, there is no code running server-side to keep track of views.
	</P>
	<Info>Update: The metrics section is gone for now :(</Info>
	<P
		>However, in the process of thinking about this I had an idea to be able to statically
		pre-render all pages while still collecting statistics: Instead of building the collection into
		the node server serving the site, I can configure the reverse proxy (in my case Caddy) to emit
		logs. Then, in a classic display of over-engineering, I consume said logs in a separate service,
		which offers aggregated statistics to the site via API call. So for the purposes of improving
		performance of the website, I can ignore the need for dynamic behaviour on the server side for
		now.
	</P>
	<H2 id="benchmark">Benchmarking marending.dev</H2>
	<P
		>To get a realistic end-to-end view of how the server performs, I'm running benchmarks at the <C
			>HTTP</C> level using <C>wrk</C> approximately like so:
	</P>
	<Code
		language={bash}
		value={`wrk -c 100 -d 30s -t 6 -H "no-track: true" --latency http://localhost:3000/notes/lines`} />
	<P
		>The purpose of the <C>no-track</C> header is to make sure the server doesn't record the page views
		for the <A href="/metrics/views">metrics</A> section.
	</P>
	<P
		>For faster turnaround when benchmarking I'm running the preview server of SvelteKit locally. So
		any numbers presented here are not to be compared with performance numbers from the production
		deployment on a much weaker VPS. They just serve to compare with other approaches on the same
		machine.
	</P>
	<P
		>Before any optimizations, I'm running SvelteKit in the default configuration with the node
		adapter. That means CSR as well as SSR are employed, while no pages are pre-rendered at
		build-time.
	</P>
	<SSR />
	<P>
		For the <C>/notes/lines</C> page there is a noticable degredation because there are thousands of
		elements rendered into <C>SVGs</C>, which has the server do some work. The whole page is also
		significantly larger, which is also going to have some impact.
	</P>
	<P
		>Next, we are going to turn on pre-rendering (or SSG) while keeping everything else the same.</P>
	<SSG />
	<P
		>The impact is immediately noticable for both pages. Now the difference between <C>/</C> and <C
			>/notes/lines</C> is probably down to the size of the pages as no additional effort is required
		per request for either.</P>
	<P
		>Obviously SSG is going to be faster than SSR, nothing unexpected here. However, that's not the
		end of it. In the production deployment, the site sits behind <A href="https://caddyserver.com/"
			>Caddy</A
		>, which acts as a reverse proxy to the SvelteKit node server. Before we go further, we should
		establish if we incur a performance penality for this.
	</P>
	<Caddy />
	<P
		>Indeed we do, and it's suprising to me how much throughput we lose. But this scenario begs the
		question if we even need the node server: If we just have static assets, we might as well let
		Caddy serve them. We would expect that to be faster at the very least because we don't need to
		proxy the requests anymore, but also, Caddy being written in Go is likely to be faster at
		serving requests than node.</P>
	<Hint
		>When switching Caddy from reverse proxying requests to directly serving the static assets, make
		sure to set the <C>trailingSlash</C>
		<A href="https://github.com/sveltejs/kit/tree/master/packages/adapter-static">appropriately</A>.
		Otherwise reloading a subpage doesn't work. Don't ask me how I know.
	</Hint>
	<CaddyStatic />
	<Details title="Validating measurements is a pretty good idea!">
		<P>
			At first I was actually measuring around 62'000 requests per second for the <C
				>/notes/lines</C> page. This was surprising since it's a larger page than <C>/</C>, so if
			anything, it should be slower. My first thought was that I mistyped the URL and was hitting
			the <C>404</C> page, but visiting it in the browser worked just fine.
		</P>
		<P
			>Only when <C>curl</C>ing the page from the command line did I notice that I was getting a
			redirect response due to the trailing slash case mentioned above: <C>/notes/lines</C> is redirected
			to <C>/notes/lines/</C>. Load testing that URL directly yields the above numbers.
		</P>
	</Details>
	<P
		>Much better! The switch from dynamically rendering pages on demand to pre-rendering them was
		actually a good thing. I had to come up with another way to collect statistics, which turned out
		to be more reusable. Hop on over to <A href="/notes/website/">this note</A> to learn more.</P>
	<P>
		Throughout this note, we have looked at locally hosted servers and proxies as representative of
		the real deployment. I'll leave you with the throughput of serving <C>/notes/lines/</C> as measured
		against my tiny VPS: <b>791 requests per second</b> versus the 40 that triggered this whole excursion.
		This is still much lower than the numbers we got from local testing. That's probably explained by
		TLS handshakes, network latency and bandwidth and other shenanigans. For now it'll do.
	</P>
</Note>
