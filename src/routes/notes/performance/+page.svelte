<script>
	import A from '$lib/components/A.svelte';
	import H2 from '$lib/components/H2.svelte';
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
		>During my studies, I put an emphasis on high-performance computing. My interest in this field
		culimnated in my (only mildly interesting) <A
			href="https://ieeexplore.ieee.org/abstract/document/7967093">bachelor thesis</A
		>. What's left of my passing passion is <em class="line-through">the need for speed</em> a distain
		for slow software.
	</P>
	<P
		>Due to this conditioning, I always find myself curious about performance characteristics of my
		own applications. I wouldn't want to just deploy something for the world to see without knowing
		how it would behave under load. While the web applications and backend systems I tend to spend
		my time with nowadays are a far cry from the number-crunching algorithms screaming along on a <A
			href="https://www.cscs.ch/">super computer</A> from back in the day, they are still worthwile to
		worry about.
	</P>
	<P
		>While writing / building the <A href="/notes/lines">A study of lines</A> note, I noticed that SvelteKit
		does not perform as well as I would like. In particular, load testing the site yielded a capacity
		of about 40 requests per second. While that's a load that will never organically occur here, it's
		a point of pride to make it faster. There is no way such a simple site should be served at such a
		mediocre rate.
	</P>
	<P>What follows is a a highly unscientific and subjectiv exploration.</P>
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
		E.g. instead of doing full page loads when clicking on a link to a subpage, only the strictly
		necessary code and data are loaded.</P>
	<P
		>Finally, SSG is similar to SSR with the difference that the pages are not rendered on demand
		for every request, but instead up front during the build step. This is useful if the page is
		static anyway and doesn't change depending on the request.</P>
	<P>Choosing the right one</P>
	<H2>Benchmarking marending.dev</H2>
	<P>wrk benchmarks</P>
	<P>Slow SSG / Caddy reverse_proxy vs. file_server</P>
</Note>
