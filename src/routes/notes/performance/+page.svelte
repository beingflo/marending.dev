<script>
	import A from '$lib/components/A.svelte';
	import C from '$lib/components/C.svelte';
	import Code from '$lib/components/Code.svelte';
	import H2 from '$lib/components/H2.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Info from '$lib/components/Info.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
</script>

<Note>
	<Title subtitle="Making sure my website can handle all those page views it isn't getting"
		>Performance tuning a specific SvelteKit application</Title>
	<Info>Work in progress!</Info>
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
			href="https://www.cscs.ch/">super computer</A> from back in the day, they are still worthwile to
		worry about.
	</P>
	<P
		>While writing / building the <A href="/notes/lines">A study of lines</A> note, I noticed that SvelteKit
		does not perform as well as I would like. In particular, load testing the site yielded a capacity
		for about 40 requests per second on my small VPS. While that's a load that will never organically
		occur here, it's a point of pride to make it faster. There is no reason such a simple site should
		be served at such a mediocre rate.
	</P>
	<P
		>What follows is a a highly unscientific and subjectiv exploration into improving the
		performance of this website.</P>
	<H2>Background on SvelteKits rendering modes (CSR, SSR, SSG)</H2>
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
		>Additionaly, if CSR is enabled SvelteKit serves code that takes over navigation in the client.
		That means instead of doing full page loads when clicking a link to a subpage, only the strictly
		necessary code and data are loaded.</P>
	<P
		>Finally, SSG is similar to SSR with the difference that the pages are not rendered on demand
		for every request, but instead up front during the build step. This is useful if the page is
		static anyway and doesn't change depending on the request.</P>
	<H2>Benchmarking marending.dev</H2>
	<P
		>Before any optimizations, I'm running SvelteKit in the default configuration with the
		node-adapter. That means CSR as well as SSR are employed, while no pages are prerendered at
		build-time. In the case of <A href="/notes/lines">A study of lines</A> the server-side rendering
		for every request is particularly noticeable because there are thousands of elements rendered into
		<C>SVGs</C>, which apparently takes its toll.
	</P>
	<P
		>To have faster turnaround when benchmarking I'm running the preview server of SvelteKit
		locally. So any numbers presented here are not to be compared with performance numbers from the
		production deployment on a much weaker VPS. They just serve to compare with other approaches on
		the same machine.
	</P>
	<P
		>To get a realistic end-to-end view of how the server performs, I'm running benchmarks at the <C
			>HTTP</C> level using <C>wrk</C>.
	</P>
	<Code
		value={`wrk -c 100 -d 30s -t 6 -H "no-track: true" --latency http://localhost:3000/notes/lines`} />
	<P
		>The purpose of the <C>no-track</C> header is to make sure the server doesn't record the page views
		for the <A href="/metrics/views">metrics</A> section.
	</P>
	<P>Slow SSG / Caddy reverse_proxy vs. file_server</P>
</Note>
